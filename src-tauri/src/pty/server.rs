extern crate env_logger;
extern crate futures;
extern crate tokio;
use bytes::BytesMut;
use futures::SinkExt;
use futures::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use log::{debug, error};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::Message;

#[cfg(target_os = "linux")]
use super::lib_linux::{PtyCommand, PtyMaster};
#[cfg(target_os = "windows")]
use super::lib_windows::{PtyCommand, PtyMaster};


use mt_logger::*;
use std::collections::HashSet;


#[tokio::main]
pub async fn main() {
    env_logger::init();
    let _ = ws_server()
        .await
        .map_err(|e| debug!("ws server exit with error: {:?}", e));
}

#[derive(Deserialize, Debug)]
struct WindowSize {
    cols: u16,
    rows: u16,
}

async fn handle_websocket_incoming(
    mut incoming: SplitStream<WebSocketStream<TcpStream>>,
    mut pty_shell_writer: PtyMaster,
    websocket_sender: UnboundedSender<Message>,
    stop_sender: UnboundedSender<()>,
) -> Result<(), anyhow::Error> {
    while let Some(Ok(msg)) = incoming.next().await {
        match msg {
            Message::Binary(data) => match data[0] {
                0 => {
                    if data.len().gt(&0) {
                        pty_shell_writer.write_all(&data[1..]).await?;
                    }
                }
                1 => {
                    let resize_msg: WindowSize = serde_json::from_slice(&data[1..])?;
                    pty_shell_writer.resize(resize_msg.cols, resize_msg.rows)?;
                }
                2 => {
                    websocket_sender.send(Message::Binary(vec![1u8]))?;
                }
                3 => {
                    mt_log!(Level::Info, "LOADING ENV VARS");
                    let mut env_vars = HashMap::new();
                    for (key, value) in std::env::vars() {
                        env_vars.insert(key, value);
                    }

                    std::thread::sleep(std::time::Duration::from_secs(1));

                    let mut load_env_var_script = String::from("export ");

                    for (key, value) in env_vars.iter() {
                        load_env_var_script.push_str(&format!("{}=\"{}\" ", key, value));
                    }
                    
                    let prompt_commnd = r#"PROMPT_COMMAND='echo -en "\033]0; [manter] {\"cwd\": \"$(pwd)\"} \a"' "#;
                    load_env_var_script.push_str(prompt_commnd);

                    let term_var = "TERM=xterm-256color ";
                    load_env_var_script.push_str(term_var);

                    load_env_var_script.push_str("\n");
                    pty_shell_writer.write_all(load_env_var_script.as_bytes()).await?;

                    std::thread::sleep(std::time::Duration::from_secs(1));

                    pty_shell_writer.write_all("source ~/.bashrc \n".as_bytes()).await?;
                }
                _ => (),
            },
            Message::Ping(data) => websocket_sender.send(Message::Pong(data))?,
            _ => (),
        };
    }
    let _ = stop_sender
        .send(())
        .map_err(|e| debug!("failed to send stop signal: {:?}", e));
    Ok(())
}

async fn handle_pty_incoming(
    mut pty_shell_reader: PtyMaster,
    websocket_sender: UnboundedSender<Message>,
) -> Result<(), anyhow::Error> {
    let fut = async move {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.resize(1024, 0u8);
        loop {
            buffer[0] = 0u8;
            let mut tail = &mut buffer[1..];
            let n = pty_shell_reader.read_buf(&mut tail).await?;
            if n == 0 {
                break;
            }
            match websocket_sender.send(Message::Binary(buffer[..n + 1].to_vec())) {
                Ok(_) => (),
                Err(e) => anyhow::bail!("failed to send msg to client: {:?}", e),
            }
        }
        Ok::<(), anyhow::Error>(())
    };
    fut.await.map_err(|e| {
        log::error!("handle pty incoming error: {:?}", &e);
        e
    })
}

async fn write_to_websocket(
    mut outgoing: SplitSink<WebSocketStream<TcpStream>, Message>,
    mut receiver: UnboundedReceiver<Message>,
) -> Result<(), anyhow::Error> {
    while let Some(msg) = receiver.recv().await {
        outgoing.send(msg).await?;
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream) -> Result<(), anyhow::Error> {
    let ws_stream = accept_async(stream).await?;
    let (ws_outgoing, ws_incoming) = ws_stream.split();
    let (sender, receiver) = unbounded_channel();
    let ws_sender = sender.clone();

    let mut cmd = Command::new("su");
    let envs: HashMap<String, String> = HashMap::new();

    cmd.envs(&envs).args(&["-", "ion"]);
    let mut pty_cmd = PtyCommand::from(cmd);
    let (stop_sender, stop_receiver) = unbounded_channel();
    let pty_master = pty_cmd.run(stop_receiver).await?;

    let pty_shell_writer = pty_master.clone();
    let pty_shell_reader = pty_master.clone();

    

    let res = tokio::select! {
        res = handle_websocket_incoming(ws_incoming, pty_shell_writer, sender, stop_sender) => res,
        res = handle_pty_incoming(pty_shell_reader, ws_sender) => res,
        res = write_to_websocket(ws_outgoing, receiver) => res,
    };
    log::debug!("res = {:?}", res);
    Ok(())
}

pub async fn ws_server() -> Result<(), anyhow::Error> {
    let addr: SocketAddr = "127.0.0.1:7703".parse().unwrap();
    match TcpListener::bind(addr).await {
        Ok(listener) => {
            while let Ok((stream, peer)) = listener.accept().await {
                log::debug!("handling request from {:?}", peer);
                let fut = async move {
                    let _ = handle_connection(stream)
                        .await
                        .map_err(|e| error!("handle connection error: {:?}", e));
                };
                tokio::spawn(fut);
            }
        }
        Err(e) => return Err(anyhow::anyhow!("failed to listen: {:?}", e)),
    }
    Ok(())
}
