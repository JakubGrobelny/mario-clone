mod block;
mod config;
mod controller;
mod hitbox;
mod level;
mod physics;
mod player;
mod render;
mod resource;
mod state;
mod utility;

use render::*;
use resource::ResourceManager;
use state::*;
use utility::Result;

extern crate sdl2;
use sdl2::pixels::Color;

use std::time::Duration;


const FPS: u32 = 60;

fn main() -> Result<()> {
    let context = sdl2::init()?;
    let video = context.video()?;

    let mut game_state = GameState::new()?;

    let window = video
        .window(
            "mario game",
            game_state.resources().config().window_width(),
            game_state.resources().config().window_height(),
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = context.event_pump()?;
    'running: loop {
        game_state.update(&mut event_pump);
        if game_state.should_exit {
            break 'running;
        }

        game_state.draw(&mut canvas);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}
