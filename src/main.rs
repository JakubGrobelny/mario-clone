extern crate sdl2;
extern crate serde_json;

mod block;
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
use utility::*;
use resource::*;

use sdl2::pixels::Color;

use std::time::{Duration, SystemTime};
use std::thread::sleep;

const FPS: u32 = 60;

fn main() {
    if let Err(err) = run() {
        panic_with_messagebox(&err.to_string());
    }
}

fn run() -> Result<()> {    
    let frame_time : Duration = Duration::from_secs(1) / FPS;
    let context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;
    let video = context.video()?;

    let window = video
        .window(
            "mario game",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(canvas);
    
    let texture_creator = renderer.canvas.texture_creator();
    let texture_cache = TextureCache::new(&texture_creator);    
    let resources = ResourceManager::new(texture_cache, &ttf_context)?;
    
    let video_text_input = video.text_input();
    let text_input = TextInput::new(&video_text_input);

    let mut game_state = GameState::new(resources, &context, text_input)?;

    renderer.clear(&Color::RGB(255, 255, 255));
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