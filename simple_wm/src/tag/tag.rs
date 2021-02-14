use crate::client::Client;
use crate::tag::Tag;
use crate::window_system::WindowSystem;
use x11::xlib;
use x11::xlib::Window;

impl Tag {
    pub fn new() -> Self {
        Self { windows: vec![] }
    }

    pub fn add_new_window_if_not_exists(&mut self, client: Client) {
        if !self.window_contained(&client.window) {
            self.windows.push(client);
        }
    }

    fn window_contained(&self, window: &Window) -> bool {
        self.windows.iter().any(|current| current.window == *window)
    }

    pub fn get_windows(&self) -> &[Client] {
        &self.windows[..]
    }

    pub fn get_windows_mut(&mut self) -> &mut [Client] {
        &mut self.windows[..]
    }

    pub fn map_window(&self, window: &Window, system: &WindowSystem) {
        // map the windows with the same id
        self.windows
            .iter()
            .filter(|current| current.window == *window)
            .for_each(|client| {
                client.map(system);
                self.set_focus(&client.window, system);
            });
    }

    pub fn set_focus(&self, window: &Window, system: &WindowSystem) {
        unsafe {
            xlib::XSetInputFocus(
                system.display,
                *window,
                xlib::RevertToParent,
                xlib::CurrentTime,
            );
        }
    }

    pub fn for_root_and_remainder<R, RS, S>(
        &mut self,
        mut root: R,
        mut root_with_sub: RS,
        mut remainder: S,
    ) where
        R: FnMut(&mut Client),
        RS: FnMut(&mut Client, usize),
        S: FnMut(&mut Client, usize),
    {
        let len = self.windows.len();
        let mut iter = self.windows.iter_mut().rev();
        match (iter.next(), iter.next()) {
            // no window contained
            (None, None) => (),
            // literally impossible
            (None, Some(_)) => (),
            (Some(client), None) => {
                root(client);
            }
            (Some(first_client), Some(second_client)) => {
                // set first client
                root_with_sub(first_client, len);

                remainder(second_client, len);

                // for all the remaining windows on the right side
                for client in iter {
                    remainder(client, len);
                }
            }
        }
    }

    pub fn remove_window(&mut self, window: &Window) {
        // delete all the matching windows
        self.windows.retain(|current| current.window != *window);
    }
}
