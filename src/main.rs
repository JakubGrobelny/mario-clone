extern crate sdl2;
extern crate serde_json;

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
mod interface;

use state::*;
use render::*;
use utility::Result;

use sdl2::pixels::Color;

use std::time::{Duration, SystemTime};
use std::thread::sleep;

const FPS: u32 = 60;

fn main() -> Result<()> {
    let context = sdl2::init()?;
    let video = context.video()?;
    let frame_time : Duration = Duration::from_secs(1) / FPS;
    let ttf_context = sdl2::ttf::init()?;

    let mut game_state = GameState::new(&context, &ttf_context)?;

    let window = video
        .window(
            "mario game",
            game_state.resources().config().window_width(),
            game_state.resources().config().window_height(),
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(canvas);
    renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
    renderer.canvas.clear();
    renderer.canvas.present();

    'running: loop {
        let now = SystemTime::now();
  
        game_state.update();
        if game_state.should_exit {
            break 'running;
        }

        game_state.draw(&mut renderer);
   
        let elapsed = now.elapsed()?;
        if let Some(time) = frame_time.checked_sub(elapsed) {
            sleep(time);
        }
    }

    Ok(())
}
