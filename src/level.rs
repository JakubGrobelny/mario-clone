use crate::block::*;
use crate::render::*;
use crate::resource::*;

use sdl2::pixels::Color;

use serde::{Deserialize, Serialize};

pub const LEVEL_HEIGHT: usize = 20;
pub const LEVEL_WIDTH: usize = 220;

#[derive(Clone)]
pub struct Level {
    pub theme: LevelTheme,
    blocks:    Box<[[Block; LEVEL_WIDTH]; LEVEL_HEIGHT]>,
}

#[derive(Deserialize, Serialize)]
pub struct LevelJSON {
    theme:  LevelTheme,
    blocks: Vec<Block>,
}

#[derive(Deserialize, Serialize, Copy, Clone)]
#[repr(u8)]
pub enum LevelTheme {
    Day,
    Underground,
    Night,
}

impl From<&Level> for LevelJSON {
    fn from(lvl: &Level) -> LevelJSON {
        let blocks: Vec<Block> = lvl
            .blocks
            .iter()
            .map(|row| row.iter())
            .flatten()
            .copied()
            .collect();
        LevelJSON {
            theme: lvl.theme,
            blocks,
        }
    }
}

impl LevelTheme {
    pub fn next(self) -> LevelTheme {
        match self {
            LevelTheme::Day => LevelTheme::Underground,
            LevelTheme::Underground => LevelTheme::Night,
            LevelTheme::Night => LevelTheme::Day,
        }
    }

    pub fn prev(self) -> LevelTheme {
        match self {
            LevelTheme::Day => LevelTheme::Night,
            LevelTheme::Underground => LevelTheme::Day,
            LevelTheme::Night => LevelTheme::Underground,
        }
    }
}

impl Default for Level {
    fn default() -> Level {
        Level::new()
    }
}

impl From<LevelJSON> for Level {
    fn from(json: LevelJSON) -> Level {
        if json.blocks.len() != LEVEL_HEIGHT * LEVEL_WIDTH {
            panic_with_messagebox!(
                "Corrupted level state (invalid level size)!"
            );
        }

        let mut blocks =
            Box::new([[Block::default(); LEVEL_WIDTH]; LEVEL_HEIGHT]);

        for (i, block) in json.blocks.into_iter().enumerate() {
            let row = i / LEVEL_WIDTH;
            let col = i % LEVEL_WIDTH;
            blocks[row][col] = block;
        }

        Level {
            theme: json.theme,
            blocks,
        }
    }
}

impl Level {
    pub fn new() -> Level {
        let mut blocks =
            Box::new([[Block::default(); LEVEL_WIDTH]; LEVEL_HEIGHT]);

        blocks[LEVEL_HEIGHT - 2][0] = Block::from(BlockType::GroundLeft);
        blocks[LEVEL_HEIGHT - 2][LEVEL_WIDTH - 1] =
            Block::from(BlockType::GroundRight);
        for col in 1..LEVEL_WIDTH - 1 {
            blocks[LEVEL_HEIGHT - 2][col] =
                Block::from(BlockType::GroundMiddle);
        }

        blocks[LEVEL_HEIGHT - 1][0] = Block::from(BlockType::GroundBottomLeft);
        blocks[LEVEL_HEIGHT - 1][LEVEL_WIDTH - 1] =
            Block::from(BlockType::GroundBottomRight);
        for col in 1..LEVEL_WIDTH - 1 {
            blocks[LEVEL_HEIGHT - 1][col] =
                Block::from(BlockType::GroundBottomMiddle);
        }

        Level {
            blocks,
            theme: LevelTheme::Day,
        }
    }

    pub fn set_block(&mut self, (x, y): (usize, usize), block: Block) {
        self.blocks[y][x] = block;
    }
}

impl From<LevelTheme> for Color {
    fn from(theme: LevelTheme) -> Color {
        match theme {
            LevelTheme::Day => Color::RGB(88, 100, 255),
            LevelTheme::Night => Color::RGB(0, 0, 0),
            LevelTheme::Underground => Color::RGB(0, 0, 0),
        }
    }
}

impl Drawable for Level {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let color = Color::from(data.object.theme);
        data.renderer.canvas.set_draw_color(color);
        data.renderer.canvas.clear();

        for (y, row) in data.object.blocks.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                let x = x as i32 * BLOCK_SIZE as i32;
                let y = y as i32 * BLOCK_SIZE as i32;
                let block = ThemedBlock {
                    block: &block,
                    theme: data.object.theme,
                };

                pass_draw!(data, &block).position((x, y)).show(res);
            }
        }
    }
}
