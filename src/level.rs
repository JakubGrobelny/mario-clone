use crate::block::*;
use crate::resource::*;
use crate::utility::*;

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

const LEVEL_HEIGHT: usize = 20;
const LEVEL_WIDTH: usize = 220;

#[derive(Deserialize, Serialize, Clone)]
pub struct Level {
    theme: LevelTheme,
    blocks: [Vec<Block>; LEVEL_HEIGHT],
}

#[derive(Deserialize, Serialize, Copy, Clone)]
enum LevelTheme {
    Underground,
    Day,
    Night,
}

impl Level {
    pub fn new(path: &Path) -> Result<Level> {
        let file_contents = fs::read_to_string(path)?;
        let level: Level = serde_json::from_str(&file_contents)?;
        if level.blocks.iter().any(|vec| vec.len() != LEVEL_WIDTH) {
            Err("Invalid level length!".into())
        } else {
            Ok(level)
        }
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

    Ok(level_list.levels.into_iter().map(|name| {
        let path = levels_path.join(format!("{}.lvl", name));
        let level = Level::new(path.as_path())
            .expect(&format!("Failed to load level '{}'!", name));
        (name, level)
    }).collect())
}
