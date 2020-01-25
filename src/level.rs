use crate::block::*;
use crate::utility::*;

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub const LEVEL_HEIGHT: usize = 20;
pub const LEVEL_WIDTH: usize = 220;

#[derive(Clone)]
pub struct Level {
    theme: LevelTheme,
    blocks: [[BlockType; LEVEL_WIDTH]; LEVEL_HEIGHT],
}

#[derive(Deserialize, Serialize)]
struct LevelJSON {
    theme: LevelTheme,
    blocks: Vec<BlockType>,
}

#[derive(Deserialize, Serialize, Copy, Clone)]
enum LevelTheme {
    Underground,
    Day,
    Night,
}

impl From<LevelJSON> for Level {
    fn from(json: LevelJSON) -> Level {
        if json.blocks.len() != LEVEL_HEIGHT * LEVEL_WIDTH {
            panic_with_messagebox("Corrupted level data (invalid level size)!");
        }

        let mut blocks = [[BlockType::default(); LEVEL_WIDTH]; LEVEL_HEIGHT];
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
    pub fn new(path: &Path) -> Result<Level> {
        let file_contents = fs::read_to_string(path)?;
        let level_json: LevelJSON = serde_json::from_str(&file_contents)?;
        Ok(Level::from(level_json))
    }
}

pub fn load_levels(res_path: &Path) -> Result<Vec<(String, Level)>> {
    #[derive(Deserialize, Serialize)]
    struct LevelList {
        levels: Vec<String>,
    }

    let levels_path = res_path.join("levels/");
    let level_list_str = fs::read_to_string(levels_path.join("levels.json"))?;
    let level_list: LevelList = serde_json::from_str(&level_list_str)?;

    Ok(level_list
        .levels
        .into_iter()
        .map(|name| {
            let path = levels_path.join(format!("{}.lvl", name));
            let level = Level::new(path.as_path()).unwrap_or_else(|_| {
                let error_msg = format!("Failed to load level '{}'!", name);
                panic_with_messagebox(&error_msg);
            });
            (name, level)
        })
        .collect())
}
