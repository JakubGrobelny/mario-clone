extern crate serde_json;
use serde::{Deserialize, Serialize};

use crate::utility::Result;
use std::path::Path;
use std::fs;

const CFG_FILE_NAME : &str = "config.json";


#[derive(Serialize, Deserialize)]
pub struct Config {
    window_height: u32,
    window_width: u32,
}

impl Config {
    pub fn new(res_path: &Path) -> Result<Config> {
        let cfg_path = res_path.join(CFG_FILE_NAME);
        let cfg_str = fs::read_to_string(cfg_path.as_path())?;
        let cfg : Config = serde_json::from_str(&cfg_str)?;
        Ok(cfg)
    }

    pub fn window_height(&self) -> u32 {
        self.window_height
    }

    pub fn window_width(&self) -> u32 {
        self.window_width
    }
}