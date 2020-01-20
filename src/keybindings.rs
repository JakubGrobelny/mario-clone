extern crate serde_json;
use serde::{Deserialize, Serialize};

extern crate sdl2;
use sdl2::keyboard::Keycode;

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings<A> {
    pub bindings: [A; KEYS_COUNT],
}

#[repr(u8)]
pub enum Key {
    Jump = 0,
    Crouch,
    Left,
    Right,
    Escape,
    Sprint,
}

pub const KEYS_COUNT: usize = Key::Sprint as usize + 1;

pub const DEFAULT_KEYS: [Keycode; KEYS_COUNT] = [
    Keycode::Space,
    Keycode::LCtrl,
    Keycode::Left,
    Keycode::Right,
    Keycode::Escape,
    Keycode::LShift,
];

pub fn default_key_bindings() -> KeyBindings<i32> {
    let mut default_keys_i32 = [0; KEYS_COUNT];
    for i in 0..KEYS_COUNT {
        default_keys_i32[i] = DEFAULT_KEYS[i] as i32;
    }

    KeyBindings {
        bindings: default_keys_i32,
    }
}

pub fn convert_to_keycodes(bindings: KeyBindings<i32>) -> KeyBindings<Keycode> {
    let mut keycodes = [Keycode::Escape; KEYS_COUNT];
    for i in 0..KEYS_COUNT {
        keycodes[i] =
            Keycode::from_i32(bindings.bindings[i]).unwrap_or(DEFAULT_KEYS[i])
    }
    KeyBindings { bindings: keycodes }
}
