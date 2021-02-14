pub mod stack_layout;

use crate::client::Client;
use crate::config::Config;
use crate::window_system::WindowSystem;
use std::sync::Arc;

pub trait Layout {
    fn resize(
        &mut self,
        clients: &[Arc<Client>],
        config: &Config,
        system: &WindowSystem,
    );
}
