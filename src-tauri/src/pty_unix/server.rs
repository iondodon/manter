#![cfg(any(target_os = "linux", target_os = "macos"))]


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
use std::io::Read;
use std::net::SocketAddr;
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::Message;
use futures::future::Future as _;
use futures::stream::Stream as _;
use std::io::Write as _;
use futures::future::Future as _;
use futures::stream::Stream as _;
use std::io::Write as _;
use tokio_pty_process_stream::*;
use std::io;
use std::process::exit;

use termion::raw::IntoRawMode;

use tokio::prelude::*;
use tokio::task;
use tokio_process_pty::Command;


use super::lib::{PtyCommand, PtyMaster};

use mt_logger::*;

mod input;



#[tokio::main]
pub async fn main() {
    env_logger::init();
    let _ = ws_server()
        .await
        .map_err(|e| mt_log!(Level::Info, "ws server exit with error: {:?}", e));
}

async fn handle_connection(_stream: TcpStream) -> Result<(), anyhow::Error> {

    Ok(())
}

pub async fn ws_server() -> Result<(), anyhow::Error> {
    let addr: SocketAddr = "127.0.0.1:7703".parse().unwrap();
    match TcpListener::bind(addr).await {
        Ok(listener) => {
            while let Ok((stream, peer)) = listener.accept().await {
                mt_log!(Level::Info, "handling request from {:?}", peer);
                let fut = async move {
                    let _ = handle_connection(stream)
                        .await
                        .map_err(|e| mt_log!(Level::Info, "handle connection error: {:?}", e));
                };
                tokio::spawn(fut);
            }
        }
        Err(e) => return Err(anyhow::anyhow!("failed to listen: {:?}", e)),
    }
    Ok(())
}
