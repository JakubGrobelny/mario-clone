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
                "Corrupted level data (invalid level size)!"
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
        const GROUND_HEIGHT: usize = 3;
        for col in 0..LEVEL_WIDTH {
            for row in LEVEL_HEIGHT - GROUND_HEIGHT..LEVEL_HEIGHT {
                blocks[row][col] = Block::from(BlockType::Rock);
            }
        }

        Level {
            blocks,
            theme: LevelTheme::Day,
        }
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
    fn draw(
        &self,
        renderer: &mut Renderer,
        cam: &Camera,
        res: &mut ResourceManager,
        tick: u32,
    ) {
        let color = Color::from(self.theme);
        renderer.canvas.set_draw_color(color);
        renderer.canvas.clear();

        for (y, row) in self.blocks.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                let x = x as i32 * BLOCK_SIZE as i32;
                let y = y as i32 * BLOCK_SIZE as i32;

                let visible = block.is_visible();
                let in_view = cam.in_view(rect!(x, y, BLOCK_SIZE, BLOCK_SIZE));

                if visible && in_view {
                    let frame = block.animation_frame(res, self.theme, tick);
                    if let Some(frame) = frame {
                        frame.draw(renderer, cam, (x, y))
                    }
                }
            }
        }
    }
}
