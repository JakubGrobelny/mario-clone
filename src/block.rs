use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Block {
    Air,
    Bricks,
}

impl Default for Block {
    fn default() -> Self {
        Block::Air
    }
}