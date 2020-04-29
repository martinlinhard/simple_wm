#![allow(non_upper_case_globals)]
mod data;
mod event_codes;
mod event_handler;
mod main_window;
mod window_manager;
use window_manager::WindowManager;

fn main() {
    let mut wm = WindowManager::new();
    wm.run();
}
