use crate::block::*;
use crate::hitbox::*;
use crate::enemy::*;
use crate::physics::*;

use sdl2::render::Texture;
use sdl2::rect::Rect;

use serde::{Deserialize, Serialize};

use std::rc::Rc;


#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct EntityPrototype {
    kind:     EntityType,
    position: (i32, i32),
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub enum EntityType {
    Collectible(Collectible),
    Enemy(EnemyType)
}

pub struct PhysicalBody {
    physics: Physics,
    hitbox: Rect,
    grounded: bool,
}

impl PhysicalBody {

}