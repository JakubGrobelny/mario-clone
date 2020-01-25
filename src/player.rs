use crate::controller::*;
use crate::hitbox::*;
use crate::physics::*;
use vector2d::Vector2D;

#[derive(Debug)]
pub struct Player {
    position: (i32, i32),
    physics: Physics,
    hitbox: Hitbox,
}

const PLAYER_MASS: f64 = 1.0;
pub const PLAYER_WIDTH: u32 = 50;
pub const PLAYER_HEIGHT: u32 = 100;

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            position: (x, y),
            physics: Physics::new(PLAYER_MASS),
            hitbox: Hitbox::new(x, y, PLAYER_WIDTH, PLAYER_HEIGHT),
        }
    }

    pub fn accelerate(&mut self, controller: &Controller) {
        fn convert_acceleration(controller: &Controller) -> Vector2D<f64> {
            let accel_x = if controller.is_key_pressed(Key::Left) {
                -1.0
            } else if controller.is_key_pressed(Key::Right) {
                1.0
            } else {
                0.0
            };

            let accel_y = if controller.is_key_pressed(Key::Up) {
                -1.0
            } else if controller.is_key_pressed(Key::Down) {
                1.0
            } else {
                0.0
            };

            Vector2D::new(accel_x, accel_y)
        }

        self.physics.accelerate(convert_acceleration(controller));
    }

    pub fn apply_speed(&mut self) {
        self.position = self.physics.apply_movement(self.position);
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn position_x(&self) -> i32 {
        self.position.0
    }

    pub fn position_y(&self) -> i32 {
        self.position.1
    }
}
