use crate::block::*;
use crate::controller::*;
use crate::interface::*;
use crate::level::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use vector2d::Vector2D;

use sdl2::pixels::Color;

pub struct Editor {
    camera: Camera,
    level: Level,
    level_name: String,
    paused: bool,
    menu: ButtonColumn<ButtonEffect>,
}

enum ButtonEffect {
    Menu,
    Save,
    Resume,
}

impl Editor {
    pub fn new(resources: &ResourceManager, name: &str) -> Editor {
        let level = resources.load_level(name).unwrap_or_default();
        let button_data = vec![
            ("RESUME", ButtonEffect::Resume),
            ("SAVE", ButtonEffect::Save),
            ("MENU", ButtonEffect::Menu),
        ];

        let buttons = ButtonColumnBuilder::new()
            .add(("RESUME", ButtonEffect::Resume))
            .add(("SAVE", ButtonEffect::Save))
            .add(("MENU", ButtonEffect::Menu))
            .build();

        Editor {
            camera: Camera::default(),
            paused: false,
            level,
            level_name: String::from(name),
            menu: buttons,
        }
    }

    fn move_camera(&mut self, game_data: &mut SharedGameData) {
        const MOVEMENT_MARGIN: i32 = BLOCK_SIZE as i32 - 1;
        const MOVEMENT_SPEED: i32 = 10;
        let (x, y) = game_data.controller.mouse().pos();
        let shift_pressed = game_data.controller.is_key_active(Key::Sprint);
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

    pub fn update(&mut self, game_data: &mut SharedGameData) -> bool {
        if game_data.controller.was_key_pressed(Key::Escape) {
            self.paused ^= true;
        }

        if self.paused {
            if let Some(effect) =
                self.menu.effect_if_clicked(&game_data.controller)
            {
                match effect {
                    ButtonEffect::Menu => {
                        return true;
                    }
                    ButtonEffect::Resume => {
                        self.paused = false;
                    }
                    ButtonEffect::Save => {
                        game_data
                            .resources
                            .save_level(&self.level_name, &self.level);
                        self.paused = false;
                    }
                }
            }
        } else {
            self.move_camera(game_data);
            if game_data.controller.was_key_pressed(Key::Left) {
                self.level.theme = self.level.theme.prev();
            } else if game_data.controller.was_key_pressed(Key::Right) {
                self.level.theme = self.level.theme.next();
            }
        }
        false
    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        self.level.draw(
            renderer,
            &self.camera,
            &mut data.resources,
            data.frame,
        );
        draw_grid(renderer, &self.camera);
        if self.paused {
            renderer.fill(Color::RGBA(0, 0, 0, 128));
            self.menu.draw(
                renderer,
                &Camera::default(),
                &mut data.resources,
                data.frame,
            );
        }
    }
}
