use std::collections::HashMap;
use std::io::{Write, Read};
use std::net::TcpStream;
use std::thread;
use bytes::BytesMut;
use serde::Deserialize;
use websocket::sync::{Server, Writer};
use websocket::OwnedMessage;
use mt_logger::*;
use portable_pty::{CommandBuilder, PtySize, native_pty_system};

const WS_ADDRESS: &str = "127.0.0.1:7703";

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

fn feed_client(mut pty_reader: Box<dyn Read + Send>, mut ws_sender: Writer<TcpStream>) {
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
        ws_sender.send_message(&OwnedMessage::Binary(data_to_send)).unwrap();
    }
}


pub fn pty_server() {
	let server = Server::bind(WS_ADDRESS).unwrap();

	for request in server.filter_map(Result::ok) {
		thread::spawn(|| {
			let client = request.accept().unwrap();

			let ip = client.peer_addr().unwrap();

			mt_log!(Level::Info, "Connection from {}", ip);

			let (mut ws_receiver, ws_sender) = client.split().unwrap();

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


            // if os is windows let handler_cmd be powershell else if unix let handler_cmd be su
            let handler_cmd = if cfg!(target_os = "windows") { "powershell" } else { "su" };
            let mut cmd = CommandBuilder::new(handler_cmd);
            #[cfg(unix)]
            cmd.args(["-", "ion"]);

            let _child = pty_pair.slave.spawn_command(cmd).unwrap();

            let pty_reader = pty_pair.master.try_clone_reader().unwrap();
            let mut pty_writer = pty_pair.master.try_clone_writer().unwrap();

            thread::spawn(|| {
                feed_client(pty_reader, ws_sender);
            });

			for message in ws_receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
                    OwnedMessage::Binary(msg) => {
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
                                pty_writer.write_all(load_env_var_script.as_bytes()).unwrap();

                                std::thread::sleep(std::time::Duration::from_secs(1));

                                #[cfg(target_os = "macos")]
                                pty_writer.write_all(r#" prmptcmd() { eval "$PROMPT_COMMAND" } "#.as_bytes()).unwrap();
                                #[cfg(target_os = "macos")]
                                pty_writer.write_all("\n".as_bytes()).unwrap();
                                #[cfg(target_os = "macos")]
                                pty_writer.write_all(r#" precmd_functions=(prmptcmd) "#.as_bytes()).unwrap();
                                #[cfg(target_os = "macos")]
                                pty_writer.write_all("\n".as_bytes()).unwrap();

                                std::thread::sleep(std::time::Duration::from_secs(1));

                                #[cfg(target_os = "linux")]
                                pty_writer.write_all("source ~/.bashrc \n".as_bytes()).unwrap();

                                #[cfg(target_os = "macos")]
                                pty_writer.write_all("source ~/.profile \n".as_bytes()).unwrap();

                                std::thread::sleep(std::time::Duration::from_secs(1));

                                #[cfg(target_os = "macos")]
                                pty_writer.write_all("source ~/.zshenv \n".as_bytes()).unwrap;
                            }
                            _ => mt_log!(Level::Error, "Unknown command {}", msg_bytes[0]),
                        }
                    },
                    _ => mt_log!(Level::Error, "Unknown received data type")
				}
			}
		});
	}
}