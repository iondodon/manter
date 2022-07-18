#![cfg(any(target_os = "linux", target_os = "macos"))]

pub mod server;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod lib;