use crate::controller::*;
use crate::level::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;

use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

pub enum Activity {
    Game {
        current_level: usize,
        levels: Vec<Level>,
        player: Player,
        paused: bool,
        camera: Camera,
    },
    Editor {},
    Menu,
}

pub struct GameState<'a> {
    pub should_exit: bool,
    controller: Controller,
    resources: &'a ResourceManager,
    activity: Activity,
}

impl GameState<'_> {
    pub fn new(resources: &ResourceManager) -> GameState {
        GameState {
            should_exit: false,
            controller: Controller::new(),
            resources: resources,
            activity: Activity::Menu,
        }
    }

    fn process_events(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    self.controller
                        .update(&event, &self.resources.config().key_bindings);
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
    }
}
