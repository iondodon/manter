#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod pty;

extern crate serde_json;

use std::{fs::{OpenOptions, self}, io::Write, path::Path};
use pty::ws_server::{pty_serve};
use mt_logger::*;
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Debug, Serialize)]
pub struct Settings {
    pub default_login_user: String
}

#[tauri::command]
fn get_settings() -> String {
  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();
  let settings_file = format!("{}/.manter.json", home_dir);
  let settings_file_path = Path::new(&settings_file);

  if !settings_file_path.exists() {
    panic!("Settings file not found");
  }

  fs::read_to_string(settings_file_path).unwrap()
}


pub fn get_setting(setting_name: &str) -> String {
  let settings_file_string = get_settings();
  let settings: Settings = serde_json::from_str(&settings_file_string).unwrap();

  mt_log!(Level::Info, "Get setting {:?}", setting_name);
  
  match setting_name {
    "default_login_user" => settings.default_login_user,
    _ => panic!("Unknown setting")
  }
}


fn check_settings_file() {
  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();
  let settings_file = format!("{}/.manter.json", home_dir);
  let settings_file_path = Path::new(&settings_file);

  if settings_file_path.exists() {
    return ;
  }
  
  mt_log!(Level::Info, "Create new settings file {:?}", settings_file_path);

  let settings_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(settings_file_path);

  let default_user = whoami::username();
  let settings_json_object = serde_json::to_string(&Settings {
    default_login_user: default_user,
  }).unwrap();
  settings_file.unwrap().write_all(settings_json_object.as_bytes()).unwrap();
}


#[tokio::main]
async fn main() {
  mt_new!(None, Level::Info, OutputStream::Both);

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
