use crate::key_handler::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The gaps in px
    pub gaps: usize,
    pub key_bindings: Vec<KeyBinding>,
}

impl Config {
    pub fn new() -> Self {
        let mut first = HashSet::new();
        first.insert(Modifier::Super);
        first.insert(Modifier::Shift);
        Self {
            gaps: 8,
            key_bindings: vec![KeyBinding {
                modifiers: first,
                key: Key::XK_a,
                action: Action::Builtin(BuiltinCommand::Close),
            }],
        }
    }
}
