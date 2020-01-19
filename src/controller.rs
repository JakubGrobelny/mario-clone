use crate::keybindings::*;
use sdl2::{event::Event, EventPump};


pub struct Movement([ButtonState; 4]);

pub struct Mouse {
    pos: (i32, i32),
    state: (ButtonState, ButtonState)
}

pub struct Controller {
    movement: Movement,
    mouse: Mouse,
    pause: ButtonState,
    sprint: ButtonState,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            movement: Movement::new(),
            mouse: Mouse::new(),
            pause: ButtonState::Inactive,
            sprint: ButtonState::Inactive,
        }
    }

}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pos: (0, 0),
                state: (ButtonState::Inactive, ButtonState::Inactive)
        }
    }
}

impl Movement {
    pub fn new() -> Movement {
        Movement([ButtonState::Inactive; 4])
    }

    fn is_moving_in_direction(&self, dir: usize) -> bool {
        match self.0[dir] {
            ButtonState::Inactive => false,
            _ => true
        }
    }

    pub fn is_moving_up(&self) -> bool {
        self.is_moving_in_direction(0)
    }

    pub fn is_moving_down(&self) -> bool {
        self.is_moving_in_direction(1)
    }

    pub fn is_moving_left(&self) -> bool {
        self.is_moving_in_direction(2)
    }

    pub fn is_moving_right(&self) -> bool {
        self.is_moving_in_direction(3)
    }
}
