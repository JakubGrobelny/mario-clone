use crate::block::*;
use crate::level::*;
use crate::resource::*;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::video::WindowContext;

use std::fmt::Debug;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub const FPS: u32 = 60;

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

pub struct Renderer {
    pub canvas:          Canvas,
    pub texture_creator: TextureCreator<WindowContext>,
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    x: i32,
    y: i32,
}

pub enum TextAlignment {
    Center,
    Left,
    Right,
    TotalCenter,
}

pub struct DrawCall<'a, T: Drawable> {
    pub object:   &'a T,
    pub tick:     u32,
    pub scale:    f64,
    pub position: (i32, i32),
    pub camera:   Camera,
    pub renderer: &'a mut Renderer,
}

pub struct Text<'a> {
    text:      &'a str,
    alignment: TextAlignment,
    color:     Color,
}

pub struct TextBuilder<'a> {
    text:      &'a str,
    alignment: Option<TextAlignment>,
    color:     Option<Color>,
}

pub trait Drawable: Sized {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager);
}

impl Drawable for Rect {
    fn show(data: DrawCall<Self>, _res: &mut ResourceManager) {
        let (shift_x, shift_y) = data.position;
        let width = (data.object.width() as f64 * data.scale) as f64;
        let height = (data.object.height() as f64 * data.scale) as f64;

        let mut rect = rect![
            data.object.x() + shift_x,
            data.object.y() + shift_y,
            width,
            height
        ];
        data.camera.move_rect(&mut rect);

        data.renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
        data.renderer
            .canvas
            .fill_rect(rect)
            .expect("Failed to draw a rectangle!");
    }
}

impl<'a> TextBuilder<'a> {
    pub fn new(text: &str) -> TextBuilder {
        TextBuilder {
            text,
            alignment: None,
            color: None,
        }
    }

    pub fn alignment(mut self, align: TextAlignment) -> TextBuilder<'a> {
        self.alignment = Some(align);
        self
    }

    pub fn color(mut self, color: Color) -> TextBuilder<'a> {
        self.color = Some(color);
        self
    }

    pub fn build(self) -> Text<'a> {
        Text::new(
            self.text,
            self.alignment.unwrap_or(TextAlignment::Left),
            self.color.unwrap_or(Color::RGB(255, 255, 255)),
        )
    }
}

impl Text<'_> {
    pub fn new(text: &str, alignment: TextAlignment, color: Color) -> Text {
        Text {
            text,
            alignment,
            color,
        }
    }

    fn aligned_rect(
        &self,
        scale: f64,
        pos: (i32, i32),
        texture_width: u32,
        texture_height: u32,
    ) -> Rect {
        let (x0, y0) = pos;
        let width = (f64::from(texture_width) * scale) as i32;
        let height = (f64::from(texture_height) * scale) as i32;
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

    pub fn draw<'a, T: Drawable>(&'a mut self, obj: &'a T) -> DrawCall<T> {
        DrawCall {
            object:   obj,
            renderer: self,
            camera:   Camera::default(),
            tick:     0,
            scale:    1.0,
            position: (0, 0),
        }
    }
}

#[macro_export]
macro_rules! pass_draw {
    ($call:expr, $object:expr) => {
        $call
            .renderer
            .draw($object)
            .camera($call.camera)
            .position($call.position)
            .scale($call.scale)
            .tick($call.tick)
    };
}

#[macro_export]
macro_rules! centered_text {
    ($text:expr) => {
        TextBuilder::new($text)
            .alignment(TextAlignment::TotalCenter)
            .build()
    };
}

#[macro_export]
macro_rules! text {
    ($test:expr) => {
        TextBuilder::new($text).build()
    };
}

#[macro_export]
macro_rules! test_right {
    ($test:expr) => {
        TextBuilder::new($text)
            .alignment(TextAlignment::Right)
            .build()
    };
}

impl<'a, T: Drawable> DrawCall<'a, T> {
    pub fn show(self, res: &mut ResourceManager) {
        T::show(self, res);
    }

    pub fn tick(mut self, tick: u32) -> Self {
        self.tick = tick;
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = camera;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> Self {
        self.position = pos;
        self
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
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

    pub fn move_rect(self, rect: &mut Rect) {
        rect.offset(-self.x, -self.y);
    }

    pub fn translate_coords(self, coords: (i32, i32)) -> (i32, i32) {
        (coords.0 - self.x, coords.1 - self.y)
    }

    pub fn to_real_coords(self, cam_coords: (i32, i32)) -> (i32, i32) {
        (cam_coords.0 + self.x, cam_coords.1 + self.y)
    }

    pub fn on_screen(self, (x, y): (i32, i32)) -> bool {
        x >= 0
            && x <= SCREEN_WIDTH as i32
            && y >= 0
            && y <= SCREEN_HEIGHT as i32
    }

    pub fn in_view(self, rect: Rect) -> bool {
        let cam_rect = Rect::new(
            self.x - 1,
            self.y - 1,
            SCREEN_WIDTH + 1,
            SCREEN_HEIGHT + 1,
        );
        cam_rect.contains_rect(rect) || cam_rect.has_intersection(rect)
    }
}

impl Drawable for Text<'_> {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        if data.object.text.is_empty() {
            return;
        }

        let creator = &data.renderer.texture_creator;
        let texture = res
            .font()
            .render(data.object.text)
            .blended(data.object.color)
            .map_err(|err| err.to_string())
            .and_then(|surface| {
                creator
                    .create_texture_from_surface(surface)
                    .map_err(|err| err.to_string())
            })
            .unwrap_or_else(|err| panic_with_messagebox!("{}", err));

        let TextureQuery { width, height, .. } = texture.query();
        let mut target =
            data.object
                .aligned_rect(data.scale, data.position, width, height);

        data.camera.move_rect(&mut target);
        data.renderer
            .canvas
            .copy(&texture, None, Some(target))
            .unwrap();
    }
}

pub fn draw_grid(renderer: &mut Renderer, camera: Camera) {
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
