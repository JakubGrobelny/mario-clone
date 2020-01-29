use crate::level::*;
use crate::render::*;
use crate::resource::*;

use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

pub const BLOCK_SIZE: u32 = 64;

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Block {
    kind: BlockType,
    contents: BlockContents,
    hidden: bool,
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum BlockType {
    Bricks = 1,
    Rock,
    QuestionMark,
    Air,
}

const MAX_BLOCK : u8 = BlockType::Air as u8;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

impl BlockType {
    pub fn texture_name(self) -> Option<&'static str> {
        match self {
            BlockType::Air => None,
            BlockType::Bricks => Some("brick"),
            BlockType::Rock => Some("rock"),
            BlockType::QuestionMark => Some("question_mark"),
        }
    }

    pub fn is_visible(self) -> bool {
        self != BlockType::Air
    }

    pub fn has_themes(self) -> bool {
        match self {
            BlockType::Bricks | BlockType::Rock => true,
            _ => false,
        }
    }

    pub fn is_animated(self) -> bool {
        match self {
            BlockType::QuestionMark => true,
            _ => false,
        }
    }

    pub fn frame_index(self, tick: u32) -> u32 {
        match self {
            BlockType::QuestionMark => (tick / FPS) % 2,
            _ => 0,
        }
    }

    pub fn variant_index(self, theme: LevelTheme) -> u32 {
        if self.has_themes() {
            theme as u32
        } else {
            0
        }
    }
}

impl BlockType {
    pub fn get_animation_frame<'a>(
        &self,
        res: &'a mut ResourceManager,
        theme: LevelTheme,
        tick: u32,
    ) -> Option<AnimationFrame<'a>> {
        if !self.is_visible() {
            return None;
        }

        let x = (self.frame_index(tick) * BLOCK_SIZE) as i32;
        let y = (self.variant_index(theme) * BLOCK_SIZE) as i32;

        let texture_name = self.texture_name().unwrap();
        let texture = res.texture(texture_name);

        let region = rect!(x, y, BLOCK_SIZE, BLOCK_SIZE);

        Some(AnimationFrame { texture, region })
    }
}
