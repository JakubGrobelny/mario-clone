use crate::controller::*;
use crate::level::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use sdl2::pixels::Color;
use sdl2::{event::Event, EventPump};

pub struct Score {
    lives: u32,
    coins: u32,
}

pub enum Activity {
    Game {
        current_level: usize,
        levels: Vec<(String, Level)>,
        player: Player,
        paused: bool,
        camera: Camera,
        score: Score,
    },
    Editor {
        camera: Camera,
        // level: Level,
        // level_name: String,
        paused: bool,
    },
    Menu {},
}

impl Activity {
    // pub fn new_game(resources: &ResourceManager) -> Activity {
    // }

    pub fn new_editor(resources: &ResourceManager) -> Activity {
        let scr_h = resources.config().window_height();
        let scr_w = resources.config().window_width();
        let cam_x = scr_w as i32 / 2;
        let cam_y = scr_h as i32 / 2;
        let camera = Camera::new(cam_x, cam_y, scr_w, scr_h);
        Activity::Editor {
            camera,
            paused: false,
        }
    }
}

pub struct GameState {
    pub should_exit: bool,
    controller: Controller,
    resources: ResourceManager,
    activity: Activity,
    frame: u32,
}

impl GameState {
    pub fn new() -> Result<GameState> {
        let resources = ResourceManager::new()?;
        let activity = Activity::new_editor(&resources);

        Ok(GameState {
            should_exit: false,
            controller: Controller::new(),
            resources,
            activity,
            frame: 0,
        })
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.resources
    }

    fn process_events(&mut self, events: &[Event]) {
        for event in events.iter() {
            if let Event::Quit { .. } = event {
                self.should_exit = true;
                break;
            }
        }
    }

    pub fn update(&mut self, event_pump: &mut EventPump) {
        if self.frame > 1_024 {
            self.frame = 0;
        }

        let events: Vec<_> = event_pump.poll_iter().collect();

        self.controller.update(&events);
        self.process_events(&events);

        if self.controller.is_key_pressed(Key::Escape) {
            self.should_exit = true;
        }

        if self.should_exit {
            return;
        }

        eprintln!("{:?}", self.controller);

        match &mut self.activity {
            Activity::Game { player, .. } => {
                player.accelerate(&self.controller);
                player.apply_speed();
            }
            Activity::Editor { camera, .. } => {
                let scroll = self.controller.mouse().scroll();
                camera.shift((scroll * -10, 0));
            }
            Activity::Menu { .. } => {}
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(Color::RGB(88, 100, 255));
        canvas.clear();

        match &self.activity {
            Activity::Game { player, camera, .. } => {
                player.draw(canvas, &camera);
            }
            Activity::Editor { camera, .. } => {
                draw_grid(canvas, &camera);
            }
            Activity::Menu { .. } => {}
        }

        canvas.present();
    }
}

impl Score {
    pub fn new() -> Score {
        Score { lives: 3, coins: 0 }
    }
}
