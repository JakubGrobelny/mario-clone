use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::rc::Rc;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::WindowContext;

use serde::{Deserialize, Serialize};

use crate::background::*;
use crate::block::*;
use crate::level::*;
use crate::utility::*;

pub struct ResourceManager<'a> {
    res_path:     PathBuf,
    font:         Font<'a, 'static>,
    textures:     TextureCache<'a, WindowContext>,
    texture_info: TexturePaths,
}

pub type TextureCache<'a, T> =
    ResourceCache<'a, String, Texture<'a>, TextureCreator<T>>;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct TextureInfo {
    pub path:      String,
    #[serde(default = "default_width")]
    pub width:     u32,
    #[serde(default = "default_height")]
    pub height:    u32,
    #[serde(default = "default_animation")]
    pub animation: TextureAnimation,
    #[serde(default = "default_themed")]
    pub themed:    bool,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct TextureAnimation {
    frames: u32,
    speed:  u32,
}

#[derive(Deserialize)]
pub struct TexturePaths {
    blocks:     HashMap<BlockType, TextureInfo>,
    background: HashMap<BackgroundElement, TextureInfo>,
}

pub struct ResourceCache<'a, Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: 'a + ResourceLoader<'a, Resource>,
{
    loader: &'a Loader,
    cache:  HashMap<Key, Rc<Resource>>,
}

pub trait ResourceLoader<'a, Resource> {
    type Args: ?Sized;
    fn load(&'a self, state: &Self::Args) -> Result<Resource>;
}

impl<'a, Key, Resource, Loader> ResourceCache<'a, Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: ResourceLoader<'a, Resource>,
{
    pub fn new(loader: &'a Loader) -> Self {
        ResourceCache {
            cache: HashMap::new(),
            loader,
        }
    }

    pub fn load<Details>(&mut self, details: &Details) -> Result<Rc<Resource>>
    where
        Loader: ResourceLoader<'a, Resource, Args = Details>,
        Details: Eq + Hash + ?Sized,
        Key: Borrow<Details> + for<'b> From<&'b Details>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}

impl<'a, T> ResourceLoader<'a, Texture<'a>> for TextureCreator<T> {
    type Args = str;

    fn load(&'a self, path: &str) -> Result<Texture> {
        let texture = self.load_texture(path)?;
        Ok(texture)
    }
}

impl ResourceManager<'_> {
    pub fn new<'a>(
        cache: TextureCache<'a, WindowContext>,
        ttf: &'a Sdl2TtfContext,
    ) -> Result<ResourceManager<'a>> {
        let res_path = get_base_path()?.join("resources/");

        let font_path = res_path.join("font.ttf");
        let mut font = ttf.load_font(font_path, 128)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        let texture_info =
            fs::read_to_string(res_path.join("textures/info.json"))
                .map_err(|err| err.to_string())
                .and_then(|info_str| {
                    serde_json::from_str(&info_str)
                        .map_err(|err| err.to_string())
                })
                .unwrap_or_else(|err| {
                    panic_with_messagebox!(
                        "Failed to load textures info due to an error in the \
                         JSON file:\n{}",
                        err
                    )
                });

        Ok(ResourceManager {
            res_path,
            font,
            textures: cache,
            texture_info,
        })
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn block_texture_info(&self, block: Block) -> &TextureInfo {
        self.texture_info
            .blocks
            .get(&block.kind())
            .or_else(|| {
                panic_with_messagebox!(
                    "Failed to find texture info for {:?}",
                    block.kind()
                )
            })
            .unwrap()
    }

    pub fn bg_texture_info(&self, bg: BackgroundElement) -> &TextureInfo {
        self.texture_info
            .background
            .get(&bg)
            .or_else(|| {
                panic_with_messagebox!(
                    "Failed to find texture info for {:?}",
                    bg
                )
            })
            .unwrap()
    }

    pub fn texture(&mut self, name: &str) -> Rc<Texture> {
        self.res_path
            .join(format!("textures/{}.png", name))
            .to_str()
            .and_then(|path_str| self.textures.load(path_str).ok())
            .unwrap_or_else(|| {
                panic_with_messagebox!("Failed to load texture {}", name)
            })
    }

    pub fn save_level(&self, name: &str, level: &Level) {
        let serialized = serde_json::to_string(&LevelJSON::from(level))
            .unwrap_or_else(|err| {
                panic_with_messagebox!(
                    "Failed to serialize a level ({})!",
                    err
                );
            });
        let path = self.res_path.join("levels/").join(format!("{}.lvl", name));
        fs::write(path, serialized).unwrap_or_else(|err| {
            panic_with_messagebox!(
                "Failed to write to file '{}' ({})!",
                name,
                err
            )
        });
    }

    pub fn load_level(&self, name: &str) -> Option<Level> {
        let path = self.res_path.join("levels/").join(format!("{}.lvl", name));

        if !path.exists() {
            return None;
        }

        fs::read_to_string(path)
            .map_err(|err| err.to_string())
            .and_then(|contents| {
                serde_json::from_str::<LevelJSON>(&contents)
                    .map_err(|err| err.to_string())
            })
            .map_err(|err| {
                panic_with_messagebox!(
                    "Failed to load level '{}' due to an error in JSON file: \
                     {}\n",
                    name,
                    err
                )
            })
            .map(|lvl| lvl.into())
            .ok()
    }

    pub fn load_listed_levels(&self) -> Vec<(String, Level)> {
        #[derive(Deserialize, Serialize)]
        struct LevelList {
            levels: Vec<String>,
        }

        let levels_path = self.res_path.join("levels/");
        let level_list: LevelList =
            fs::read_to_string(levels_path.join("levels.json"))
                .map_err(|err| err.to_string())
                .and_then(|contents| {
                    serde_json::from_str(&contents)
                        .map_err(|err| err.to_string())
                })
                .unwrap_or_else(|err| {
                    panic_with_messagebox!(
                        "Failed to load listed levels due to an error in JSON \
                         file:\n {}",
                        err
                    )
                });

        level_list
            .levels
            .into_iter()
            .map(|name| {
                let level = self.load_level(&name).unwrap_or_else(|| {
                    panic_with_messagebox!("Level {} does not exist!", name)
                });
                (name, level)
            })
            .collect()
    }
}

impl TextureInfo {
    pub fn frame_index(&self, tick: u32) -> u32 {
        let frames = self.animation.frames;
        let speed = self.animation.speed;
        Frequency::new(frames, speed).phase(tick)
    }

    pub fn variant_index(&self, theme: LevelTheme) -> u32 {
        if self.themed {
            theme as u32
        } else {
            0
        }
    }
}

// for serde_json default values purposes
fn default_themed() -> bool {
    true
}

fn default_height() -> u32 {
    BLOCK_SIZE
}

fn default_width() -> u32 {
    BLOCK_SIZE
}

fn default_animation() -> TextureAnimation {
    TextureAnimation {
        frames: 1,
        speed:  1,
    }
}
