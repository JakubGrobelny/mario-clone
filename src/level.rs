use crate::block::*;
use crate::resource::*;
use crate::utility::*;

use std::path::Path;
use std::fs;

use serde::{Deserialize, Serialize};

const LEVEL_HEIGHT : usize = 20;
const LEVEL_WIDTH : usize = 220;

#[derive(Deserialize, Serialize)]
pub struct Level {
    theme: LevelTheme,
    blocks: [Vec<Block>; LEVEL_HEIGHT],
}


#[derive(Deserialize, Serialize)]
enum LevelTheme {
    Underground,
    Day,
    Night,
}

impl Level {    
    pub fn new(path: &Path) -> Result<Level> {
        let file_contents = fs::read_to_string(path)?;
        let level : Level = serde_json::from_str(&file_contents)?;        
        if level.blocks.iter().any(|vec| vec.len() != LEVEL_WIDTH) {
            return Err("Invalid file length!".into());
        }
        Ok(level)
    }
}
