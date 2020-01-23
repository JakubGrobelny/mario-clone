use sdl2::event::Event;
use sdl2::event::EventPollIterator;
use sdl2::keyboard::Keycode;

enum KeyEventType {
    Down,
    Up,
}

#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
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

#[derive(Debug)]
pub struct Controller {
    mouse: Mouse,
    keys: [ButtonState; KEY_NUM],
}

#[repr(usize)]
pub enum Key {
    Up = 0,
    Down,
    Left,
    Right,
    Escape,
    Sprint,

    Invalid,
}

const KEY_NUM: usize = Key::Invalid as usize;

impl From<Keycode> for Key {
    fn from(code: Keycode) -> Self {
        match code {
            Keycode::Up => Key::Up,
            Keycode::Down => Key::Down,
            Keycode::Left => Key::Left,
            Keycode::Right => Key::Right,
            Keycode::Escape => Key::Escape,
            Keycode::LShift => Key::Sprint,
            _ => Key::Invalid,
        }
    }
}

impl ButtonState {
    fn update_with_event(&mut self, event: KeyEventType) {
        use ButtonState::*;
        use KeyEventType::*;

        match (&self, event) {
            (Active, Up) => *self = Inactive,
            (Inactive, Down) => *self = Pressed,
            (Pressed, Up) => *self = Inactive,
            _ => (),
        }
    }

    fn update_pressed(&mut self) {
        if let ButtonState::Pressed = &self {
            *self = ButtonState::Active
        }
    }
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            mouse: Mouse::new(),
            keys: [ButtonState::Inactive; KEY_NUM],
        }
    }

    pub fn update(&mut self, events: Vec<Event>) {
        for key in self.keys.iter_mut() {
            key.update_pressed();
        }

        for event in events.iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    let index = Key::from(*code) as usize;
                    self.keys[index].update_with_event(KeyEventType::Down);
                }
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => {
                    let index = Key::from(*code) as usize;
                    self.keys[index].update_with_event(KeyEventType::Up);
                }
                _ => (),
            }
        }

    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys[key as usize] != ButtonState::Inactive
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
