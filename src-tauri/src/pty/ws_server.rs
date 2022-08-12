use std::io::{Write, Read};
use futures::{StreamExt, SinkExt};
use futures::stream::{SplitSink, SplitStream};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use bytes::BytesMut;
use serde::Deserialize;
use mt_logger::*;
use portable_pty::{CommandBuilder, PtySize, native_pty_system, PtyPair};
use tokio_tungstenite::{accept_async, WebSocketStream};

const PTY_SERVER_ADDRESS: &str = "127.0.0.1:7703";


#[derive(Deserialize, Debug)]
struct WindowSize {
  /// The number of lines of text
  pub rows: u16,
  /// The number of columns of text
  pub cols: u16,
  /// The width of a cell in pixels.  Note that some systems never
  /// fill this value and ignore it.
  pub pixel_width: u16,
  /// The height of a cell in pixels.  Note that some systems never
  /// fill this value and ignore it.
  pub pixel_height: u16,
}

#[derive(Deserialize, Debug)]
struct LoginData {
  pub password: String
}

async fn feed_client_from_pty(
  mut pty_reader: Box<dyn Read + Send>, 
  mut ws_sender: SplitSink<WebSocketStream<TcpStream>, Message>
) {
  let mut buffer = BytesMut::with_capacity(1024);
  buffer.resize(1024, 0u8);
  loop {
    buffer[0] = 0u8;
    let mut tail = &mut buffer[1..];
    let n = pty_reader.read(&mut tail).unwrap();
    if n == 0 {
        break;
    }
    let mut data_to_send = Vec::with_capacity(n + 1);
    data_to_send.extend_from_slice(&buffer[..n + 1]);
    let message  = Message::Binary(data_to_send);
    ws_sender.send(message).await.unwrap();
  }
}


async fn feed_pty_from_ws(
  mut ws_receiver: SplitStream<WebSocketStream<TcpStream>>, 
  mut pty_writer: Box<dyn Write + Send>, pty_pair: PtyPair
) {
  loop {
    let message = ws_receiver.next().await.unwrap().unwrap();

    match message {
      Message::Binary(msg) => {
          let msg_bytes = msg.as_slice();
          match msg_bytes[0] {
            0 => {
              if msg_bytes.len().gt(&0) {
                pty_writer.write_all(&msg_bytes[1..]).unwrap();
              }
            }
            1 => {
              let resize_msg: WindowSize = serde_json::from_slice(&msg_bytes[1..]).unwrap();
              let pty_size = PtySize {
                rows: resize_msg.rows,
                cols: resize_msg.cols,
                pixel_width: resize_msg.pixel_width,
                pixel_height: resize_msg.pixel_height,
              };
              pty_pair.master.resize(pty_size).unwrap();
            }
            2 => {
              mt_log!(Level::Info, "Login & Environment initialization...");

              let login_data: LoginData = serde_json::from_slice(&msg_bytes[1..]).unwrap();

              pty_writer.write_all(login_data.password.as_bytes()).unwrap();
              pty_writer.write_all("\n".as_bytes()).unwrap();

              let mut load_env_var_script = String::from(" export ");

              let prompt_commnd = r#"PROMPT_COMMAND='echo -en "\033]0; [manter] {\"cwd\": \"$(pwd)\"} \a"' "#;
              load_env_var_script.push_str(prompt_commnd);

              let term_var = "TERM=xterm-256color ";
              load_env_var_script.push_str(term_var);

              load_env_var_script.push_str("\n");
              pty_writer.write_all(load_env_var_script.as_bytes()).unwrap();

              #[cfg(target_os = "linux")]
              pty_writer.write_all(" source ~/.bashrc \n".as_bytes()).unwrap();

              #[cfg(target_os = "macos")]
              pty_writer.write_all(r#" prmptcmd() { eval "$PROMPT_COMMAND" } "#.as_bytes()).unwrap();
              #[cfg(target_os = "macos")]
              pty_writer.write_all("\n".as_bytes()).unwrap();
              #[cfg(target_os = "macos")]
              pty_writer.write_all(r#" precmd_functions=(prmptcmd) "#.as_bytes()).unwrap();
              #[cfg(target_os = "macos")]
              pty_writer.write_all("\n".as_bytes()).unwrap();
              #[cfg(target_os = "macos")]
              pty_writer.write_all(" source ~/.profile \n".as_bytes()).unwrap();
              #[cfg(target_os = "macos")]
              pty_writer.write_all(" source ~/.zshenv \n".as_bytes()).unwrap();

              mt_log!(Level::Info, "Login & Environment initialization finished");
            }
            _ => mt_log!(Level::Error, "Unknown command {}", msg_bytes[0]),
          }
      },
      _ => mt_log!(Level::Error, "Unknown received data type")
    }
  }
}


async fn accept_connection(stream: TcpStream) {
  let ws_stream = accept_async(stream).await.expect("Failed to accept");
  let (ws_sender, ws_receiver) = ws_stream.split();

  let pty_system = native_pty_system();
  let pty_pair = pty_system.openpty(PtySize {
    rows: 24,
    cols: 80,
    // Not all systems support pixel_width, pixel_height,
    // but it is good practice to set it to something
    // that matches the size of the selected font.  That
    // is more complex than can be shown here in this
    // brief example though!
    pixel_width: 0,
    pixel_height: 0,
  }).unwrap();


  let cmd = if cfg!(target_os = "windows") { 
    CommandBuilder::new("powershell")
  } else {
    let user = crate::get_setting("default_login_user");
    let mut cmd = CommandBuilder::new("su");  
    cmd.args(["-m", user.as_str()]);
    cmd
  };

  let _child = pty_pair.slave.spawn_command(cmd).unwrap();

  let pty_reader = pty_pair.master.try_clone_reader().unwrap();
  let pty_writer = pty_pair.master.try_clone_writer().unwrap();

  std::thread::spawn(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      feed_client_from_pty(pty_reader, ws_sender).await;
    })
  });

  feed_pty_from_ws(ws_receiver, pty_writer, pty_pair).await;
}


pub async fn pty_serve() {
  let listener = TcpListener::bind(PTY_SERVER_ADDRESS).await.expect("Can't listen");

  while let Ok((stream, _)) = listener.accept().await {
    let peer = stream.peer_addr().expect("connected streams should have a peer address");
    mt_log!(Level::Info, "Peer address: {}", peer);

    std::thread::spawn(|| {
      let rt = tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async {
        accept_connection(stream).await;
      });
    });
  }
}