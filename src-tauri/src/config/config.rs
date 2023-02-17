use std::{path::Path, fs::OpenOptions, io::{BufReader, BufWriter, Write, Read}, env};

use mt_logger::{mt_log, Level};

const CONFIG_SCRIPTS_ZSH: &str = r#"
prmptcmd() { eval "$PROMPT_COMMAND" }
precmd_functions+=(prmptcmd)
trap "echo The shell session has terminated." EXIT
"#;

const CONFIG_SCRIPTS_BASH: &str = r#"
trap "echo The shell session has terminated." EXIT
"#;

pub fn configure() {
  if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
    match env::var("SHELL") {
      Ok(shell) => update_rc_file(&shell),
      Err(e) => panic!("Failed to get the shell: {}", e),
    }
  }
}

fn update_rc_file(shell: &str) {
  let (rc_file_name, config_scripts) = match shell {
    "/bin/zsh" => (".zshrc", CONFIG_SCRIPTS_ZSH),
    "/bin/bash" => (".bashrc", CONFIG_SCRIPTS_BASH),
    _ => panic!("Shell not supported"),
  };

  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();

  let rc_file = format!("{}/{}", home_dir, rc_file_name);

  let rc_file_path = Path::new(&rc_file);

  if !rc_file_path.exists() {
    panic!("rc file not found");
  }

  create_config_scripts_file(config_scripts);
  let config_scripts_file = format!("{}/.manter.sh", home_dir);
  let script = format!("\nsource {}\n", config_scripts_file);
  write_if_not_present_in_file(&rc_file, &script);
}

fn create_config_scripts_file(config_scripts: &str) {
  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();
  let config_scripts_file = format!("{}/.manter.sh", home_dir);
  let config_scripts_file_path = Path::new(&config_scripts_file);

  if config_scripts_file_path.exists() {
    return;
  }

  mt_log!(Level::Info, "Create new config scripts file {:?}", config_scripts_file_path);

  let config_scripts_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(config_scripts_file_path);

  match config_scripts_file {
    Ok(mut file) => {
      file.write_all(config_scripts.as_bytes()).unwrap();
      file.flush().unwrap();
    }
    Err(e) => panic!("Failed to create config scripts file: {}", e),
  }
}

fn write_if_not_present_in_file(file_path: &str, text: &str) {
  let file = OpenOptions::new().read(true).open(file_path).unwrap();
  let mut reader = BufReader::new(file);

  let mut contents = String::new();
  reader.read_to_string(&mut contents).unwrap();

  if contents.contains(text) {
    return;
  }

  let file = OpenOptions::new().append(true).open(file_path).unwrap();
  let mut writer = BufWriter::new(file);

  writer.write_all(text.as_bytes()).unwrap();
  writer.flush().unwrap();

}