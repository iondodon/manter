#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod pty;

use pty::ws_server::pty_serve;
use std::thread;
use mt_logger::*;

fn main() {
  mt_new!(None, Level::Info, OutputStream::Both);
  thread::spawn(|| {
    pty_serve();
  });

  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
