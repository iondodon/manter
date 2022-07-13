#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::thread;
use log::{debug, error};
use mt_logger::*;


/// The error that might happen on a [`fix`] call.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Shell(#[from] std::io::Error),
  #[error("failed to run shell echo: {0}")]
  EchoFailed(String),
}

/// Reads the shell configuration to properly set the PATH environment variable.
#[cfg(not(windows))]
pub fn fix() -> std::result::Result<(), Error> {
  mt_log!(Level::Info, "Entered1");

  let default_shell = if cfg!(target_os = "macos") {
    "/bin/zsh"
  } else {
    "/bin/sh"
  };
  let shell = std::env::var("SHELL").unwrap_or_else(|_| default_shell.into());

  let out = std::process::Command::new(shell)
    .arg("-ilc")
    .arg("echo -n \"_SHELL_ENV_DELIMITER_\"; env; echo -n \"_SHELL_ENV_DELIMITER_\"; exit")
    // Disables Oh My Zsh auto-update thing that can block the process.
    .env("DISABLE_AUTO_UPDATE", "true")
    .output()
    .map_err(Error::Shell)?;

  if out.status.success() {
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let env = stdout.split("_SHELL_ENV_DELIMITER_").nth(1).unwrap();
    for line in String::from_utf8_lossy(&strip_ansi_escapes::strip(env)?)
      .split('\n')
      .filter(|l| !l.is_empty())
    {
      let mut s = line.split('=');
      if let (Some(var), Some(value)) = (s.next(), s.next()) {
        if var == "PATH" {
          std::env::set_var("PATH", value);
          break;
        }
      }
    }
    Ok(())
  } else {
    Err(Error::EchoFailed(
      String::from_utf8_lossy(&out.stderr).into_owned(),
    ))
  }
}

/// Do nothing on Windows as the PATH environment variable is already set.
#[cfg(windows)]
pub fn fix() -> std::result::Result<(), Error> {
  Ok(())
}


mod pty;

fn main() {
  mt_new!(None, Level::Info, OutputStream::Both);
  thread::spawn(|| {
    pty::server::main();
  });

  let _  = fix();

  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
