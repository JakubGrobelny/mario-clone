extern crate vector2d;
use vector2d::Vector2D;

use crate::hitbox::*;

pub struct Physics {
    mass: f64,
    speed: Vector2D<f64>,
}

impl Physics {
    pub fn new(mass: f64) -> Physics {
        Physics {
            mass,
            speed: Vector2D::new(0.0, 0.0),
        }
    }

    pub fn accelerate(&mut self, accel: Vector2D<f64>) {
        self.speed += accel
    }

    pub fn apply_movement(&self, position: (i32, i32)) -> (i32, i32) {
        let x_shift = self.speed.x.round() as i32;
        let y_shift = self.speed.y.round() as i32;
        (position.0 + x_shift, position.1 + y_shift)
    }
}

