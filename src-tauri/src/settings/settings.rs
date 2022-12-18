use std::{path::Path, fs::{self, OpenOptions}, io::Write};

use mt_logger::{mt_log, Level};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Settings {
  pub default_login_user: String
}

#[tauri::command]
pub fn get_settings() -> String {
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


pub fn check_settings_file() {
  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();
  let settings_file = format!("{}/.manter.json", home_dir);
  let settings_file_path = Path::new(&settings_file);

  if settings_file_path.exists() {
    return;
  }

  mt_log!(Level::Info, "Create new settings file {:?}", settings_file_path);

  let settings_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(settings_file_path);

  let default_user = whoami::username();
  let settings_json_object = serde_json::to_string(&Settings {
    default_login_user: default_user
  }).unwrap();
  
  settings_file.unwrap().write_all(settings_json_object.as_bytes()).unwrap();
}