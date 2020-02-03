use crate::controller::*;
use crate::hitbox::*;
use crate::level::*;
use crate::movement::*;
use crate::physics::*;
use crate::render::*;
use crate::resource::*;
use crate::texture_id::*;
use crate::utility::*;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use vector2d::Vector2D;

pub struct Player {
    body:    PhysicalBody,
    variant: PlayerVariant,
}

const PLAYER_MASS: f64 = 1.0;
pub const PLAYER_WIDTH: u32 = 64;
pub const PLAYER_HEIGHT: u32 = 64;

#[derive(PartialEq, Eq)]
pub enum PlayerVariant {
    Small,
    Big,
    CanShoot,
}

impl Default for Player {
    fn default() -> Player {
        Player::new(10, SCREEN_HEIGHT as i32 - 70)
    }
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        let hitbox = Hitbox::new(x, y, PLAYER_WIDTH, PLAYER_HEIGHT);
        let mass = 0.83;

        Player {
            body:    PhysicalBody::new(mass, hitbox),
            variant: PlayerVariant::Small,
        }
    }

    pub fn is_big(&self) -> bool {
        self.variant != PlayerVariant::Small
    }

    pub fn accelerate(&mut self, controller: &Controller) {
        const HORIZONTAL_ACCELERATION: f64 = 0.9;
        const AIRBORNE_HANDICAP: f64 = 0.3;
        const JUMP_ACCELERATION: f64 = -13.5;
        const LONG_JUMP_MULT: f64 = 0.09;
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

        let jumped = controller.is_key_active_time_limited(Key::Up, 10);
        let holding_jump = controller.is_key_active(Key::Up);

        let y_accel = if holding_jump && self.body.speed_y() < 0.0 {
            self.body.speed_y() * LONG_JUMP_MULT
        } else if self.body.grounded && jumped {
            JUMP_ACCELERATION
        } else {
            0.0
        };

        let boosted_y_accel =
            y_accel + y_accel * x_accel.abs() * SPEED_JUMP_BONUS;
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

    pub fn position(&self) -> (i32, i32) {
        self.body.hitbox.top_left().into()
    }
}

impl Drawable for Player {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        const MOVEMENT_THRESHOLD: f64 = 0.3;
        let player = data.object;
        let variant = if !player.body.grounded {
            TextureId::PlayerJumping
        } else if player.body.is_still() {
            TextureId::PlayerStanding
        } else {
            TextureId::PlayerRunning
        };

        let flip = player.body.x_direction() == XDirection::Right;

        let info = res.entity_texture_info(variant);
        let (x, y) = player.position();
        let tick = data.tick + player.body.speed_x().abs() as u32;
        let sprite_x = info.frame_index(tick) * info.width;
        let src_region = rect!(sprite_x, 0, info.width, info.height);

        let (cam_x, cam_y) = data.camera.translate_coords((x, y));
        let width = (info.width as f64 * data.scale) as u32;
        let height = (info.height as f64 * data.scale) as u32;
        let dest = rect!(cam_x, cam_y, width, height);

        let path = info.path.clone();

        data.renderer
            .canvas
            .copy_ex(
                &res.texture(&path),
                src_region,
                dest,
                0.0,
                Point::new(0, 0),
                flip,
                false,
            )
            .expect("Failed to draw the player!");
    }
}
