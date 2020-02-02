#![allow(dead_code)]
#![allow(unused)]

extern crate num_traits;
extern crate sdl2;
extern crate serde_json;
#[macro_use]
extern crate num_derive;

#[macro_use]
mod utility;
#[macro_use]
mod render;
mod background;
mod block;
mod controller;
mod editor;
mod enemy;
mod entity;
mod game;
mod hitbox;
mod interface;
mod level;
mod menu;
mod physics;
mod player;
mod resource;
mod state;
mod texture_id;

use render::*;
use resource::*;
use state::*;
use utility::*;

use sdl2::pixels::Color;
use sdl2::render::BlendMode;

use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    if let Err(err) = run() {
        panic_with_messagebox!("{}", err);
    }
}

fn run() -> Result<()> {
    let frame_time: Duration = Duration::from_secs(1) / FPS;
    let context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;
    let video = context.video()?;

    let window = video
        .window("mario game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);

    let mut renderer = Renderer::new(canvas);
    let texture_creator = renderer.canvas.texture_creator();
    let texture_cache = TextureCache::new(&texture_creator);
    let resources = ResourceManager::new(texture_cache, &ttf_context)?;
    let video_text_input = video.text_input();
    let text_input = TextInput::new(&video_text_input);

    let mut game_state = GameState::new(resources, &context, text_input)?;

    renderer.clear(Color::RGB(255, 255, 255));
    renderer.canvas.present();

    'running: loop {
        let now = SystemTime::now();
        game_state.update();
        if game_state.should_exit() {
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
