#![cfg(target_os = "windows")]

extern crate env_logger;

use mt_logger::*;

pub fn main() {
    env_logger::init();
    mt_log!(Level::Info, "Hello, world!");
}