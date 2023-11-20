#![windows_subsystem = "windows"]
use std::thread::spawn;
use storage::create_table;

fn main() {
  create_table();
  spawn(|| tray::run());
  let windows_handle = spawn(|| input_watcher::run());
  windows_handle.join().unwrap();
}
