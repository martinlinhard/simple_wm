use crate::client::Client;
use crate::config::Config;
use crate::layout::Layout;
use crate::window_system::WindowSystem;
use std::sync::Arc;
use x11::xlib;

pub struct StackLayout {}

impl StackLayout {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for StackLayout {
    fn resize(&mut self, clients: &[Arc<Client>], config: &Config, system: &WindowSystem) {
        let len = clients.len();
        if len == 1 {
            clients[0].set_bounds(
                system,
                config.gaps as i32,
                config.gaps as i32,
                system.width - (config.gaps * 2) as i32,
                system.height - (config.gaps * 2) as i32,
            );
        } else if len > 1 {
            let window_width =
                ((system.width / 2) - (config.gaps / 2) as i32 - (config.gaps as i32));
            let left_window_height: i32 = system.height - (config.gaps * 2) as i32;
            let right_window_height: i32 =
                (system.height - (clients.len() * config.gaps) as i32) / (clients.len() - 1) as i32;

            clients[0].set_bounds(
                system,
                config.gaps as i32,
                config.gaps as i32,
                window_width,
                left_window_height,
            );

            // we know its more than 1 anyways
            let mut clients_iter = clients[1..].iter();
            // start with gaps
            let mut offset_y: i32 = config.gaps as i32;
            let offset_x = window_width + 2 * config.gaps as i32;

            // for all the remaining windows on the right side
            for client in clients_iter {
                client.set_bounds(
                    system,
                    offset_x,
                    offset_y as i32,
                    window_width,
                    right_window_height,
                );
                // add the height
                offset_y += right_window_height as i32;
                // add the gaps
                offset_y += config.gaps as i32;
            }
        }
    }
}
