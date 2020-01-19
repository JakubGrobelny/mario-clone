use crate::block::*;

pub struct Level {
    worlds: Vec<World>,
}

enum WorldTheme {
    Underground,
    Normal
}

const WORLD_HEIGHT : usize = 64;

pub struct World {
    theme: WorldTheme,
    blocks: Vec<[Block; WORLD_HEIGHT]>,
}


