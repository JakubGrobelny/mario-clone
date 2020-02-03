extern crate vector2d;
use vector2d::Vector2D;

use crate::block::*;
use crate::hitbox::*;
use crate::level::*;
use crate::utility::*;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Physics {
    mass:  f64,
    speed: Vector2D<f64>,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct PhysicalBody {
    physics:       Physics,
    pub hitbox:    Hitbox,
    pub grounded:  bool,
    pub direction: XDirection,
}

impl PhysicalBody {
    pub fn new(mass: f64, hitbox: Hitbox) -> Self {
        PhysicalBody {
            physics: Physics::new(mass),
            hitbox,
            grounded: false,
            direction: XDirection::Still,
        }
    }

    pub fn out_of_bounds(&self) -> bool {
        let world: Hitbox = rect!(
            0,
            0,
            LEVEL_WIDTH * BLOCK_SIZE as usize,
            LEVEL_HEIGHT * BLOCK_SIZE as usize
        );
        !self.hitbox.collides(&world)
    }

    pub fn is_still_x(&self) -> bool {
        self.physics.speed.x.abs() < 1.0
    }

    pub fn is_still_y(&self) -> bool {
        self.physics.speed.y.abs() < 1.0
    }

    pub fn is_still(&self) -> bool {
        self.is_still_x() && self.is_still_y()
    }

    pub fn x_direction(&self) -> XDirection {
        self.direction
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

    pub fn stop_x(&mut self) {
        self.physics.speed.x = 0.0;
    }

    pub fn stop_y(&mut self) {
        self.physics.speed.y = 0.0;
    }

    pub fn clear_speed(&mut self) {
        const CLEAR_THRESHOLD: f64 = 0.2;
        let speed = self.physics.speed;
        self.physics.speed =
            vec_map(
                &speed,
                |x| if x.abs() < CLEAR_THRESHOLD { 0.0 } else { x },
            );
    }

    pub fn apply_movement_unchecked(&mut self) {
        let Vector2D { x, y } = self.physics.speed;
        self.hitbox.offset(x.round() as i32, y.round() as i32);
    }
}

const GRAVITY: f64 = 1.0;

const AIR_DRAG: f64 = 0.045;
const AIR_DRAG_VEC: Vector2D<f64> = vec2d!(AIR_DRAG, AIR_DRAG);

const GROUND_DRAG: f64 = 0.10;
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

        let drag_x = if ground {
            GROUND_DRAG_VEC
        } else {
            vec2d!(0.0, 0.0)
        };

        let drag = self.speed.mul_components(drag_x + AIR_DRAG_VEC);

        self.speed += accel + grav_accel - drag;
    }
}
