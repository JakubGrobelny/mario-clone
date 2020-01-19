extern crate serde_json;
use serde::{Deserialize, Serialize};

use crate::utility::Result;
use std::fs;
use std::path::Path;

use crate::keybindings::*;

const CFG_FILE_NAME: &str = "config.json";

fn default_window_height() -> u32 {
    720
}

fn default_window_width() -> u32 {
    1280
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_window_height")]
    window_height: u32,
    #[serde(default = "default_window_width")]
    window_width: u32,
    #[serde(default = "default_key_bindings")]
    key_bindings: KeyBindings,
}

impl Config {
    pub fn new(res_path: &Path) -> Result<Config> {
        let cfg_path = res_path.join(CFG_FILE_NAME);
        let cfg_str = fs::read_to_string(cfg_path.as_path())?;
        let cfg: Config = serde_json::from_str(&cfg_str)?;
        eprintln!("{:?}", cfg);
        Ok(cfg)
    }

    pub fn window_height(&self) -> u32 {
        self.window_height
    }

    pub fn window_width(&self) -> u32 {
        self.window_width
    }
}
