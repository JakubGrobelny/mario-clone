use crate::render::*;
use crate::level::*;
use crate::resource::*;
use crate::state::*;
use crate::controller::*;

use vector2d::Vector2D;

pub struct Editor {
    camera: Camera,
    level: Level,
    level_name: String,
    paused: bool,
}

impl Editor {
    pub fn new(resources: &ResourceManager, name: &str) -> Editor {
        let level = resources.load_level(name).unwrap_or_default();
        Editor {
            camera: Camera::default(),
            paused: false,
            level,
            level_name: String::from(name),
        }
    }

    pub fn update(&mut self, game_data: &mut SharedGameData) {
        let x_movement = game_data.controller.mouse().scroll() * -100;
        let y_movement = Vector2D::<i32>::from(&game_data.controller).y * 10;
        self.camera.shift((x_movement, y_movement));
    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        draw_grid(renderer, &self.camera);
        // TODO: remove tests
        let (x, y) =
            self.camera.translate_coords(data.controller.mouse().pos());
    }
}
