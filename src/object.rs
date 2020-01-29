use crate::block::*;
use crate::hitbox::*;

use sdl2::render::Texture;

use std::rc::Rc;

pub enum ObjectType {
    Block(BlockType),
}

pub struct Object<'a> {
    kind:    ObjectType,
    hitbox:  Hitbox,
    texture: Rc<Texture<'a>>,
}
