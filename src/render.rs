use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::block::*;
use crate::interface::*;
use crate::level::*;
use crate::player::*;
use crate::resource::*;
use crate::state::*;

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

#[derive(Debug)]
pub struct Camera {
    x: i32,
    y: i32,
    screen_width: u32,
    screen_height: u32,
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas, cam: &Camera, res: &ResourceManager);
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(0, 0, 0, 0)
    }
}

impl Camera {
    pub fn new(x: i32, y: i32, scr_w: u32, scr_h: u32) -> Camera {
        let center_x = scr_w as i32 / 2;
        let center_y = scr_h as i32 / 2;

        Camera {
            x: center_x - x,
            y: center_y - y,
            screen_width: scr_w,
            screen_height: scr_h,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn shift(&mut self, amount: (i32, i32)) {
        self.x += amount.0;
        self.y += amount.1;
    }

    pub fn screen_width(&self) -> u32 {
        self.screen_width
    }

    pub fn screen_height(&self) -> u32 {
        self.screen_height
    }

    pub fn to_camera_coordinates(&self, coords: (i32, i32)) -> (i32, i32) {
        (coords.0 - self.x, coords.1 - self.y)
    }
}

impl Drawable for Button {
    fn draw(&self, canvas: &mut Canvas, _cam: &Camera, res: &ResourceManager) {
        let button_color = Color::RGB(255, 153, 0);
        canvas.set_draw_color(button_color);
        canvas
            .fill_rect(*self.rect())
            .expect("Failed to draw a button!");
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas, _cam: &Camera, _res: &ResourceManager) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(
            self.position_x(),
            self.position_y(),
            PLAYER_WIDTH as u32,
            PLAYER_HEIGHT as u32,
        );
        canvas.fill_rect(rect).expect("Failed to fill a rectangle!");
    }
}

pub fn draw_grid(canvas: &mut Canvas, camera: &Camera) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let cols = (camera.x + camera.screen_width as i32) / BLOCK_SIZE as i32;

    for col in 0..=cols {
        let x = col * BLOCK_SIZE as i32 - camera.x;
        let from = Point::new(x, 0);
        let to = Point::new(x, camera.screen_height as i32);
        canvas.draw_line(from, to).unwrap();
    }

    let rows = (camera.y) / BLOCK_SIZE as i32 + LEVEL_HEIGHT as i32;
    for row in 0..=rows {
        let y = row * BLOCK_SIZE as i32 - camera.y;
        let from = Point::new(0, y);
        let to = Point::new(camera.screen_width as i32, y);
        canvas.draw_line(from, to).unwrap();
    }
}
