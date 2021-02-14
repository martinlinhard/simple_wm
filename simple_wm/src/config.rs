use crate::key_handler::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The gaps in px
    pub gaps: usize,
    pub key_bindings: HashMap<KeyCombination, Action>,
}

impl Config {
    pub fn new() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert(
            KeyCombination {
                modifiers: vec![Modifier::Alt, Modifier::Shift],
                key: Key::XK_q,
            },
            Action::Builtin(BuiltinCommand::Close),
        );
        key_bindings.insert(
            KeyCombination {
                modifiers: vec![Modifier::Alt, Modifier::Shift],
                key: Key::XK_f,
            },
            Action::Custom(vec!["dolphin".to_owned()]),
        );

        let new = Self {
            gaps: 8,
            key_bindings,
        };
        new
    }
}
