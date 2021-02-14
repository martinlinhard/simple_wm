use crate::window_system::WindowSystem;
use x11::xlib;

/// In simple_wm, a client basically represents the entire window a client sees,
/// that is: The actual window + the frame (as we're reparenting) to draw a border
#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct Client {
    /// This is the actual window which stems from the client
    pub window: xlib::Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Client {
    pub fn new(conf_event: xlib::XConfigureRequestEvent) -> Self {
        Self {
            window: conf_event.window,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn set_bounds(
        &mut self,
        window_system: &WindowSystem,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;

        self.move_window(window_system);
    }

    pub fn move_horizontal(&mut self, amount: i32, move_x: bool, window_system: &WindowSystem) {
        self.width += amount;
        if move_x {
            self.x += amount;
        }
        self.move_window(window_system);
    }

    fn move_window(&self, window_system: &WindowSystem) {
        unsafe {
            xlib::XMoveResizeWindow(
                window_system.display,
                self.window,
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
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
