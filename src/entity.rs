use crate::block::*;
use crate::hitbox::*;

use sdl2::render::Texture;

use serde::{Deserialize, Serialize};

use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub enum EntityType {
    Collectible(Collectible)
}

pub struct Entity<'a> {
    kind:    EntityType,
    hitbox:  Hitbox,
    texture: Rc<Texture<'a>>,
}

#[derive(PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum EntityTextureId {
    CollectibleCoin,
    CollectibleMushroom,
    CollectibleFlower,
}