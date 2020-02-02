use crate::controller::*;
use crate::hitbox::*;
use crate::physics::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;
use crate::level::*;
use crate::movement::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use vector2d::Vector2D;

pub struct Player {
    body:    PhysicalBody,
    variant: PlayerVariant,
}

const PLAYER_MASS: f64 = 1.0;
pub const PLAYER_WIDTH: u32 = 40;
pub const PLAYER_HEIGHT: u32 = 50;

pub enum PlayerVariant {
    Small,
    Big,
    CanShoot,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        let hitbox = Hitbox::new(x, y, PLAYER_WIDTH, PLAYER_HEIGHT);
        let mass = 0.8;

        Player {
            body:    PhysicalBody::new(mass, hitbox),
            variant: PlayerVariant::Small,
        }
    }

    pub fn accelerate(&mut self, controller: &Controller) {
        const HORIZONTAL_ACCELERATION: f64 = 1.6;
        const AIRBORNE_HANDICAP: f64 = 0.4;
        const JUMP_ACCELERATION: f64 = -11.0;
        const LONG_JUMP_MULT: f64 = 0.1;
        const SPRINT_MULT: f64 = 1.35;
        const SPEED_JUMP_BONUS: f64 = 0.02;

        let sprinting = controller.is_key_active(Key::Sprint);

        let mut x_accel = HORIZONTAL_ACCELERATION * controller.x_acceleration();
        
        if !self.body.grounded {
            x_accel *= AIRBORNE_HANDICAP;
        }

        if sprinting {
            x_accel *= SPRINT_MULT;
        }

        let jumping = controller.is_key_active(Key::Up);
        let y_accel = if jumping && self.body.speed_y() < 0.0 {
            self.body.speed_y() * LONG_JUMP_MULT
        } else if self.body.grounded && jumping {
            JUMP_ACCELERATION
        } else {
            0.0
        };

        let boosted_y_accel = y_accel + y_accel * x_accel * SPEED_JUMP_BONUS;
        let accel = vec2d!(x_accel, boosted_y_accel);
        self.body.accelerate(accel);
    }

    pub fn rect(&self) -> Rect {
        self.body.hitbox
    }

    pub fn apply_movement(&mut self, world: &mut PlayableLevel) {
        self.body.apply_movement(world, true);
    }

    pub fn stick_camera(&self, cam: &mut Camera) {
        let x = self.body.hitbox.x() - SCREEN_WIDTH as i32 / 2;
        let y = self.body.hitbox.y() - SCREEN_HEIGHT as i32 / 2;
        cam.move_to((x, y));
    }
}
