#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate serde_json;

use mt_logger::*;
use pty::ws_server::pty_serve;
use settings::settings::{check_settings_file, get_settings};
use config::config::configure;

mod pty;
mod settings;
mod config;


#[tokio::main]
async fn main() {
  mt_new!(None, Level::Info, OutputStream::Both);

  configure();
  check_settings_file();

  std::thread::spawn(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { pty_serve().await });
  });

  let context = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_settings])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
