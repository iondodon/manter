use std::{path::Path, fs::OpenOptions, io::{BufReader, BufWriter, Write, Read}, env};

use mt_logger::{mt_log, Level};

const CONFIG_SCRIPTS_MACOS: &str = r#"
prmptcmd() { eval "$PROMPT_COMMAND" }
precmd_functions+=(prmptcmd)
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
  let rc_file_name = match shell {
    "zsh" => ".zshrc",
    "bash" => return,
    _ => panic!("Shell not supported"),
  };

  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();

  let rc_file = format!("{}/{}", home_dir, rc_file_name);

  let rc_file_path = Path::new(&rc_file);

  if !rc_file_path.exists() {
    panic!("rc file not found");
  }

  create_config_scripts_file();
  let config_scripts_file = format!("{}/.manter.sh", home_dir);
  write_if_not_present_in_file(&rc_file, &config_scripts_file).unwrap();
}

fn create_config_scripts_file() {
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
      file.write_all(CONFIG_SCRIPTS_MACOS.as_bytes()).unwrap();
      file.flush().unwrap();
    }
    Err(e) => panic!("Failed to create config scripts file: {}", e),
  }
}

fn write_if_not_present_in_file(file_path: &str, text: &str) -> std::io::Result<()> {
  let file = OpenOptions::new().read(true).open(file_path)?;
  let mut reader = BufReader::new(file);

  let mut contents = String::new();
  reader.read_to_string(&mut contents)?;

  if contents.contains(text) {
    return Ok(());
  }

  let file = OpenOptions::new().append(true).open(file_path)?;
  let mut writer = BufWriter::new(file);

  writer.write_all(text.as_bytes())?;
  writer.flush()?;

  Ok(())
}