use crate::window_system::WindowSystem;
use x11::xlib;

pub const BORDER_WIDTH: u32 = 3;
pub const BORDER_COLOR: u64 = 0xff0000;
pub const BG_COLOR: u64 = 0x0000ff;

/// In simple_wm, a client basically represents the entire window a client sees,
/// that is: The actual window + the frame (as we're reparenting) to draw a border
#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Client {
    /// This is the actual window which stems from the client
    pub window: xlib::Window,
}

impl Client {
    pub fn new(
        window_system: &WindowSystem,
        conf_event: xlib::XConfigureRequestEvent,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Self {
        let mut changes = xlib::XWindowChanges {
            x,
            y,
            width,
            height,
            border_width: BORDER_WIDTH as i32,
            sibling: conf_event.above,
            stack_mode: conf_event.detail,
        };

        unsafe {
            xlib::XConfigureWindow(
                window_system.display,
                conf_event.window,
                conf_event.value_mask as u32,
                &mut changes,
            );
        }

        Self {
            window: conf_event.window,
        }
    }

    pub fn configure(
        &self,
        window_system: &WindowSystem,
        conf_event: xlib::XConfigureRequestEvent,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let mut changes = xlib::XWindowChanges {
            x,
            y,
            width,
            height,
            border_width: BORDER_WIDTH as i32,
            sibling: conf_event.above,
            stack_mode: conf_event.detail,
        };

        unsafe {
            xlib::XConfigureWindow(
                window_system.display,
                self.window,
                conf_event.value_mask as u32,
                &mut changes,
            );
        }
    }

    pub fn map(&mut self, window_system: &mut WindowSystem) {
        unsafe {
            xlib::XMapWindow(window_system.display, self.window);
        }
    }
}
