#![allow(non_upper_case_globals)]
mod client;
mod config;
mod key_handler;
mod layout;
mod models;
mod tag;
mod window_manager;
mod window_system;

use window_manager::WindowManager;

fn main() {
    let mut wm = WindowManager::new();
    wm.run();
}
