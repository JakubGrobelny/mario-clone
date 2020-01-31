use crate::level::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

pub const BLOCK_SIZE: u32 = 64;

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Block {
    kind:     BlockType,
    contents: Option<BlockContents>,
}

#[derive(Copy, Clone)]
#[derive(Deserialize, Serialize, Hash)]
#[derive(PartialEq, Eq)]
#[derive(FromPrimitive, Debug)]
#[repr(u8)]
pub enum BlockType {
    Bricks = 0,
    Rock,
    RockLeft,
    RockMiddle,
    RockRight,
    Ground,
    GroundLeft,
    GroundMiddle,
    GroundRight,
    GroundBottom,
    GroundBottomLeft,
    GroundBottomMiddle,
    GroundBottomRight,
    QuestionMarkEmpty,
    QuestionMark,
    PipeUpperLeft,
    PipeUpperRight,
    PipeLowerLeft,
    PipeLowerRight,
    PipeSidewaysLeftBottom,
    PipeSidewaysLeftUpper,
    PipeSidewaysRightBottom,
    PipeSidewaysRightUpper,
    PipeJunctionLower,
    PipeJunctionUpper,
    TreeTrunk,
    TreeTrunkTop,
    TreeLeafsLeft,
    TreeLeafsMiddle,
    TreeLeafsRight,
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
        }
    }
}

impl Default for Block {
    fn default() -> Block {
        Block::from(BlockType::default())
    }
}

impl Block {
    pub fn new(kind: BlockType, contents: BlockContents) -> Self {
        Block {
            kind,
            contents: Some(contents),
        }
    }

    pub fn new_empty(kind: BlockType) -> Self {
        Block {
            kind,
            contents: None,
        }
    }

    pub fn collidable(self) -> bool {
        self.kind.collidable()
    }

    pub fn kind(self) -> BlockType {
        self.kind
    }

    pub fn next_kind(self) -> Block {
        Block::from(self.kind.next())
    }

    pub fn prev_kind(self) -> Block {
        Block::from(self.kind.prev())
    }

    pub fn is_visible(self) -> bool {
        self.kind.is_visible()
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
    fn next(self) -> BlockType {
        let next_id = (self as u8 + 1) % MAX_BLOCK;
        FromPrimitive::from_u8(next_id).unwrap()
    }

    fn prev(self) -> BlockType {
        let id = self as u8;
        let prev_id = if id == 0 { MAX_BLOCK - 1 } else { id - 1 };
        FromPrimitive::from_u8(prev_id).unwrap()
    }

    fn is_visible(self) -> bool {
        self != BlockType::Air
    }

    fn collidable(self) -> bool {
        match self {
            BlockType::TreeTrunk | BlockType::TreeTrunkTop | BlockType::Air => {
                false
            },
            _ => true,
        }
    }
}

impl<'a> Drawable for ThemedBlock<'a> {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let block = data.object.block;

        if !block.is_visible() {
            return;
        }

        let (src_region, dest, path) = {
            let info = res.block_texture_info(*block);
    
            let (x, y) = data.position;
            let width = (info.width as f64 * data.scale) as u32;
            let height = (info.height as f64 * data.scale) as u32;
    
            if !data.camera.in_view(rect!(x, y, width, height)) {
                return;
            }
    
            let theme = data.object.theme;
    
            let sprite_x = (info.frame_index(data.tick) * info.width) as i32;
            let sprite_y = (info.variant_index(theme) * info.height) as i32;
            
            let src_region = rect!(sprite_x, sprite_y, info.width, info.height);
            let (cam_x, cam_y) = data.camera.translate_coords((x, y));
            let dest = rect!(cam_x, cam_y, width, height);

            (src_region, dest, info.path.clone())
        };

        let texture = res.texture(&path);

        data.renderer
            .canvas
            .copy(&texture, src_region, dest)
            .unwrap();
    }
}
