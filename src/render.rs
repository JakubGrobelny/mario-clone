use crate::block::*;
use crate::interface::*;
use crate::level::*;
use crate::player::*;
use crate::resource::*;
use crate::utility::*;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use std::collections::HashMap;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub const SCREEN_WIDTH : u32 = 1280;
pub const SCREEN_HEIGHT : u32 = 720;

pub struct Renderer<'a> {
    pub canvas: Canvas,
    pub texture_creator: TextureCreator<WindowContext>,
    strings: HashMap<String, Surface<'a>>,
}

#[derive(Debug)]
pub struct Camera {
    x: i32,
    y: i32
}

pub enum TextAlignment {
    Center,
    Left,
    Right,
}

pub struct PositionedText<'a> {
    text: &'a str,
    rect: &'a Rect,
    alignment: TextAlignment,
    color: Color
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Renderer, cam: &Camera, res: &ResourceManager);
}

impl PositionedText<'_> {
    pub fn new<'a>(
        text: &'a str,
        rect: &'a Rect,
        alignment: TextAlignment,
        color: Color
    ) -> PositionedText<'a> {
        PositionedText { text, rect, alignment, color }
    }
}

impl Renderer<'_> {
    pub fn new<'a>(canvas: Canvas) -> Renderer<'a> {
        let creator = canvas.texture_creator();
        Renderer {
            canvas,
            texture_creator: creator,
            strings: HashMap::new(),
        }
    }

    pub fn get_text_surface<'a>(
        &'a mut self,
        text: &str,
        font: &Font,
        color: Color,
    ) -> &'a Surface {
        if self.strings.contains_key(text) {
            self.strings.get(text).unwrap()
        } else {
            let surface =
                font.render(text).blended(color).unwrap_or_else(|_| {
                    panic_with_messagebox("Failed to render text!");
                });

            self.strings.insert(String::from(text), surface);
            self.strings.get(text).unwrap()
        }
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(0, 0)
    }
}

impl Camera {
    pub fn new(x: i32, y: i32) -> Camera {
        const CENTER_X : i32 = SCREEN_WIDTH as i32 / 2;
        const CENTER_Y : i32 = SCREEN_HEIGHT as i32 / 2;

        Camera {
            x: CENTER_X - x,
            y: CENTER_Y - y
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn shift(&mut self, amount: (i32, i32)) {
        self.x += amount.0;
        self.y += amount.1;

        if self.x <= 0 {
            self.x = 0;
        }

        if self.y <= 0 {
            self.y = 0;
        }
    }

    pub fn to_camera_coordinates(&self, coords: (i32, i32)) -> (i32, i32) {
        (coords.0 - self.x, coords.1 - self.y)
    }
}

impl Drawable for Button {
    fn draw(&self, renderer: &mut Renderer, _: &Camera, res: &ResourceManager) {
        let button_color = Color::RGB(255, 153, 0);
        renderer.canvas.set_draw_color(button_color);
        renderer
            .canvas
            .fill_rect(*self.rect())
            .expect("Failed to draw a button!");
    }
}

impl Drawable for Player {
    fn draw(&self, renderer: &mut Renderer, _: &Camera, _: &ResourceManager) {
        renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(
            self.position_x(),
            self.position_y(),
            PLAYER_WIDTH as u32,
            PLAYER_HEIGHT as u32,
        );
        renderer
            .canvas
            .fill_rect(rect)
            .expect("Failed to fill a rectangle!");
    }
}

impl Drawable for PositionedText<'_> {
    fn draw(&self, renderer: &mut Renderer, _: &Camera, res: &ResourceManager) {
        // let surface = renderer.get_text_surface(self.text, res.font)
    }
}

pub fn draw_grid(renderer: &mut Renderer, camera: &Camera) {
    renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
    let cols = (camera.x + SCREEN_WIDTH as i32) / BLOCK_SIZE as i32;

    for col in 0..=cols {
        let x = col * BLOCK_SIZE as i32 - camera.x;
        let from = Point::new(x, 0);
        let to = Point::new(x, SCREEN_HEIGHT as i32);
        renderer.canvas.draw_line(from, to).unwrap();
    }

    let rows = (camera.y) / BLOCK_SIZE as i32 + LEVEL_HEIGHT as i32;
    for row in 0..=rows {
        let y = row * BLOCK_SIZE as i32 - camera.y;
        let from = Point::new(0, y);
        let to = Point::new(SCREEN_WIDTH as i32, y);
        renderer.canvas.draw_line(from, to).unwrap();
    }
}
