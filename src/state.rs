use crate::controller::*;
use crate::level::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use sdl2::{event::Event, EventPump};
use sdl2::pixels::Color;

pub struct Score {
    lives: u32,
    coins: u32,
}

pub enum Activity {
    Game {
        // current_level: usize,
        // levels: Vec<Level>,
        player: Player,
        paused: bool,
        camera: Camera,
        score: Score,
    },
    Editor {},
    Menu,
}

pub struct GameState {
    pub should_exit: bool,
    controller: Controller,
    resources: ResourceManager,
    activity: Activity,
}

impl GameState {
    pub fn new() -> Result<GameState> {
        let resources = ResourceManager::new()?;
        let player = Player::new(
            resources.config().window_width() as i32 / 2,
            resources.config().window_height() as i32 / 2,
        );
        let camera = Camera::new(player.position_x(), player.position_y());
        Ok(GameState {
            should_exit: false,
            controller: Controller::new(),
            resources,
            activity: Activity::Game {
                player,
                camera,
                paused: false,
                score: Score::new(),
            },
        })
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.resources
    }

    fn process_events(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    self.controller.update(&event);
                }
                Event::Quit { .. } => {
                    self.should_exit = true;
                    break;
                }
                _ => (),
            }
        }
    }

    pub fn update(&mut self, event_pump: &mut EventPump) {
        self.process_events(event_pump);

        if self.controller.active(Key::Escape) {
            self.should_exit = true;
        }

        eprintln!("{:?}", self.controller);

        if self.should_exit {
            return;
        }

        if let Activity::Game { player, .. } = &mut self.activity {
            player.accelerate(&self.controller);
            player.apply_speed();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        if let Activity::Game {
            player,
            camera,
            ..
        } = &self.activity
        {
            player.draw(canvas, &camera);
        }
        canvas.present();
    }
}

impl Score {
    pub fn new() -> Score {
        Score { lives: 3, coins: 0 }
    }
}
