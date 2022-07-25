#![cfg(target_os = "windows")]

use std::io::{Write, Read};
use std::net::TcpStream;
use std::thread;
use bytes::BytesMut;
use conpty::io::PipeReader;
use websocket::sync::{Server, Writer};
use websocket::OwnedMessage;
use mt_logger::*;
use portable_pty::{CommandBuilder, PtySize, native_pty_system, PtySystem};
use anyhow::Error;

fn listen_pty(mut reader: Box<dyn Read + Send>, mut sender: Writer<TcpStream>) {
    let mut buffer = BytesMut::with_capacity(1024);
    buffer.resize(1024, 0u8);
    loop {
        buffer[0] = 0u8;
        let mut tail = &mut buffer[1..];
        let n = reader.read(&mut tail).unwrap();
        if n == 0 {
            break;
        }
        sender.send_message(&OwnedMessage::Text(String::from_utf8_lossy(&buffer[..n + 1]).to_string())).unwrap();
    }
}


pub fn main() {
	let server = Server::bind("127.0.0.1:7703").unwrap();

	for request in server.filter_map(Result::ok) {
		thread::spawn(|| {
			let client = request.accept().unwrap();

			let ip = client.peer_addr().unwrap();

			mt_log!(Level::Info, "Connection from {}", ip);

			let (mut receiver, sender) = client.split().unwrap();

            // Use the native pty implementation for the system
            let pty_system = native_pty_system();

            // Create a new pty
            let mut pair = pty_system.openpty(PtySize {
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

            // Spawn a shell into the pty
            let cmd = CommandBuilder::new("powershell");
            let child = pair.slave.spawn_command(cmd).unwrap();

            // Read and parse output from the pty with reader
            let mut reader = pair.master.try_clone_reader().unwrap();
            let mut writer = pair.master.try_clone_writer().unwrap();

            thread::spawn(|| {
                listen_pty(reader, sender);
            });

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
                    OwnedMessage::Binary(msg) => {
                        let msg_bytes = msg.as_slice();
                        writer.write_all(&msg_bytes[1..]).unwrap();
                    },
                    _ => todo!()
				}
			}
		});
	}
}