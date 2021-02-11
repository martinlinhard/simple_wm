use crate::key_handler::*;
impl KeyBinding {
    pub fn get_mask(&self) -> u32 {
        self.modifiers
            .iter()
            .fold(0, |acc, curr| *curr as u32 | acc)
    }
}
