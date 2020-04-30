#![allow(non_upper_case_globals)]
mod data;
//mod event_codes;
//mod event_handler;
mod window_system;
mod window_manager;

mod clients;
mod handlers;
mod workspaces;

use window_manager::WindowManager;

fn main() {
    let mut wm = WindowManager::new();
    wm.run();
}
