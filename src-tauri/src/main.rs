#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::thread;
use mt_logger::*;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod pty_linux;
#[cfg(target_os = "windows")]
mod pty_windows;

fn main() {
  mt_new!(None, Level::Info, OutputStream::Both);
  thread::spawn(|| {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pty_linux::server::main();
    #[cfg(target_os = "windows")]
    pty_windows::server::main();
  });

  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
