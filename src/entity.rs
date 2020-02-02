use crate::block::*;
use crate::hitbox::*;
use crate::enemy::*;

use sdl2::render::Texture;

use serde::{Deserialize, Serialize};

use std::rc::Rc;


#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct EntityPrototype {
    kind:     EntityType,
    position: (i32, i32),
    buff:     Option<EntityBuff>,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub enum EntityType {
    Collectible(Collectible),
    Enemy(EnemyType)
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub enum EntityBuff {
    Flying,
    Large,
}

pub struct Entity {
    kind:   EntityType,
    hitbox: Hitbox,
}

#[derive(PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum EntityTextureId {
    CollectibleCoin,
    CollectibleMushroom,
    CollectibleFlower,
}
