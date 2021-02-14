use crate::config::Config;
use crate::layout::Layout;
use crate::tag::Tag;
use crate::window_system::WindowSystem;
use x11::xlib::Window;

pub struct StackLayout {}

impl StackLayout {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for StackLayout {
    fn resize(&mut self, tag: &mut Tag, config: &Config, system: &WindowSystem) {
        let mut offset_y: i32 = config.gaps as i32;
        let window_width = (system.width / 2) - (config.gaps / 2) as i32 - (config.gaps as i32);
        let left_window_height: i32 = system.height - (config.gaps * 2) as i32;
        let offset_x = window_width + 2 * config.gaps as i32;

        tag.for_root_and_remainder(
            // only one client
            |client| {
                client.set_bounds(
                    system,
                    config.gaps as i32,
                    config.gaps as i32,
                    system.width - (config.gaps * 2) as i32,
                    system.height - (config.gaps * 2) as i32,
                );
            },
            // more than one client; left
            |client, _| {
                // set first client
                client.set_bounds(
                    system,
                    config.gaps as i32,
                    config.gaps as i32,
                    window_width,
                    left_window_height,
                );
            },
            // more than one client; right
            |client, len| {
                let right_window_height: i32 =
                    (system.height - (len * config.gaps) as i32) / (len - 1) as i32;

                // set the second client
                client.set_bounds(
                    system,
                    offset_x,
                    offset_y as i32,
                    window_width,
                    right_window_height,
                );
                // add the height
                offset_y += right_window_height as i32;
                // add the gaps
                offset_y += config.gaps as i32;
            },
        );
    }
    fn shift_left(
        &mut self,
        tag: &mut Tag,
        shift_by: usize,
        system: &WindowSystem,
        current: Option<Window>,
    ) {
        println!("shifting left");
    }
    fn shift_right(
        &mut self,
        tag: &mut Tag,
        shift_by: usize,
        system: &WindowSystem,
        current: Option<Window>,
    ) {
        println!("shifting right");
    }
}
