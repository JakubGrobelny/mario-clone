use crate::block::*;
use crate::controller::*;
use crate::interface::*;
use crate::level::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

use std::mem::replace;

pub struct Editor {
    camera:     Camera,
    level:      Level,
    level_name: String,
    paused:     bool,
    menu:       ButtonColumn<ButtonEffect>,
    selected:   Selection,
}

enum Selection {
    Block(Block),
}

enum ButtonEffect {
    Menu,
    Save,
    Resume,
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
            selected: Selection::Block(Block::from(BlockType::Bricks)),
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

        self.selected = if scroll > 0 {
            match self.selected {
                Selection::Block(block) => Selection::Block(block.next_kind()),
            }
        } else {
            match self.selected {
                Selection::Block(block) => Selection::Block(block.prev_kind()),
            }
        }
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
                    state.controller.clear_mouse();
                },
                ButtonEffect::Save => {
                    state.resources.save_level(&self.level_name, &self.level);
                    self.paused = false;
                    state.controller.clear_mouse();
                },
            }
        }
        ActivityResult::Active
    }

    fn cursor_block(&self, state: &SharedState) -> Option<(usize, usize)> {
        let mouse_pos = state.controller.mouse().pos();
        if !self.camera.on_screen(mouse_pos) {
            return None;
        }

        let (x, y) = self.camera.to_real_coords(mouse_pos);
        let block_x = (x / BLOCK_SIZE as i32) as usize;
        let block_y = (y / BLOCK_SIZE as i32) as usize;
        Some((block_x, block_y))
    }

    fn set_selected(&mut self, pos: (usize, usize)) {
        match self.selected {
            Selection::Block(block) => {
                self.level.set_block(pos, block);
            },
        }
    }

    fn free_selected(&mut self, pos: (usize, usize)) {
        match self.selected {
            Selection::Block(..) => {
                self.level.set_block(pos, Block::default());
            },
        }
    }

    fn copy_pointed(&mut self, pos: (usize, usize)) {
        let pointed = match self.selected {
            Selection::Block(..) => {
                Selection::Block(self.level.get_block(pos))
            },
        };

        self.selected = pointed;
    }

    fn modify_level(&mut self, state: &mut SharedState) {
        if state.controller.mouse().is_left_button_active() {
            if let Some(coords) = self.cursor_block(state) {
                self.set_selected(coords);
            }
        } else if state.controller.mouse().is_right_button_active() {
            if let Some(coords) = self.cursor_block(state) {
                self.free_selected(coords);
            }
        } else if state.controller.mouse().is_middle_button_active() {
            if let Some(coords) = self.cursor_block(state) {
                self.copy_pointed(coords);
            }
        }
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
            self.modify_level(state);
            ActivityResult::Active
        }
    }

    fn draw_selected(&self, renderer: &mut Renderer, state: &mut SharedState) {
        let pos = state.controller.mouse().pos();
        match self.selected {
            Selection::Block(block) => {
                let block = ThemedBlock {
                    block: &block,
                    theme: self.level.theme,
                };

                renderer
                    .draw(&block)
                    .position(pos)
                    .scale(0.75)
                    .tick(state.frame)
                    .show(&mut state.resources);
            },
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
            draw_grid(renderer, self.camera);
            self.draw_selected(renderer, state);
        }
    }
}
