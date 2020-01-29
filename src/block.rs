use crate::level::*;
use crate::render::*;
use crate::resource::*;

use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

pub const BLOCK_SIZE: u32 = 64;

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Block {
    kind:     BlockType,
    contents: Option<BlockContents>,
    hidden:   bool,
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum BlockType {
    Bricks = 1,
    Rock,
    QuestionMark,
    Air,
}

const MAX_BLOCK: u8 = BlockType::Air as u8;

impl From<BlockType> for Block {
    fn from(block_type: BlockType) -> Block {
        Block {
            kind:     block_type,
            contents: None,
            hidden:   false,
        }
    }
}

impl Default for Block {
    fn default() -> Block {
        Block::from(BlockType::default())
    }
}

impl Block {
    pub fn new(kind: BlockType, hidden: bool, contents: BlockContents) -> Self {
        Block {
            kind,
            hidden,
            contents: Some(contents),
        }
    }

    pub fn new_empty(kind: BlockType, hidden: bool) -> Self {
        Block {
            kind,
            hidden,
            contents: None,
        }
    }

    pub fn is_visible(&self) -> bool {
        !self.hidden && self.kind.is_visible()
    }

    pub fn animation_frame<'a>(
        &self,
        res: &'a mut ResourceManager,
        theme: LevelTheme,
        tick: u32,
    ) -> Option<AnimationFrame<'a>> {
        if !self.is_visible() {
            return None;
        }

        let block = self.kind;

        let x = (block.frame_index(tick) * BLOCK_SIZE) as i32;
        let y = (block.variant_index(theme) * BLOCK_SIZE) as i32;

        let texture_name = block.texture_name().unwrap();
        let texture = res.texture(texture_name);

        let region = rect!(x, y, BLOCK_SIZE, BLOCK_SIZE);

        Some(AnimationFrame { texture, region })
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum BlockContents {
    Coins(u8),
    Mushroom,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Air
    }
}

impl BlockType {
    fn texture_name(self) -> Option<&'static str> {
        match self {
            BlockType::Air => None,
            BlockType::Bricks => Some("brick"),
            BlockType::Rock => Some("rock"),
            BlockType::QuestionMark => Some("question_mark"),
        }
    }

    fn is_visible(self) -> bool {
        self != BlockType::Air
    }

    fn has_themes(self) -> bool {
        match self {
            BlockType::Bricks | BlockType::Rock => true,
            _ => false,
        }
    }

    fn is_animated(self) -> bool {
        match self {
            BlockType::QuestionMark => true,
            _ => false,
        }
    }

    fn frame_index(self, tick: u32) -> u32 {
        match self {
            BlockType::QuestionMark => (tick / FPS) % 2,
            _ => 0,
        }
    }

    fn variant_index(self, theme: LevelTheme) -> u32 {
        if self.has_themes() {
            theme as u32
        } else {
            0
        }
    }
}
