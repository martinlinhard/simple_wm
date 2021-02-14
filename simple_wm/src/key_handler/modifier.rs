use crate::key_handler::*;

use x11::xlib;
impl KeyCombination {
    pub fn get_mask(&self) -> u32 {
        self.modifiers
            .iter()
            .fold(0, |acc, curr| *curr as u32 | acc)
    }
}

impl Modifier {
    pub fn from_event(evt: &xlib::XKeyEvent) -> Vec<Self> {
        let mut modifiers = vec![];
        if evt.state & Modifier::Alt as u32 != 0 {
            modifiers.push(Modifier::Alt);
        }
        if evt.state & Modifier::Shift as u32 != 0 {
            modifiers.push(Modifier::Shift);
        }
        if evt.state & Modifier::Super as u32 != 0 {
            modifiers.push(Modifier::Super);
        }
        modifiers
    }
}
