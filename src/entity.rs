use crate::block::*;
use crate::enemy::*;
use crate::hitbox::*;
use crate::level::*;
use crate::physics::*;
use crate::render::*;
use crate::resource::*;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use serde::{Deserialize, Serialize};

use std::rc::Rc;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Entity {
    pub kind: EntityType,
    pub body: PhysicalBody,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct EntityPrototype {
    kind:     EntityType,
    position: (i32, i32),
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub enum EntityType {
    Collectible(Collectible),
    Enemy(EnemyType),
    Particle(Particle),
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub enum Particle {
    Coin,
    BlockFragment { kind: BlockType, theme: LevelTheme },
}

impl Particle {
    pub fn new_fragment(kind: BlockType, theme: LevelTheme) -> Self {
        Particle::BlockFragment { kind, theme }
    }
}

impl EntityPrototype {
    pub fn hitbox(self) -> Hitbox {
        let (x, y) = self.position;
        match self.kind {
            EntityType::Collectible(..) => {
                Hitbox::new(x, y, BLOCK_SIZE, BLOCK_SIZE)
            },
            EntityType::Enemy(..) => {
                unimplemented!();
            },
            EntityType::Particle(particle) => Hitbox::new(x, y, 1, 1),
        }
    }

    pub fn mass(self) -> f64 {
        1.0
    }
}

impl From<EntityPrototype> for Entity {
    fn from(prototype: EntityPrototype) -> Entity {
        let hitbox = prototype.hitbox();
        let mass = prototype.mass();
        let body = PhysicalBody::new(mass, hitbox);

        Entity {
            kind: prototype.kind,
            body,
        }
    }
}

impl From<&Entity> for EntityPrototype {
    fn from(entity: &Entity) -> EntityPrototype {
        EntityPrototype::new(entity.kind, entity.body.position())
    }
}

impl EntityPrototype {
    pub fn new(kind: EntityType, pos: (i32, i32)) -> EntityPrototype {
        EntityPrototype {
            kind,
            position: pos,
        }
    }
}

impl Entity {
    pub fn new(kind: EntityType, pos: (i32, i32)) -> Entity {
        Entity::from(EntityPrototype::new(kind, pos))
    }

    pub fn spawn(kind: EntityType, (x, y): (usize, usize)) -> Entity {
        let x = x as i32 * BLOCK_SIZE as i32;
        let y = (y as i32) * BLOCK_SIZE as i32;
        Self::new(kind, (x, y))
    }
}

impl Drawable for Particle {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        match data.object {
            Particle::Coin => unimplemented!(),
            Particle::BlockFragment { kind, theme } => {
                let info = res.block_texture_info(*kind);

                const SIZE: u32 = 13;
                
                let (x, y) = data.position;
                let x = x + BLOCK_SIZE as i32 / 2 - (SIZE / 2) as i32;
                let y = y + BLOCK_SIZE as i32 / 2 - (SIZE / 2) as i32;

                if !data.camera.in_view(rect!(x, y, SIZE, SIZE)) {
                    return;
                }

                let sprite_y =
                    (info.variant_index(*theme) * info.height) as i32;

                let src_region = rect!(0, sprite_y, SIZE, SIZE);

                let (cam_x, cam_y) = data.camera.translate_coords((x, y));
                let dest = rect!(cam_x, cam_y, SIZE, SIZE);

                let path = info.path.clone();

                data.renderer
                    .canvas
                    .copy(&res.texture(&path), src_region, dest)
                    .expect("Failed to draw a particle!");
            },
        }
    }
}

impl Drawable for EntityPrototype {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        match data.object.kind {
            EntityType::Collectible(collectible) => {
                pass_draw!(data, &collectible)
                    .position(data.object.position)
                    .show(res);
            },
            EntityType::Particle(particle) => {
                pass_draw!(data, &particle)
                    .position(data.object.position)
                    .show(res);
            },
            _ => unimplemented!(),
        }
    }
}

impl Drawable for Entity {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let prototype = EntityPrototype::from(data.object);
        pass_draw!(data, &prototype)
            .position(data.object.body.position())
            .show(res);
    }
}
