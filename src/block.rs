use crate::level::*;
use crate::render::*;
use crate::resource::*;

use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

pub const BLOCK_SIZE: u32 = 64;

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Block {
    kind:     BlockType,
    contents: Option<BlockContents>,
    hidden:   bool,
}

#[derive(Copy, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(PartialEq, Eq)]
#[derive(FromPrimitive, Debug)]
#[repr(u8)]
pub enum BlockType {
    Bricks = 0,
    Rock,
    QuestionMark,
    Air,
}

pub struct ThemedBlock<'a> {
    pub block: &'a Block,
    pub theme: LevelTheme,
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

    pub fn next(&self) -> Option<Block> {
        self.kind.next().map(Block::from)
    }

    pub fn prev(&self) -> Option<Block> {
        self.kind.prev().map(Block::from)
    }

    pub fn is_visible(&self) -> bool {
        !self.hidden && self.kind.is_visible()
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
    fn next(self) -> Option<BlockType> {
        let next_id = (self as u8 + 1) % MAX_BLOCK;
        FromPrimitive::from_u8(next_id)
    }

    fn prev(self) -> Option<BlockType> {
        let id = self as u8;
        let prev_id = if id == 0 { MAX_BLOCK - 1 } else { id - 1 };
        FromPrimitive::from_u8(prev_id)
    }

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

impl<'a> Drawable for ThemedBlock<'a> {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let block = data.object.block;
        let (x, y) = data.position;
        let size = (BLOCK_SIZE as f64 * data.scale) as u32;

        let visible = block.is_visible();
        let in_view = data.camera.in_view(rect!(x, y, size, size));

        if !visible || !in_view {
            return;
        }

        let kind = block.kind;
        let theme = data.object.theme;

        let sprite_x = (kind.frame_index(data.tick) * BLOCK_SIZE) as i32;
        let sprite_y = (kind.variant_index(theme) * BLOCK_SIZE) as i32;
        let texture_name = kind
            .texture_name()
            .expect("Visible block has no associated texture name!");
        let texture = res.texture(texture_name);
        let src_region = rect!(sprite_x, sprite_y, BLOCK_SIZE, BLOCK_SIZE);

        let (cam_x, cam_y) = data.camera.translate_coords((x, y));
        let dest = rect!(cam_x, cam_y, size, size);

        dbg!(data.position);


        data.renderer
            .canvas
            .copy(&texture, src_region, dest)
            .unwrap();
    }
}
