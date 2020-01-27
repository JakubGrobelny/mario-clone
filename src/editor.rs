use crate::controller::*;
use crate::level::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;
use crate::block::*;

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
        const MOVEMENT_MARGIN: i32 = BLOCK_SIZE as i32 - 1;
        let (x, y) = game_data.controller.mouse().pos();
        let x_movement = if x < MOVEMENT_MARGIN {
            -10
        } else if x > SCREEN_WIDTH as i32 - MOVEMENT_MARGIN {
            10
        } else {
            0
        };

        let y_movement = if y < MOVEMENT_MARGIN {
            -10
        } else if y > SCREEN_HEIGHT as i32 - MOVEMENT_MARGIN {
            10
        } else {
            0
        };

        self.camera.shift((x_movement, y_movement));
    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        draw_grid(renderer, &self.camera);
        // TODO: remove tests
        let (x, y) =
            self.camera.translate_coords(data.controller.mouse().pos());
    }
}
