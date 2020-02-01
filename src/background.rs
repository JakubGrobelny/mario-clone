use crate::block::*;
use crate::editor::*;
use crate::level::*;
use crate::render::*;
use crate::resource::*;

use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use sdl2::pixels::Color;

#[derive(Copy, Clone)]
#[derive(Deserialize, Serialize, Hash, FromPrimitive)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
#[repr(u8)]
pub enum BackgroundElement {
    Water = 0,
    Fence,
    TreeTopSmall,
    TreeTopBig,
    TreeBottom,
    BigTreeTrunk,
    BigTreeTrunkTop,
    GrassLeft,
    GrassRight,
    GrassMiddle,
    Castle,
    Air,
}

pub struct ThemedBackgroundElement {
    pub element: BackgroundElement,
    pub theme:   LevelTheme,
}

const MAX_BG: u8 = BackgroundElement::Air as u8;

impl Default for BackgroundElement {
    fn default() -> Self {
        Self::Air
    }
}

impl BackgroundElement {
    pub fn default_visible() -> Self {
        Self::Water
    }

    pub fn is_visible(self) -> bool {
        self != BackgroundElement::Air
    }

    pub fn next(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 1) % MAX_BG).unwrap()
    }

    pub fn prev(self) -> Self {
        let id = self as u8;
        let prev_id = if id == 0 { MAX_BG - 1 } else { id - 1 };
        FromPrimitive::from_u8(prev_id).unwrap()
    }
}

impl Drawable for ThemedBackgroundElement {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let bg = data.object.element;
        if !bg.is_visible() {
            return;
        }

        let (src_region, dest, path) = {
            let info = res.bg_texture_info(bg);
            let (x, y) = data.position;
            let width = (info.width as f64 * data.scale) as u32;
            let height = (info.height as f64 * data.scale) as u32;

            if !data.camera.in_view(rect!(x, y, width, height)) {
                return;
            }

            let theme = data.object.theme;

            let sprite_x = (info.frame_index(data.tick) * info.width) as i32;
            let sprite_y = (info.variant_index(theme) * info.height) as i32;

            let src_region = rect!(sprite_x, sprite_y, info.width, info.height);
            let (cam_x, cam_y) = data.camera.translate_coords((x, y));
            let dest = rect!(cam_x, cam_y, width, height);

            if data.mode == DrawMode::EditorSelection {
                let rect = rect!(x, y, info.width, info.height);
                data.renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
                data.renderer.canvas.draw_rect(rect).expect(
                    "Failed to draw selection rectangle in the editor!",
                );
            }

            (src_region, dest, info.path.clone())
        };

        let texture = res.texture(&path);
        data.renderer
            .canvas
            .copy(&texture, src_region, dest)
            .unwrap();
    }
}
