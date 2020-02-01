use crate::block::*;
use crate::hitbox::*;

use sdl2::render::Texture;

use std::rc::Rc;
use std::hash::Hasher;

pub enum EntityType {
    Block(BlockType),
}

pub struct Entity<'a> {
    kind:    EntityType,
    hitbox:  Hitbox,
    texture: Rc<Texture<'a>>,
}
