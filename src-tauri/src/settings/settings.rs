use std::{path::Path, fs::{self, OpenOptions}, io::Write};

use mt_logger::{mt_log, Level};
use serde_json::{Value, Map};


#[tauri::command]
pub fn get_settings() -> Map<String, Value> {
  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();
  let settings_file = format!("{}/.manter.json", home_dir);
  let settings_file_path = Path::new(&settings_file);

  if !settings_file_path.exists() {
    panic!("Settings file not found");
  }

  let settings_file_string = fs::read_to_string(settings_file_path).unwrap();

  let settings: Map<String, Value> = serde_json::from_str(&settings_file_string).unwrap();

  settings
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
  let mut settings_json_object = Map::new();
  settings_json_object.insert("default_login_user".to_string(), Value::String(default_user));

  let user_scripts = if cfg!(target_os = "linux") {
    r#"{ "cwd": "$(pwd)", "git": { "currentBranch" : "$(git rev-parse --abbrev-ref HEAD 2> /dev/null )" } }"#
  } else if cfg!(target_os = "macos") {
    r#"{ "cwd": "$(pwd)" }"#
  } else {
    r#"{}"#
  };

  settings_json_object.insert("user_scripts".to_string(), serde_json::from_str(user_scripts).unwrap());
  
  let settings_json_object = serde_json::to_string(&settings_json_object).unwrap();
  
  settings_file.unwrap().write_all(settings_json_object.as_bytes()).unwrap();
}
