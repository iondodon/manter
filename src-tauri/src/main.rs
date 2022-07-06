#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::thread;
use log::{debug, error};
use mt_logger::*;

mod pty;

fn main() {
  thread::spawn(|| {
    pty::server::main();
  });

  // debug!("YOKLMN");


  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
