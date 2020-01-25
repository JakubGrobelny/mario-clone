use crate::level::*;

use std::path::{Path, PathBuf};

use sdl2::ttf::Sdl2TtfContext;

use crate::config::Config;
use crate::utility::{get_base_path, Result};

pub struct ResourceManager<'a> {
    res_path: PathBuf,
    config: Config,
    levels: Vec<(String, Level)>,
    font: sdl2::ttf::Font<'a, 'static>,
}

impl ResourceManager<'_> {
    pub fn new<'a>(ttf: &'a Sdl2TtfContext) -> Result<ResourceManager<'a>> {
        let res_path = get_base_path()?.join("resources/");
        let config = Config::new(&res_path.as_path())?;
        let levels = load_levels(&res_path.as_path())?;
        
        const POINT_SIZE : u16 = 128;
        let font_path = res_path.join("font.ttf");
        let mut font = ttf.load_font(font_path, POINT_SIZE)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        Ok(ResourceManager { res_path, config, levels, font})
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn path(&self) -> &Path {
        &self.res_path.as_path()
    }
}
