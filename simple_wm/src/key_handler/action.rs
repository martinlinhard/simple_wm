use crate::key_handler::*;
use crate::layout::Layout;
use crate::tag::Tag;
use crate::window_system::WindowSystem;
use std::process::*;
use std::thread::spawn;
use x11::xlib;
use x11::xlib::Window;

impl Action {
    pub fn execute(
        &mut self,
        system: &WindowSystem,
        shift_by: usize,
        current: Option<Window>,
        layout: &mut dyn Layout,
        tag: &mut Tag,
    ) {
        match self {
            // builtin action --> delegate!
            Action::Builtin(builtin) => {
                match builtin {
                    BuiltinCommand::Close => unsafe {
                        // if it is *some*, we know that it itsn't the root window
                        // --> kill window
                        if let Some(current) = current {
                            xlib::XKillClient(system.display, current);
                        }
                    },
                    BuiltinCommand::MoveLeft => layout.shift_left(tag, shift_by, system, current),
                    BuiltinCommand::MoveRight => layout.shift_right(tag, shift_by, system, current),
                }
            }
            // custom command --> execute it!
            Action::Custom(items) => {
                let mut iter = items.iter();
                if let Some(cmd) = iter.next() {
                    let mut command = Command::new(cmd);
                    command.args(iter);
                    spawn(move || {
                        let _ = command.spawn();
                    });
                }
            }
        }
    }
}
