use crate::level::*;

use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::utility::{get_base_path, Result};

pub struct ResourceManager {
    res_path: PathBuf,
    config: Config,
    levels: Vec<(String, Level)>,
}

impl ResourceManager {
    pub fn new() -> Result<ResourceManager> {
        let res_path = get_base_path()?.join("resources/");
        let config = Config::new(&res_path.as_path())?;
        let levels = load_levels(&res_path.as_path())?;

        Ok(ResourceManager { res_path, config, levels })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn path(&self) -> &Path {
        &self.res_path.as_path()
    }
}
