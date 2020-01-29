use crate::block::*;
use crate::controller::*;
use crate::interface::*;
use crate::level::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

pub struct Editor {
    camera:     Camera,
    level:      Level,
    level_name: String,
    paused:     bool,
    menu:       ButtonColumn<ButtonEffect>,
    selected:   Option<Block>,
    layer:      EditorLayer,
}

enum ButtonEffect {
    Menu,
    Save,
    Resume,
}

enum EditorLayer {
    Blocks,
    Background,
    Collectibles,
    Enemies,
}

impl Editor {
    pub fn new(resources: &ResourceManager, name: &str) -> Editor {
        let level = resources.load_level(name).unwrap_or_default();
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
            selected: Some(Block::from(BlockType::Bricks)),
            layer: EditorLayer::Blocks,
        }
    }

    fn move_camera(&mut self, state: &mut SharedState) {
        const MOVEMENT_MARGIN: i32 = BLOCK_SIZE as i32 - 1;
        const MOVEMENT_SPEED: i32 = 10;
        let (x, y) = state.controller.mouse().pos();
        let shift_pressed = state.controller.is_key_active(Key::Sprint);
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

    fn swap_selection(&mut self, state: &mut SharedState) {
        let scroll = state.controller.mouse().scroll();
        if scroll == 0 {
            return;
        }

        if self.selected.is_none() {
            self.selected = Some(Block::default());
        }

        let next = if scroll > 0 {
            self.selected.unwrap().next()
        } else {
            self.selected.unwrap().prev()
        };

        self.selected = next;
    }

    fn update_menu(&mut self, state: &mut SharedState) -> ActivityResult {
        let effect = self.menu.effect_if_clicked(&state.controller);
        if let Some(effect) = effect {
            match effect {
                ButtonEffect::Menu => {
                    return ActivityResult::Exited;
                },
                ButtonEffect::Resume => {
                    self.paused = false;
                },
                ButtonEffect::Save => {
                    state.resources.save_level(&self.level_name, &self.level);
                    self.paused = false;
                },
            }
        }
        ActivityResult::Active
    }

    pub fn update(&mut self, state: &mut SharedState) -> ActivityResult {
        if state.controller.was_key_pressed(Key::Escape) {
            self.paused ^= true;
        }

        if self.paused {
            self.update_menu(state)
        } else {
            self.move_camera(state);
            if state.controller.was_key_pressed(Key::Left) {
                self.level.theme = self.level.theme.prev();
            } else if state.controller.was_key_pressed(Key::Right) {
                self.level.theme = self.level.theme.next();
            }

            self.swap_selection(state);
            ActivityResult::Active
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, state: &mut SharedState) {
        renderer
            .draw(&self.level)
            .camera(self.camera)
            .tick(state.frame)
            .show(&mut state.resources);

        if self.paused {
            renderer.fill(Color::RGBA(0, 0, 0, 128));
            renderer
                .draw(&self.menu)
                .tick(state.frame)
                .show(&mut state.resources);
        } else {
            draw_grid(renderer, &self.camera);
            if let Some(selection) = self.selected {
                let pos = state.controller.mouse().pos();
                let block = ThemedBlock {
                    block: &selection,
                    theme: self.level.theme,
                };
                renderer
                    .draw(&block)
                    .position(pos)
                    .scale(0.75)
                    .tick(state.frame)
                    .show(&mut state.resources);
            }
        }
    }
}
