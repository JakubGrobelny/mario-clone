mod config;
mod resource;
mod utility;
mod keybindings;
mod controller;
mod state;
mod level;
mod block;
mod player;
mod render;
mod physics;
mod hitbox;

use resource::ResourceManager;
use controller::*;
use utility::Result;
use state::*;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;

const FPS: u32 = 60;

fn main() -> Result<()> {
    let context = sdl2::init()?;
    let video = context.video()?;

    let resources = ResourceManager::new()?;
    let game_state = GameState::new(&resources);

    let window = video
        .window(
            "mario game",
            resources.config().window_width(),
            resources.config().window_height(),
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut controller = Controller::new();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    
    let mut event_pump = context.event_pump()?;
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => (),
            }
        }

        canvas.clear();
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}
