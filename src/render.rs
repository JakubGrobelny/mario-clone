use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::player::*;
use crate::state::*;

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub struct Camera {
    x: i32,
    y: i32,
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas, camera: &Camera);
}

impl Camera {
    pub fn new(x: i32, y: i32) -> Camera {
        Camera { x, y }
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(
            self.position_x(),
            self.position_y(),
            PLAYER_WIDTH as u32,
            PLAYER_HEIGHT as u32,
        );
        canvas.fill_rect(rect).expect("Failed to fill a rectangle");
    }
}