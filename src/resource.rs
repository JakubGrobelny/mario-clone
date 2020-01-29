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

use crate::level::*;
use crate::utility::*;

pub type TextureCache<'a, T> =
    ResourceCache<'a, String, Texture<'a>, TextureCreator<T>>;

pub struct ResourceManager<'a> {
    res_path: PathBuf,
    font:     Font<'a, 'static>,
    textures: TextureCache<'a, WindowContext>,
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

        Ok(ResourceManager {
            res_path,
            font,
            textures: cache,
        })
    }

    pub fn font(&self) -> &Font {
        &self.font
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
            .map_err(|err| panic_with_messagebox!("{}", err))
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
                .unwrap_or_else(|err| panic_with_messagebox!("{}", err));

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
