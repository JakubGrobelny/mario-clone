extern crate serde_json;
use serde::{Deserialize, Serialize};

use crate::utility::Result;
use std::fs;
use std::path::Path;

use crate::keybindings::*;

use sdl2::keyboard::Keycode;

const CFG_FILE_NAME: &str = "config.json";

fn default_window_height() -> u32 {
    720
}

fn default_window_width() -> u32 {
    1280
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawConfig {
    #[serde(default = "default_window_height")]
    window_height: u32,
    #[serde(default = "default_window_width")]
    window_width: u32,
    #[serde(default = "default_key_bindings")]
    pub key_bindings: KeyBindings<i32>,
}

#[derive(Debug)]
pub struct Config {
    window_height: u32,
    window_width: u32,
    pub key_bindings: KeyBindings<Keycode>,
}

impl Config {
    pub fn new(res_path: &Path) -> Result<Config> {        
        let cfg_path = res_path.join(CFG_FILE_NAME);
        let cfg_str = fs::read_to_string(cfg_path.as_path())?;
        let raw_cfg: RawConfig = serde_json::from_str(&cfg_str)?;
        let cfg  = Config {
            window_height: raw_cfg.window_height,
            window_width: raw_cfg.window_width,
            key_bindings: convert_to_keycodes(raw_cfg.key_bindings)
        };
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
