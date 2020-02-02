extern crate vector2d;
use vector2d::Vector2D;

use crate::hitbox::*;

#[macro_export]
macro_rules! vec2d {
    ($x:expr, $y:expr) => {
        Vector2D { x: $x, y: $y }
    };
}

#[derive(Debug)]
pub struct Physics {
    mass:  f64,
    speed: Vector2D<f64>,
}

pub struct PhysicalBody {
    physics:      Physics,
    pub hitbox:   Hitbox,
    pub grounded: bool,
}

impl PhysicalBody {
    pub fn new(mass: f64, hitbox: Hitbox) -> Self {
        PhysicalBody {
            physics: Physics::new(mass),
            hitbox,
            grounded: false,
        }
    }

    pub fn accelerate(&mut self, accel: Vector2D<f64>) {
        self.physics.accelerate(self.grounded, accel);
    }

    pub fn move_by_vec(&mut self, vec: Vector2D<i32>) {
        self.hitbox.offset(vec.x, vec.y);
    }

    pub fn move_by(&mut self, (x, y): (i32, i32)) {
        self.hitbox.offset(x, y);
    }

    pub fn speed(&self) -> Vector2D<f64> {
        self.physics.speed
    }

    pub fn speed_y(&self) -> f64 {
        self.physics.speed.y
    }

    pub fn speed_x(&self) -> f64 {
        self.physics.speed.x
    }

    pub fn position(&self) -> (i32, i32) {
        (self.hitbox.x(), self.hitbox.y())
    }
}

const GRAVITY: f64 = 1.0;

const AIR_DRAG: f64 = 0.045;
const AIR_DRAG_VEC: Vector2D<f64> = vec2d!(AIR_DRAG, AIR_DRAG);

const GROUND_DRAG: f64 = 0.15;
const GROUND_DRAG_VEC: Vector2D<f64> = vec2d!(GROUND_DRAG, 0.0);

impl Physics {
    fn new(mass: f64) -> Physics {
        Physics {
            mass,
            speed: vec2d!(0.0, 0.0),
        }
    }

    fn accelerate(&mut self, ground: bool, accel: Vector2D<f64>) {
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
}

pub fn vec_map<A, B>(vector: &Vector2D<A>, f: fn(A) -> B) -> Vector2D<B>
where
    A: Copy,
{
    vec2d!(f(vector.x), f(vector.y))
}
