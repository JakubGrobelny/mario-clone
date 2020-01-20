use sdl2::pixels::Color;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub struct Camera {
    x: u32,
    y: u32,
}

pub fn clear_screen(canvas: &mut Canvas, r: u8, g: u8, b: u8) {
    canvas.set_draw_color(Color::RGB(r, g, b));
    canvas.clear();
    canvas.present();
}