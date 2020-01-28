use crate::block::*;
use crate::object::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use sdl2::rect::Rect;

pub const LEVEL_HEIGHT: usize = 20;
pub const LEVEL_WIDTH: usize = 220;

#[derive(Clone)]
pub struct Level {
    theme: LevelTheme,
    blocks: Box<[[BlockType; LEVEL_WIDTH]; LEVEL_HEIGHT]>,
}

#[derive(Deserialize, Serialize)]
pub struct LevelJSON {
    theme: LevelTheme,
    blocks: Vec<BlockType>,
}

#[derive(Deserialize, Serialize, Copy, Clone)]
#[repr(u8)]
pub enum LevelTheme {
    Day,
    Underground,
    Night,
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
                "Corrupted level data (invalid level size)!"
            );
        }

        let mut blocks =
            Box::new([[BlockType::default(); LEVEL_WIDTH]; LEVEL_HEIGHT]);
        for (i, block) in json.blocks.into_iter().enumerate() {
            let row = i / LEVEL_HEIGHT;
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
            Box::new([[BlockType::default(); LEVEL_WIDTH]; LEVEL_HEIGHT]);
        const GROUND_HEIGHT: usize = 3;
        for col in 0..LEVEL_WIDTH {
            for row in LEVEL_HEIGHT - GROUND_HEIGHT..LEVEL_HEIGHT {
                blocks[row][col] = BlockType::Bricks;
            }
        }

        Level {
            blocks,
            theme: LevelTheme::Day,
        }
    }
}

impl Drawable for Level {
    fn draw(
        &self,
        renderer: &mut Renderer,
        cam: &Camera,
        res: &mut ResourceManager,
        tick: u32,
    ) {

        for (y, row) in self.blocks.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                let x = x as i32 * BLOCK_SIZE as i32;
                let y = y as i32 * BLOCK_SIZE as i32;

                if !cam.in_view(rect!(x, y, BLOCK_SIZE, BLOCK_SIZE)) {
                    continue;
                }

                if let Some(frame) =
                    block.get_animation_frame(res, self.theme, tick)
                {                    
                    frame.draw(renderer, cam, (x, y))
                }
            }
        }
    }
}
