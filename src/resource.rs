use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::{Sdl, VideoSubsystem};

use crate::level::*;
use crate::render::*;
use crate::utility::*;

pub type TextureCache<'a, T> =
    ResourceCache<'a, String, Texture<'a>, TextureCreator<T>>;

pub struct ResourceManager<'a> {
    res_path: PathBuf,
    levels: Vec<(String, Level)>,
    font: sdl2::ttf::Font<'a, 'static>,
    textures: TextureCache<'a, WindowContext>,
}

pub struct ResourceCache<'a, Key, Resource, Loader>
where
    Key: Hash + Eq,
    Loader: 'a + ResourceLoader<'a, Resource>,
{
    loader: &'a Loader,
    cache: HashMap<Key, Rc<Resource>>,
}

pub trait ResourceLoader<'a, Resource> {
    type Args: ?Sized;
    fn load(&'a self, data: &Self::Args) -> Result<Resource>;
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
        let levels = load_levels(&res_path.as_path())?;

        const POINT_SIZE: u16 = 128;
        let font_path = res_path.join("font.ttf");
        let mut font = ttf.load_font(font_path, POINT_SIZE)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        Ok(ResourceManager {
            res_path,
            levels,
            font,
            textures: cache,
        })
    }

    pub fn texture(&mut self, name: &str) -> Rc<Texture> {
        self.res_path
            .join(format!("textures/{}.png", name))
            .to_str()
            .and_then(|path_str| self.textures.load(path_str).ok())
            .unwrap_or_else(|| {
                panic_with_messagebox(&format!(
                    "Failed to load texture {}",
                    name
                ));
            })
    }
}
