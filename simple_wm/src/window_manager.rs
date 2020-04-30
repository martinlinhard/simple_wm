use crate::clients::client::Client;
use crate::clients::client::BORDER_WIDTH;
use crate::data::{Position, Size};
use crate::window_system::WindowSystem;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::thread::spawn;
use x11::xlib;
use x11::xlib::Display;
use x11::xlib::Window;

use std::collections::HashMap;

pub struct WindowManager {
    pub window_system: WindowSystem,
    /// The key is the actual window id, the value is the client (which has the same window field)
    pub clients: HashMap<Window, Client>,

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
        loop {
            let event = unsafe {
                let mut event: xlib::XEvent = MaybeUninit::uninit().assume_init();
                xlib::XNextEvent(self.window_system.display, &mut event);
                event
            };
            let event_type = event.get_type();
            match event_type as i32 {
                xlib::ConfigureRequest => {
                    let conf_event = xlib::XConfigureRequestEvent::from(event);

                    match self.clients.get(&conf_event.window) {
                        Some(client) => {
                            /*//Client already exists --> configure him
                            client.configure(
                                &self.window_system,
                                conf_event,
                                0,
                                0,
                                self.window_system.width,
                                self.window_system.height,
                            );*/
                        }
                        None => {
                            //Client doesn't exist yet --> create it!
                            let client: Client = Client::new(
                                &self.window_system,
                                conf_event,
                                0,
                                0,
                                self.window_system.width,
                                self.window_system.height,
                            );
                            self.clients.insert(conf_event.window, client);
                        }
                    };
                }
                xlib::MapRequest => {
                    let map_event = xlib::XMapRequestEvent::from(event);
                    //Client is already known --> map him!
                    if let Some(client) = self.clients.get_mut(&map_event.window) {
                        client.map(&mut self.window_system);
                    }
                }
                /*ButtonPress => {
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
                }*/
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
}
