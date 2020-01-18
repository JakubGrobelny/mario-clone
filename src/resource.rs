use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::utility::Result;


pub struct ResourceManager {
    res_path: PathBuf,
    config: Config,
}

impl ResourceManager {
    pub fn new() -> Result<ResourceManager> {
        let res_path = Path::new("./resources/").canonicalize()?;
        let config = Config::new(&res_path.as_path())?;

        Ok(ResourceManager {
            res_path: res_path,
            config: config,
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn path(&self) -> &Path {
        &self.res_path.as_path()
    }
}
