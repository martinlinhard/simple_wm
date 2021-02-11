pub mod stack_layout;

use crate::client::Client;
use crate::config::Config;
use crate::window_system::WindowSystem;
use std::sync::Arc;
use x11::xlib;

pub trait Layout {
    fn resize(
        &mut self,
        clients: &[Arc<Client>],
        config: &Config,
        system: &WindowSystem,
    );
}
