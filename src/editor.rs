use crate::block::*;
use crate::controller::*;
use crate::level::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

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

    fn move_camera(&mut self, game_data: &mut SharedGameData) {
        const MOVEMENT_MARGIN: i32 = BLOCK_SIZE as i32 - 1;
        const MOVEMENT_SPEED: i32 = 10;
        let (x, y) = game_data.controller.mouse().pos();
        let shift_pressed = game_data.controller.is_key_pressed(Key::Sprint);
        let accel = if shift_pressed { 3 } else { 1 };

        let x_movement = if x < MOVEMENT_MARGIN {
            -MOVEMENT_SPEED
        } else if x > SCREEN_WIDTH as i32 - MOVEMENT_MARGIN {
            MOVEMENT_SPEED
        } else {
            0
        };

        let y_movement = if y < MOVEMENT_MARGIN {
            -MOVEMENT_SPEED
        } else if y > SCREEN_HEIGHT as i32 - MOVEMENT_MARGIN {
            MOVEMENT_SPEED
        } else {
            0
        };

        self.camera.shift((x_movement * accel, y_movement * accel));
    }

    pub fn update(&mut self, game_data: &mut SharedGameData) {
        self.move_camera(game_data);
        if game_data.controller.is_key_pressed(Key::Left) {

        }

    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        self.level.draw(
            renderer,
            &self.camera,
            &mut data.resources,
            data.frame,
        );
        draw_grid(renderer, &self.camera);
    }
}
