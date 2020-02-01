use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use vector2d::Vector2D;

pub enum KeyEventType {
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
    pos:           (i32, i32),
    left_button:   ButtonState,
    right_button:  ButtonState,
    middle_button: ButtonState,
    scroll:        i32,
}

#[derive(Debug)]
pub struct Controller {
    mouse: Mouse,
    keys:  [ButtonState; KEY_NUM],
}

#[repr(usize)]
pub enum Key {
    Up = 0,
    Down,
    Left,
    Right,
    Escape,
    Sprint,
    Enter,
    Tab,

    Invalid,
}

const KEY_NUM: usize = Key::Invalid as usize + 1;

impl From<Keycode> for Key {
    fn from(code: Keycode) -> Self {
        match code {
            Keycode::Up => Key::Up,
            Keycode::Down => Key::Down,
            Keycode::Left => Key::Left,
            Keycode::Right => Key::Right,
            Keycode::Escape => Key::Escape,
            Keycode::KpEnter => Key::Enter,
            Keycode::LShift => Key::Sprint,
            Keycode::Return => Key::Enter,
            Keycode::Tab => Key::Tab,
            _ => Key::Invalid,
        }
    }
}

impl From<&Controller> for Vector2D<i32> {
    fn from(controller: &Controller) -> Vector2D<i32> {
        let x = if controller.is_key_active(Key::Left) {
            -1
        } else if controller.is_key_active(Key::Right) {
            1
        } else {
            0
        };

        let y = if controller.is_key_active(Key::Up) {
            -1
        } else if controller.is_key_active(Key::Down) {
            1
        } else {
            0
        };

        Vector2D::new(x, y)
    }
}

impl From<&Controller> for Vector2D<f64> {
    fn from(controller: &Controller) -> Vector2D<f64> {
        let i_vec = Vector2D::<i32>::from(controller);
        Vector2D::new(f64::from(i_vec.x), f64::from(i_vec.y))
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

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            mouse: Mouse::new(),
            keys:  [ButtonState::Inactive; KEY_NUM],
        }
    }

    pub fn update(&mut self, events: &[Event]) {
        for key in self.keys.iter_mut() {
            key.update_pressed();
        }

        self.mouse.right_button.update_pressed();
        self.mouse.left_button.update_pressed();
        self.mouse.scroll = 0;

        for event in events.iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    let index = Key::from(*code) as usize;
                    self.keys[index].update_with_event(KeyEventType::Down);
                },
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => {
                    let index = Key::from(*code) as usize;
                    self.keys[index].update_with_event(KeyEventType::Up);
                },
                Event::MouseMotion { x, y, .. } => {
                    self.mouse.update_position(*x, *y);
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.mouse.update_button_with_event(
                        *mouse_btn,
                        KeyEventType::Down,
                    );
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.mouse
                        .update_button_with_event(*mouse_btn, KeyEventType::Up);
                },
                Event::MouseWheel { y, .. } => {
                    self.mouse.scroll = *y;
                },
                _ => (),
            }
        }
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn is_key_active(&self, key: Key) -> bool {
        self.keys[key as usize] != ButtonState::Inactive
    }

    pub fn was_key_pressed(&self, key: Key) -> bool {
        self.keys[key as usize] == ButtonState::Pressed
    }

    pub fn clear_mouse(&mut self) {
        self.mouse.right_button = ButtonState::Inactive;
        self.mouse.left_button = ButtonState::Inactive;
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pos:           (0, 0),
            left_button:   ButtonState::Inactive,
            right_button:  ButtonState::Inactive,
            middle_button: ButtonState::Inactive,
            scroll:        0,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn scroll(&self) -> i32 {
        self.scroll
    }

    pub fn was_left_button_pressed(&self) -> bool {
        self.left_button == ButtonState::Pressed
    }

    pub fn was_right_button_pressed(&self) -> bool {
        self.right_button == ButtonState::Pressed
    }

    pub fn was_middle_button_pressed(&self) -> bool {
        self.middle_button == ButtonState::Pressed
    }

    pub fn is_left_button_active(&self) -> bool {
        self.left_button != ButtonState::Inactive
    }

    pub fn is_right_button_active(&self) -> bool {
        self.right_button != ButtonState::Inactive
    }

    pub fn is_middle_button_active(&self) -> bool {
        self.middle_button != ButtonState::Inactive
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.pos = (x, y);
    }

    pub fn update_button_with_event(
        &mut self,
        button: MouseButton,
        event: KeyEventType,
    ) {
        match button {
            MouseButton::Left => {
                self.left_button.update_with_event(event);
            },
            MouseButton::Right => {
                self.right_button.update_with_event(event);
            },
            MouseButton::Middle => {
                self.middle_button.update_with_event(event);
            },
            _ => (),
        }
    }
}
