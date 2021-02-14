pub mod stack_layout;

use crate::config::Config;
use crate::tag::Tag;
use crate::window_system::WindowSystem;
use x11::xlib::Window;

pub trait Layout {
    fn resize(&mut self, tag: &mut Tag, config: &Config, system: &WindowSystem);
    fn shift_left(
        &mut self,
        tag: &mut Tag,
        shift_by: usize,
        system: &WindowSystem,
        current: Option<Window>,
    );
    fn shift_right(
        &mut self,
        tag: &mut Tag,
        shift_by: usize,
        system: &WindowSystem,
        current: Option<Window>,
    );
}
