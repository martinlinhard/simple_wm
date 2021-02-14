use crate::client::Client;
use crate::config::Config;
use crate::key_handler::*;
use crate::layout::stack_layout::StackLayout;
use crate::layout::Layout;
use crate::tag::Tag;
use crate::window_system::WindowSystem;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use x11::xlib;
use x11::xlib::Display;
use x11::xlib::Window;

use std::collections::*;

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
    /// The currently focused window
    /// Some --> A window has focus; there is at least 1 window present
    /// None --> The root window has the focus
    pub current_window: Option<Window>,
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
            current_window: None,
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

    fn set_and_focus_current(&mut self, window: &Window) {
        // not the root window
        if *window != self.window_system.root {
            // focus the current window
            unsafe {
                xlib::XSetInputFocus(
                    self.window_system.display,
                    *window,
                    xlib::RevertToPointerRoot,
                    xlib::CurrentTime,
                );
            }
            // set it to be the current window
            self.current_window = Some(*window);
        }
    }

    pub fn run(&mut self) {
        self.init();
        loop {
            let event = self.get_next_event();
            match event.get_type() as i32 {
                xlib::ConfigureRequest => {
                    // convert to request
                    let conf_event = xlib::XConfigureRequestEvent::from(event);
                    // get the current tag & make sure that it contains the new window
                    let current_tag = &mut self.tags[self.current_workspace];

                    // add it to the current tag
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
                    // make sure to focus the newly mapped window
                    self.set_and_focus_current(&map_event.window);
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

                    // we are only cloning an arc here
                    let next_window = windows.iter().rev().next().cloned();

                    // at least  1 window present --> focus it!
                    if let Some(client) = next_window {
                        self.set_and_focus_current(&client.window);
                    }
                    // none present --> focused window is root window!
                    else {
                        self.current_window = None;
                    }
                }
                /*ButtonPress => {
                    EventHandler::on_button_press(self, event);
                }*/
                /*xlib::MotionNotify => {
                    //skip any pending motion events
                    unsafe {
                        while xlib::XCheckTypedWindowEvent(
                            self.window_system.display,
                            event.motion.window,
                            6, // stands for motion notify
                            &mut event,
                        ) == 1
                        {}
                    }
                    let motion_e = xlib::XMotionEvent::from(event);
                }*/
                xlib::KeyPress => {
                    let event: xlib::XKeyEvent = xlib::XKeyEvent::from(event);
                    // get all the keys used within the bindings (todo: cache?)
                    let used_keys: HashSet<&Key> = self
                        .config
                        .key_bindings
                        .keys()
                        .map(|entry| &entry.key)
                        .collect();

                    // try to find a matching key for the event
                    let res: Option<&Key> = used_keys.iter().find_map(|item| unsafe {
                        match event.keycode
                            == xlib::XKeysymToKeycode(
                                self.window_system.display,
                                (**item) as c_ulong,
                            )
                            .into()
                        {
                            true => Some(*item),
                            false => None,
                        }
                    });

                    // key found --> get modifiers from event
                    if let Some(key) = res {
                        let kc = KeyCombination {
                            modifiers: Modifier::from_event(&event),
                            key: *key,
                        };
                        // if the combination is found --> execute its action
                        if let Some(action) = self.config.key_bindings.get_mut(&kc) {
                            action.execute(&self.window_system, self.current_window);
                        }
                    }
                }
                xlib::EnterNotify => {
                    let crossing_evt = xlib::XCrossingEvent::from(event);
                    // can never be the root window!
                    self.current_window = Some(crossing_evt.window);
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
            xlib::XSync(self.window_system.display, xlib::False);
            xlib::XSetErrorHandler(Some(WindowManager::error_handler));
        }
        // register bindings for root window
        self.register_keybindings(&self.window_system.root);
    }

    fn register_keybindings(&self, window: &Window) {
        for binding in self.config.key_bindings.keys() {
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
