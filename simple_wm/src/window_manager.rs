use crate::client::Client;
use crate::config::Config;
use crate::layout::stack_layout::StackLayout;
use crate::layout::Layout;
use crate::tag::Tag;
use crate::window_system::WindowSystem;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::thread::spawn;
use x11::xlib;
use x11::xlib::Display;
use x11::xlib::Window;

use std::collections::HashMap;

pub const MAX_WORKSPACES: usize = 10;

pub struct WindowManager {
    /// A handle to the underlying X11 system
    pub window_system: WindowSystem,
    /// All the various tags with their respective clients
    pub tags: Vec<Tag>,
    /// The currently visible workspace -> from 0 to 9
    pub current_workspace: usize,
    /// The config for the window manager, can be reloaded
    pub config: Config,
    /// The current layout
    pub current_layout: Box<dyn Layout>,
}

impl WindowManager {
    pub fn new() -> Self {
        let mut tags = Vec::with_capacity(MAX_WORKSPACES);
        for _ in 0..MAX_WORKSPACES {
            tags.push(Tag::new());
        }

        Self {
            window_system: WindowSystem::new(),
            tags,
            current_workspace: 0,
            config: Config::new(),
            current_layout: Box::new(StackLayout::new()),
        }
    }

    fn get_next_event(&self) -> xlib::XEvent {
        let event = unsafe {
            let mut event: xlib::XEvent = MaybeUninit::uninit().assume_init();
            xlib::XNextEvent(self.window_system.display, &mut event);
            event
        };
        event
    }

    pub fn run(&mut self) {
        self.init();
        loop {
            let event = self.get_next_event();
            match event.get_type() as i32 {
                xlib::ConfigureRequest => {
                    // convert to request
                    let conf_event = xlib::XConfigureRequestEvent::from(event);
                    // register keybindings
                    self.register_keybindings(&conf_event.window);
                    // get the current tag & make sure that it contains the new window
                    let current_tag = &mut self.tags[self.current_workspace];
                    current_tag.add_new_window_if_not_exists(Client::new(conf_event));
                    // resize all the windows based on the current layout
                    let windows = current_tag.get_windows();
                    self.current_layout
                        .resize(windows, &self.config, &self.window_system);
                }
                xlib::MapRequest => {
                    let map_event = xlib::XMapRequestEvent::from(event);
                    //Client is already known in the current tag --> map it!
                    let current_tag = &self.tags[self.current_workspace];
                    current_tag.map_window(&map_event.window, &self.window_system);
                }
                xlib::UnmapNotify => {
                    let map_event = xlib::XUnmapEvent::from(event);
                    for tag in self.tags.iter_mut() {
                        tag.remove_window(&map_event.window);
                    }
                    let current_tag = &mut self.tags[self.current_workspace];
                    let windows = current_tag.get_windows();
                    self.current_layout
                        .resize(windows, &self.config, &self.window_system);
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
                }*/
                xlib::KeyPress => {
                    println!("here!");
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

    fn register_keybindings(&self, window: &Window) {
        for binding in self.config.key_bindings.iter() {
            unsafe {
                xlib::XGrabKey(
                    self.window_system.display,
                    xlib::XKeysymToKeycode(
                        self.window_system.display,
                        binding.key as std::os::raw::c_ulong,
                    )
                    .into(),
                    binding.get_mask(),
                    *window,
                    0,
                    xlib::GrabModeAsync,
                    xlib::GrabModeAsync,
                );
            }
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
