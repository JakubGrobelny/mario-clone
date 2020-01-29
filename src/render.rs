use crate::block::*;
use crate::level::*;
use crate::resource::*;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::video::WindowContext;

use std::convert::TryInto;
use std::fmt::Debug;
use std::rc::Rc;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub const FPS: u32 = 60;

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

pub struct Renderer {
    pub canvas:          Canvas,
    pub texture_creator: TextureCreator<WindowContext>,
}

#[derive(Debug)]
pub struct Camera {
    x: i32,
    y: i32,
}

pub struct AnimationFrame<'a> {
    pub texture: Rc<Texture<'a>>,
    pub region:  Rect,
}

pub enum TextAlignment {
    Center,
    Left,
    Right,
    TotalCenter,
}

pub struct DrawCall<'a, T: Drawable> {
    object: &'a T,
    scale: f64,
    position: (i32, i32),
    camera: Option<Camera>,
}

pub struct PositionedText<'a> {
    text:      &'a str,
    position:  (i32, i32),
    alignment: TextAlignment,
    scale:     f64,
    color:     Color,
}

pub struct TextBuilder<'a> {
    text:      &'a str,
    position:  Option<(i32, i32)>,
    alignment: Option<TextAlignment>,
    scale:     Option<f64>,
    color:     Option<Color>,
}

pub trait Drawable {
    fn draw(
        &self,
        canvas: &mut Renderer,
        cam: &Camera,
        res: &mut ResourceManager,
        tick: u32,
    );
}

impl<'a> TextBuilder<'a> {
    pub fn new(text: &str) -> TextBuilder {
        TextBuilder {
            text,
            position: None,
            alignment: None,
            scale: None,
            color: None,
        }
    }

    pub fn position<A, B>(mut self, x: A, y: B) -> TextBuilder<'a>
    where
        A: TryInto<i32>,
        B: TryInto<i32>,
    {
        let x: i32 = x.try_into().unwrap_or(0);
        let y: i32 = y.try_into().unwrap_or(0);
        self.position = Some((x as i32, y as i32));
        self
    }

    pub fn alignment(mut self, align: TextAlignment) -> TextBuilder<'a> {
        self.alignment = Some(align);
        self
    }

    pub fn scale(mut self, scale: f64) -> TextBuilder<'a> {
        self.scale = Some(scale);
        self
    }

    pub fn color(mut self, color: Color) -> TextBuilder<'a> {
        self.color = Some(color);
        self
    }

    pub fn build(self) -> PositionedText<'a> {
        PositionedText::new(
            self.text,
            self.position.unwrap_or((0, 0)),
            self.alignment.unwrap_or(TextAlignment::Left),
            self.scale.unwrap_or(1.0),
            self.color.unwrap_or(Color::RGB(255, 255, 255)),
        )
    }
}

impl PositionedText<'_> {
    pub fn new(
        text: &str,
        position: (i32, i32),
        alignment: TextAlignment,
        scale: f64,
        color: Color,
    ) -> PositionedText {
        PositionedText {
            text,
            position,
            alignment,
            scale,
            color,
        }
    }

    fn aligned_rect(&self, texture_w: u32, texture_h: u32) -> Rect {
        let (x0, y0) = self.position;
        let width = (f64::from(texture_w) * self.scale) as i32;
        let height = (f64::from(texture_h) * self.scale) as i32;
        let (x, y) = match self.alignment {
            TextAlignment::Center => (x0 - width / 2, y0),
            TextAlignment::Left => (x0, y0),
            TextAlignment::Right => (x0 - width, y0),
            TextAlignment::TotalCenter => (x0 - width / 2, y0 - height / 2),
        };

        Rect::new(x, y, width as u32, height as u32)
    }
}

impl Renderer {
    pub fn new(canvas: Canvas) -> Renderer {
        let creator = canvas.texture_creator();
        Renderer {
            canvas,
            texture_creator: creator,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn fill(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(rect!(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT))
            .unwrap();
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(0, 0)
    }
}

impl Camera {
    pub fn new(x: i32, y: i32) -> Camera {
        Camera { x, y }
    }

    pub fn shift(&mut self, amount: (i32, i32)) {
        self.x += amount.0;
        self.y += amount.1;

        const MAX_X: i32 =
            (LEVEL_WIDTH as u32 * BLOCK_SIZE - SCREEN_WIDTH) as i32;

        const MAX_Y: i32 =
            (LEVEL_HEIGHT as u32 * BLOCK_SIZE - SCREEN_HEIGHT) as i32;

        if self.x <= 0 {
            self.x = 0;
        } else if self.x > MAX_X {
            self.x = MAX_X;
        }

        if self.y <= 0 {
            self.y = 0;
        } else if self.y > MAX_Y {
            self.y = MAX_Y;
        }
    }

    pub fn move_rect(&self, rect: &mut Rect) {
        rect.offset(-self.x, -self.y);
    }

    pub fn translate_coords(&self, coords: (i32, i32)) -> (i32, i32) {
        (coords.0 - self.x, coords.1 - self.y)
    }

    pub fn in_view(&self, rect: Rect) -> bool {
        let cam_rect = Rect::new(
            self.x - 1,
            self.y - 1,
            SCREEN_WIDTH + 1,
            SCREEN_HEIGHT + 1,
        );
        cam_rect.contains_rect(rect) || cam_rect.has_intersection(rect)
    }
}

impl Drawable for PositionedText<'_> {
    fn draw(
        &self,
        renderer: &mut Renderer,
        cam: &Camera,
        res: &mut ResourceManager,
        _tick: u32,
    ) {
        if self.text.is_empty() {
            return;
        }

        let creator = &renderer.texture_creator;
        let texture = res
            .font()
            .render(self.text)
            .blended(self.color)
            .map_err(|err| err.to_string())
            .and_then(|surface| {
                creator
                    .create_texture_from_surface(surface)
                    .map_err(|err| err.to_string())
            })
            .unwrap_or_else(|err| panic_with_messagebox!("{}", err));

        let TextureQuery { width, height, .. } = texture.query();
        let mut target = self.aligned_rect(width, height);
        cam.move_rect(&mut target);
        renderer.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}

pub fn draw_grid(renderer: &mut Renderer, camera: &Camera) {
    renderer.canvas.set_draw_color(Color::RGB(50, 50, 50));
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

impl AnimationFrame<'_> {
    pub fn draw(&self, renderer: &mut Renderer, cam: &Camera, pos: (i32, i32)) {
        let (x, y) = cam.translate_coords(pos);
        let dest = rect!(x, y, self.region.width(), self.region.height());
        renderer
            .canvas
            .copy(&self.texture, self.region, dest)
            .unwrap();
    }
}
