use std::{path::Path, fs::OpenOptions, io::{BufReader, BufWriter, Write, Read}};

pub fn configure() {
  if cfg!(target_os = "macos") {
    update_rc_file(".zshrc.rs")
  }
}

fn update_rc_file(rc_file_name: &str) {
  let home_dir = dirs::home_dir().unwrap();
  let home_dir = home_dir.to_str().unwrap();

  let rc_file = format!("{}/{}", home_dir, rc_file_name);

  let rc_file_path = Path::new(&rc_file);

  if !rc_file_path.exists() {
    panic!("rc file not found");
  }
  
  write_if_not_present_in_file(&rc_file, "Hi").unwrap();
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