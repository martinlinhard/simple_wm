use crate::client::Client;
use crate::tag::Tag;
use crate::window_system::WindowSystem;
use std::sync::Arc;
use x11::xlib::Window;

impl Tag {
    pub fn new() -> Self {
        Self { windows: vec![] }
    }

    pub fn add_new_window_if_not_exists(&mut self, client: Client) {
        if !self.window_contained(&client.window) {
            self.windows.push(Arc::new(client));
        }
    }

    fn window_contained(&self, window: &Window) -> bool {
        self.windows.iter().any(|current| current.window == *window)
    }

    pub fn get_windows(&self) -> &[Arc<Client>] {
        &self.windows[..]
    }

    pub fn map_window(&self, window: &Window, system: &WindowSystem) {
        // map the windows with the same id
        self.windows
            .iter()
            .filter(|current| current.window == *window)
            .for_each(|client| {
                client.map(system);
            });
    }

    pub fn remove_window(&mut self, window: &Window) {
        // delete all the matching windows
        self.windows.retain(|current| current.window != *window);
    }
}
