#![cfg(target_os = "windows")]

use mt_logger::*;

use message_io::node::{self};
use message_io::network::{NetEvent, Transport};
use serde::Deserialize;
use std::ffi::OsString;
use winptyrs::{PTY, PTYArgs, MouseMode, AgentConfig};
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Debug)]
struct WindowSize {
    cols: i32,
    rows: i32,
}


pub fn main() {
    let pty_args = PTYArgs {
        cols: 80,
        rows: 25,
        mouse_mode: MouseMode::WINPTY_MOUSE_MODE_NONE,
        timeout: 10000,
        agent_config: AgentConfig::WINPTY_FLAG_COLOR_ESCAPES
    };

    // Initialize a pseudoterminal.
    let pty = PTY::new(&pty_args).unwrap();
    let pty = Arc::new(Mutex::new(pty));


    // Create a node, the main message-io entity. It is divided in 2 parts:
    // The 'handler', used to make actions (connect, send messages, signals, stop the node...)
    // The 'listener', used to read events from the network or signals.
    let (handler, listener) = node::split::<()>();

    // Listen for TCP, UDP and WebSocket messages at the same time.
    handler.network().listen(Transport::Ws, "0.0.0.0:7703").unwrap();
    let handler = Arc::new(handler);

    // Read incoming network events.
    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => unreachable!(),
        NetEvent::Accepted(endpoint, _listener) => {
            let cmd = OsString::from("c:\\windows\\system32\\cmd.exe").to_owned();

            // Spawn a process inside the pseudoterminal.
            pty.lock().unwrap().spawn(cmd, None, None, None).unwrap();

            let pty_clone = Arc::clone(&pty);
            let handler_clone = Arc::clone(&handler);
            thread::spawn(move || {
                loop {
                    let output_os_string = pty_clone.lock().unwrap().read(1023, false).unwrap();
                    let output_str = output_os_string.into_string().unwrap();
                    let output_str = format!("{}{}", 0u8, output_str);
                    let output_bytes = output_str.as_bytes();

                    handler_clone.network().send(endpoint, output_bytes);
                }
            });

            println!("Client connected")
        },
        NetEvent::Message(_endpoint, data) => {
            match data[0] {
                0 => {
                    let str_data = std::str::from_utf8(&data[1..]).unwrap();
                    let to_write = OsString::from(str_data);
                    let _num_bytes = pty.lock().unwrap().write(to_write).unwrap();
                },
                1 => {
                    let resize_msg: WindowSize = serde_json::from_slice(&data[1..]).unwrap();
                    let _ = pty.lock().unwrap().set_size(resize_msg.cols, resize_msg.rows);
                },
                _ => {
                    mt_log!(Level::Info, "Unknown message: {:?}", data);
                }
            }
        },
        NetEvent::Disconnected(_endpoint) => mt_log!(Level::Info, "Disconnected"),
    });
}