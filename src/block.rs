use serde::{Deserialize, Serialize};

pub const BLOCK_SIZE: u32 = 64;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum BlockType {
    Air,
    Bricks,
    QuestionMark(BlockContents),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum BlockContents {
    Coins(u8),
    Mushroom,
    Empty,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Air
    }
}
