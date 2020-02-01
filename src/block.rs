use crate::level::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use sdl2::pixels::Color;

pub const BLOCK_SIZE: u32 = 64;

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Block {
    kind:     BlockType,
    contents: Collectible,
}

#[derive(Copy, Clone)]
#[derive(Deserialize, Serialize, Hash)]
#[derive(PartialEq, Eq)]
#[derive(FromPrimitive, Debug)]
#[repr(u8)]
pub enum BlockType {
    Bricks = 0,
    SolidBox,
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
    TreeLeafsLeft,
    TreeLeafsMiddle,
    TreeLeafsRight,
    Air,
}

pub struct ThemedBlock {
    pub block: Block,
    pub theme: LevelTheme,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Collectible {
    Coins(u8),
    Mushroom,
    Flower,
    Empty,
}

const MAX_BLOCK: u8 = BlockType::Air as u8;

impl From<BlockType> for Block {
    fn from(block_type: BlockType) -> Block {
        Block {
            kind:     block_type,
            contents: Collectible::Empty,
        }
    }
}

impl Default for Block {
    fn default() -> Block {
        Block::from(BlockType::default())
    }
}

impl Block {
    pub fn default_visible() -> Self {
        Self::from(BlockType::Bricks)
    }

    pub fn new(kind: BlockType, contents: Collectible) -> Self {
        Block { kind, contents }
    }

    pub fn is_collidable(self) -> bool {
        self.kind.is_collidable()
    }

    pub fn is_empty(self) -> bool {
        match self.contents {
            Collectible::Empty => true,
            _ => false,
        }
    }

    pub fn is_bumpable(self) -> bool {
        self.kind().is_bumpable()
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

    pub fn insert_item(&mut self, item: Collectible) {
        assert_ne!(item, Collectible::Coins(0));
        self.contents = match (self.contents, item) {
            (Collectible::Coins(n), Collectible::Coins(m)) => {
                Collectible::Coins(n + m)
            },
            (_, item) => item,
        }
    }

    pub fn delete_item(&mut self) {
        self.contents = match self.contents {
            Collectible::Coins(n) if n > 1 => Collectible::Coins(n - 1),
            _ => Collectible::Empty,
        }
    }

    pub fn is_visible(self) -> bool {
        self.kind.is_visible()
    }

    pub fn get_contents(self) -> Collectible {
        self.contents
    }
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

    fn is_collidable(self) -> bool {
        self != BlockType::Air
    }

    fn is_bumpable(self) -> bool {
        match self {
            BlockType::Bricks | BlockType::QuestionMark => true,
            _ => false,
        }
    }
}

impl Drawable for ThemedBlock {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let block = data.object.block;

        if !block.is_visible() {
            return;
        }

        let (src_region, dest, path) = {
            let info = res.block_texture_info(block);
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

            if data.mode == DrawMode::EditorSelection {
                let rect = rect!(x, y, info.width, info.height);
                data.renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
                data.renderer.canvas.draw_rect(rect).expect(
                    "Failed to draw selection rectangle in the editor!",
                );
            }
            (src_region, dest, info.path.clone())
        };

        data.renderer
            .canvas
            .copy(&res.texture(&path), src_region, dest)
            .unwrap();

        let is_editor = data.mode == DrawMode::Editor
            || data.mode == DrawMode::EditorSelection;
        if is_editor && !block.is_empty() {
            pass_draw!(data, &block.contents).show(res);
        }
    }
}

impl Collectible {
    pub fn next(self) -> Self {
        match self {
            Collectible::Coins(_) => Collectible::Mushroom,
            Collectible::Mushroom => Collectible::Flower,
            Collectible::Flower => Collectible::Coins(1),
            Collectible::Empty => {
                panic!("Can't take the successor of empty collectible!")
            },
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Collectible::Coins(_) => Collectible::Flower,
            Collectible::Flower => Collectible::Mushroom,
            Collectible::Mushroom => Collectible::Coins(1),
            Collectible::Empty => {
                panic!("Can't take the predecessor of empty collectible!")
            },
        }
    }
}

impl Drawable for Collectible {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        // TODO: implement
        match data.mode {
            DrawMode::Editor | DrawMode::EditorSelection => {
                let text = match data.object {
                    Collectible::Coins(amount) => format!("{}", amount),
                    Collectible::Flower => String::from("F"),
                    Collectible::Mushroom => String::from("M"),
                    _ => String::from(""),
                };

                let text = text!(&text);
                pass_draw!(data, &text).scale(0.4).show(res);
            },
            _ => unimplemented!(),
        }
    }
}
