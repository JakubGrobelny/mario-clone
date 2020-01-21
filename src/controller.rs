use sdl2::event::Event;
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
    up: ButtonState,
    down: ButtonState,
    left: ButtonState,
    right: ButtonState,
    escape: ButtonState,
    sprint: ButtonState,
}

pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Sprint,
}

const UP_KEY: Keycode = Keycode::Up;
const DOWN_KEY: Keycode = Keycode::Down;
const LEFT_KEY: Keycode = Keycode::Left;
const RIGHT_KEY: Keycode = Keycode::Right;
const ESCAPE_KEY: Keycode = Keycode::Escape;
const SPRINT_KEY: Keycode = Keycode::LShift;

impl ButtonState {
    fn update(&mut self, event: KeyEventType) {
        use ButtonState::*;
        use KeyEventType::*;
        match (&self, event) {
            (Active, Up) => *self = Inactive,
            (Inactive, Down) => *self = Pressed,
            (Pressed, Down) => *self = Active,
            (Pressed, Up) => *self = Inactive,
            _ => (),
        }
    }
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            mouse: Mouse::new(),
            up: ButtonState::Inactive,
            down: ButtonState::Inactive,
            left: ButtonState::Inactive,
            right: ButtonState::Inactive,
            escape: ButtonState::Inactive,
            sprint: ButtonState::Inactive,
        }
    }

    fn update_key(&mut self, event: KeyEventType, code: Keycode) {
        match code {
            UP_KEY => self.up.update(event),
            DOWN_KEY => self.down.update(event),
            LEFT_KEY => self.left.update(event),
            RIGHT_KEY => self.right.update(event),
            ESCAPE_KEY => self.escape.update(event),
            SPRINT_KEY => self.sprint.update(event),
            _ => (),
        }
    }

    pub fn update(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                self.update_key(KeyEventType::Down, *key);
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                self.update_key(KeyEventType::Up, *key);
            }
            _ => (),
        }
    }

    pub fn active(&self, key: Key) -> bool {
        match key {
            Key::Down => self.down != ButtonState::Inactive,
            Key::Right => self.right != ButtonState::Inactive,
            Key::Up => self.up != ButtonState::Inactive,
            Key::Left => self.left != ButtonState::Inactive,
            Key::Sprint => self.sprint != ButtonState::Inactive,
            Key::Escape => self.escape != ButtonState::Inactive,
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
