extern crate serde_json;
use serde::{Deserialize, Serialize};

extern crate sdl2;
use sdl2::keyboard::Keycode;

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings {
    jump: i32,
    crouch: i32,
    left: i32,
    right: i32,
    escape: i32,
    sprint: i32,
}

pub fn default_keybindings() -> KeyBindings {
    KeyBindings {
        jump: Keycode::Space as i32,
        crouch: Keycode::LCtrl as i32,
        left: Keycode::Left as i32,
        right: Keycode::Right as i32,
        escape: Keycode::Escape as i32,
        sprint: Keycode::LShift as i32,
    }
}
