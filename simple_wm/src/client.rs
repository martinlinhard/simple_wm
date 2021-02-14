use crate::window_system::WindowSystem;
use x11::xlib;

/// In simple_wm, a client basically represents the entire window a client sees,
/// that is: The actual window + the frame (as we're reparenting) to draw a border
#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Client {
    /// This is the actual window which stems from the client
    pub window: xlib::Window,
}

impl Client {
    pub fn new(conf_event: xlib::XConfigureRequestEvent) -> Self {
        Self {
            window: conf_event.window,
        }
    }

    pub fn set_bounds(
        &self,
        window_system: &WindowSystem,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        unsafe {
            xlib::XMoveResizeWindow(
                window_system.display,
                self.window,
                x,
                y,
                width as u32,
                height as u32,
            );
        }
    }

    pub fn map(&self, window_system: &WindowSystem) {
        unsafe {
            xlib::XMapWindow(window_system.display, self.window);
        }
    }
    pub fn unmap(&self, window_system: &WindowSystem) {
        unsafe {
            xlib::XUnmapWindow(window_system.display, self.window);
        }
    }
}
