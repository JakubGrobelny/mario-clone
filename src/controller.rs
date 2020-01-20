use crate::keybindings::*;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

enum KeyEventType {
    Down,
    Up,
}

#[derive(Debug, Copy, Clone)]
pub enum ButtonState {
    Active,
    Inactive,
    Pressed,
}

#[derive(Debug)]
pub struct Mouse {
    pos: (i32, i32),
    state: (ButtonState, ButtonState),
}

pub struct Controller {
    mouse: Mouse,
    keys: [ButtonState; KEYS_COUNT],
}

impl ButtonState {
    fn update(&mut self, event: KeyEventType) {
        use ButtonState::*;
        match (&self, event) {
            (Active, Up) => *self = Inactive,
            (Inactive, Down) => *self = Pressed,
            (Pressed, Down) => *self = Active,
            (Pressed, Up) => *self = Inactive,
            _ => ()
        }
    }
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            mouse: Mouse::new(),
            keys: [ButtonState::Inactive; KEYS_COUNT],
        }
    }

    fn update_key(
        &mut self,
        event: KeyEventType,
        code: Keycode,
        bindings: &KeyBindings<Keycode>,
    ) {
        for i in 0..KEYS_COUNT {
            if code == bindings.bindings[i] {
                self.keys[i].update(event);
                return;
            }
        }
    }

    pub fn update(&mut self, event: &Event, bindings: &KeyBindings<Keycode>) {
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                self.update_key(KeyEventType::Down, *key, bindings);
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                self.update_key(KeyEventType::Up, *key, bindings);
            }
            _ => (),
        }
    }
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pos: (0, 0),
            state: (ButtonState::Inactive, ButtonState::Inactive),
        }
    }
}
