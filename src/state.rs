use crate::controller::*;
use crate::level::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;

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

pub struct GameState {
    should_exit: bool,
    controller: Controller,
    resources: ResourceManager,
    activity: Activity,
}

impl GameState {
    pub fn new(resources: ResourceManager) -> GameState {
        GameState {
            should_exit: false,
            controller: Controller::new(),
            resources: resources,
            activity: Activity::Menu,
        }
    }
}