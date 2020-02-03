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
    Active(u8),
    Inactive,
}

#[derive(Debug)]
pub struct Mouse {
    pos:     (i32, i32),
    buttons: [ButtonState; 3],
    scroll:  i32,
}

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum MButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug)]
pub struct Controller {
    mouse: Mouse,
    keys:  [ButtonState; KEY_NUM],
}

#[derive(Copy, Clone)]
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
    Ctrl,

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
            Keycode::LCtrl => Key::Ctrl,
            Keycode::Space => Key::Up,
            Keycode::A => Key::Left,
            Keycode::D => Key::Right,
            Keycode::W => Key::Up,
            Keycode::S => Key::Down,
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
        match (*self, event) {
            (_, KeyEventType::Up) => *self = ButtonState::Inactive,
            (ButtonState::Inactive, KeyEventType::Down) => {
                *self = ButtonState::Active(0)
            },
            (ButtonState::Active(time), KeyEventType::Down) => {
                *self = ButtonState::Active(time)
            },
        }
    }

    fn update_time(&mut self) {
        if let ButtonState::Active(time) = *self {
            if time != std::u8::MAX {
                *self = ButtonState::Active(time + 1)
            }
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
            key.update_time();
        }

        for button in self.mouse.buttons.iter_mut() {
            button.update_time();
        }

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

    fn is_key_active_timed(&self, key: Key, time: u8) -> bool {
        match self.keys[key as usize] {
            ButtonState::Active(t) if t >= time => true,
            _ => false,
        }
    }

    pub fn is_key_active_time_limited(&self, key: Key, time: u8) -> bool {
        match self.keys[key as usize] {
            ButtonState::Active(t) if t <= time => true,
            _ => false,
        }
    }

    pub fn is_key_active_delayed(&self, key: Key, time: u8) -> bool {
        self.was_key_pressed(key) || self.is_key_active_timed(key, time)
    }

    pub fn is_key_active(&self, key: Key) -> bool {
        self.keys[key as usize] != ButtonState::Inactive
    }

    pub fn was_key_pressed(&self, key: Key) -> bool {
        self.keys[key as usize] == ButtonState::Active(0)
    }

    pub fn is_button_active_delayed(&self, button: MButton, time: u8) -> bool {
        self.mouse.is_button_active_delayed(button, time)
    }

    pub fn is_button_active(&self, button: MButton) -> bool {
        self.mouse.is_button_active(button)
    }

    pub fn was_button_pressed(&self, button: MButton) -> bool {
        self.mouse.was_button_pressed(button)
    }

    pub fn x_acceleration(&self) -> f64 {
        if self.is_key_active(Key::Left) {
            -1.0
        } else if self.is_key_active(Key::Right) {
            1.0
        } else {
            0.0
        }
    }

    pub fn clear_mouse(&mut self) {
        for button in self.mouse.buttons.iter_mut() {
            *button = ButtonState::Inactive;
        }
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
            pos:     (0, 0),
            buttons: [ButtonState::Inactive; 3],
            scroll:  0,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn scroll(&self) -> i32 {
        self.scroll
    }

    pub fn is_button_active_delayed(&self, button: MButton, time: u8) -> bool {
        self.was_button_pressed(button)
            || self.is_button_active_timed(button, time)
    }

    pub fn is_button_active(&self, button: MButton) -> bool {
        self.buttons[button as usize] != ButtonState::Inactive
    }

    pub fn is_button_active_timed(&self, button: MButton, time: u8) -> bool {
        match self.buttons[button as usize] {
            ButtonState::Active(t) if t >= time => true,
            _ => false,
        }
    }

    pub fn was_button_pressed(&self, button: MButton) -> bool {
        self.buttons[button as usize] == ButtonState::Active(0)
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
                self.buttons[MButton::Left as usize].update_with_event(event);
            },
            MouseButton::Right => {
                self.buttons[MButton::Right as usize].update_with_event(event);
            },
            MouseButton::Middle => {
                self.buttons[MButton::Middle as usize].update_with_event(event);
            },
            _ => (),
        }
    }
}
