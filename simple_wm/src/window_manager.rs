use crate::data::{Position, Size};
use crate::event_codes::*;
use crate::event_handler::EventHandler;
use crate::main_window::WindowSystem;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use x11::xlib;
use x11::xlib::Display;
use std::thread::spawn;
use x11::xlib::Window;

use std::collections::HashMap;

pub struct WindowManager {
    pub window_system: WindowSystem,
    pub clients: HashMap<Window, Window>,

    /// The cursor position at the start of a window move / resize
    pub drag_start_position: Position,

    /// The position of the affected window of a window move / resize (or rather frame, since we are
    /// reparenting! )
    pub drag_start_frame_position: Position,

    /// The size of the affected window at the start of a window resize
    pub drag_start_frame_size: Size,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            window_system: WindowSystem::new(),
            clients: HashMap::new(),
            drag_start_position: Default::default(),
            drag_start_frame_position: Default::default(),
            drag_start_frame_size: Default::default(),
        }
    }

    pub fn run(&mut self) {
        self.init();
                spawn(move || {
                    std::process::Command::new("alacritty").spawn();
                });
        loop {
            let mut event = unsafe {
                let mut event: xlib::XEvent = MaybeUninit::uninit().assume_init();
                xlib::XNextEvent(self.window_system.display, &mut event);
                event
            };
            let event_type = event.get_type();
            match event_type as usize {
                ConfigureRequest => {
                    EventHandler::on_configure_request(self, event);
                }
                MapRequest => {
                    EventHandler::on_map_request(self, event);
                }
                UnmapNotify => {
                    let mut unmap_notify = xlib::XUnmapEvent::from(event);
                    if self.clients.contains_key(&unmap_notify.window)
                        && unmap_notify.event != self.window_system.root
                    {
                        EventHandler::on_unmap_notify(self, &mut unmap_notify.window);
                    }
                }
                ButtonPress => {
                    EventHandler::on_button_press(self, event);
                }
                MotionNotify => {
                    //skip any pending motion events
                    unsafe {
                        while xlib::XCheckTypedWindowEvent(
                            self.window_system.display,
                            event.motion.window,
                            MotionNotify as i32,
                            &mut event,
                        ) == 1
                        {}
                    }
                    EventHandler::on_motion_notify(self, event);
                }
                KeyPress => {
                    EventHandler::on_key_press(self, event);
                }
                _ => (),
            }
        }
    }

    pub fn init(&mut self) {
        unsafe {
            xlib::XSelectInput(
                self.window_system.display,
                self.window_system.root,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask,
            );
            xlib::XSync(self.window_system.display, 0);
            xlib::XSetErrorHandler(Some(WindowManager::error_handler));
        }
    }

    unsafe extern "C" fn error_handler(
        _display: *mut Display,
        event: *mut xlib::XErrorEvent,
    ) -> c_int {
        println!("{:?}", *event);
        0
    }

    pub fn add_client(&mut self, top_lvl_window: Window, framed_window: Window) {
        self.clients.insert(top_lvl_window, framed_window);
    }
}
