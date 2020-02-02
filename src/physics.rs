extern crate vector2d;
use vector2d::Vector2D;

#[derive(Debug)]
pub struct Physics {
    mass:  f64,
    speed: Vector2D<f64>,
}

#[macro_export]
macro_rules! vec2d {
    ($x:expr, $y:expr) => {
        Vector2D { x: $x, y: $y }
    };
}

const GRAVITY: f64 = 1.0;

const AIR_DRAG: f64 = 0.045;
const AIR_DRAG_VEC: Vector2D<f64> = vec2d!(AIR_DRAG, AIR_DRAG);

const GROUND_DRAG: f64 = 0.15;
const GROUND_DRAG_VEC: Vector2D<f64> = vec2d!(GROUND_DRAG, 0.0);

impl Physics {
    pub fn new(mass: f64) -> Physics {
        Physics {
            mass,
            speed: vec2d!(0.0, 0.0),
        }
    }

    pub fn accelerate(&mut self, ground: bool, accel: Vector2D<f64>) {
        let grav_accel =
            vec2d!(0.0, if ground { 0.0 } else { self.mass * GRAVITY });

        let drag = self.speed.mul_components(
            AIR_DRAG_VEC
                + if ground {
                    GROUND_DRAG_VEC
                } else {
                    vec2d!(0.0, 0.0)
                },
        );

        self.speed = self.speed + accel + grav_accel - drag;
    }

    pub fn apply_speed(&self, position: (i32, i32)) -> (i32, i32) {
        let x_shift = self.speed.x.round() as i32;
        let y_shift = self.speed.y.round() as i32;
        (position.0 + x_shift, position.1 + y_shift)
    }
}
