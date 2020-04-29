use std::ptr;
use x11::xlib::{Display, Window};
use x11::xlib::{XDefaultScreenOfDisplay, XOpenDisplay, XRootWindowOfScreen};
use std::os::raw::c_int;

pub struct WindowSystem {
    pub display: *mut Display,
    pub root: Window,
    pub width: c_int,
    pub height: c_int,
}

impl WindowSystem {
    pub fn new() -> WindowSystem {
        unsafe {
            let display = XOpenDisplay(ptr::null_mut());
            let screen = XDefaultScreenOfDisplay(display);
            let root = XRootWindowOfScreen(screen);

            WindowSystem {
                display: display,
                root: root,
                width: (*screen).width,
                height: (*screen).height,
            }
        }
    }
}

impl Drop for WindowSystem {
    fn drop(&mut self) {
        unsafe {
            x11::xlib::XCloseDisplay(self.display);
        }
    }
}
