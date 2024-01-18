// #![windows_subsystem = "windows"]
use std::thread::spawn;
mod activity;
mod input_watcher;
mod storage;
mod tray;

fn main() {
  storage::storage::create_table();
  spawn(|| tray::tray::run());
  let windows_handle = spawn(|| input_watcher::input_watcher::run());
  windows_handle.join().unwrap();
}
