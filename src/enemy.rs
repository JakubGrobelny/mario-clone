use crate::render::*;
use crate::resource::*;
use crate::texture_id::*;
use crate::utility::*;

use serde::{Deserialize, Serialize};

use sdl2::rect::Point;

use num_traits::FromPrimitive;

// https://www.mariowiki.com/List_of_enemies_by_game#Super_Mario_Bros.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum EnemyType {
    Goomba,
    Koopa,
    /* FlyingKoopa,
     * PiranhaPlant,
     * EmptyShell,
     * BuzzyBeetle,
     * Spiny,
     * HammerBro, */
}

pub const ENEMY_KILL_BOUNCE : f64 = -10.0;

const MAX_EDITOR_SELECTION: u8 = EnemyType::Koopa as u8;

pub const GOOMBA_ACCELERATION: f64 = 0.4;

impl EnemyType {
    pub fn prev(self) -> Self {
        let id = self as u8;
        let prev_id = if id == 0 {
            MAX_EDITOR_SELECTION
        } else {
            id - 1
        };
        FromPrimitive::from_u8(prev_id).unwrap()
    }

    pub fn next(self) -> Self {
        let next_id = (self as u8 + 1) % (MAX_EDITOR_SELECTION + 1);
        FromPrimitive::from_u8(next_id).unwrap()
    }

    pub fn texture_id(self) -> TextureId {
        match self {
            EnemyType::Goomba => TextureId::EnemyGoomba,
            EnemyType::Koopa => TextureId::EnemyKoopa,
        }
    }
}

impl Drawable for EnemyType {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let texture_id = data.object.texture_id();
        let info = res.entity_texture_info(texture_id);

        let (x, y) = data.position;
        let (off_x, off_y) = info.hitbox_offset();
        let (x, y) = (x + off_x, y + off_y);

        let width = (info.width as f64 * data.scale) as u32;
        let height = (info.height as f64 * data.scale) as u32;

        if !data.camera.in_view(rect!(x, y, width, height)) {
            return;
        }

        let sprite_x = info.frame_index(data.tick) * info.width;
        let src_region = rect!(sprite_x, 0, info.width, info.height);

        let (cam_x, cam_y) = data.camera.translate_coords((x, y));
        let dest = rect!(cam_x, cam_y, width, height);

        let flip = data.mode == DrawMode::EntityDirection(XDirection::Right);
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
            .expect("Failed to draw an enemy!");
    }
}
