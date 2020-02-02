use crate::background::*;
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

#[derive(Clone, Copy)]
enum Selection {
    Block(Block),
    Background(BackgroundElement),
    Collectible(Collectible),
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
            selected: Selection::Block(Block::default_visible()),
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

        if scroll > 0 {
            self.selected.set_to_next()
        } else if scroll < 0 {
            self.selected.set_to_prev()
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

    pub fn coords_to_block((x, y): (i32, i32)) -> (usize, usize) {
        let block_x = (x / BLOCK_SIZE as i32) as usize;
        let block_y = (y / BLOCK_SIZE as i32) as usize;
        (block_x, block_y)
    }

    fn cursor_block(&self, state: &SharedState) -> Option<(usize, usize)> {
        let mouse_pos = state.controller.mouse().pos();
        if !self.camera.on_screen(mouse_pos) {
            return None;
        }

        let real_pos = self.camera.to_real_coords(mouse_pos);
        Some(Editor::coords_to_block(real_pos))
    }

    fn set_selected(&mut self, pos: (usize, usize)) {
        match self.selected {
            Selection::Block(block) => {
                self.level.set_block(pos, block);
            },
            Selection::Background(bg) => {
                self.level.set_bg(pos, bg);
            },
            Selection::Collectible(collectible) => {
                if self.level.get_block(pos).is_bumpable() {
                    self.level.fill_block(pos, collectible);
                }
            },
        }
    }

    fn free_selected(&mut self, pos: (usize, usize)) {
        match self.selected {
            Selection::Block(..) => {
                self.level.set_block(pos, Block::default());
            },
            Selection::Background(..) => {
                self.level.set_bg(pos, BackgroundElement::default());
            },
            Selection::Collectible(..) => {
                self.level.remove_block_contents(pos);
            },
        }
    }

    fn copy_pointed(&mut self, pos: (usize, usize)) {
        let pointed = match self.selected {
            Selection::Block(..) => Selection::Block(self.level.get_block(pos)),
            Selection::Background(..) => {
                Selection::Background(self.level.get_bg(pos))
            },
            Selection::Collectible(..) => {
                let contents = self.level.get_block(pos).get_contents();
                if let Some(contents) = contents {
                    Selection::Collectible(contents)
                } else {
                    self.selected
                }
            },
        };

        self.selected = pointed;
    }

    fn modify_level(&mut self, state: &mut SharedState) {
        fn is_proper_input(ctrl: &Controller, button: MButton) -> bool {
            ctrl.was_button_pressed(button)
                || ctrl.is_button_active(button)
                    && ctrl.is_key_active(Key::Ctrl)
        }

        let controller = &state.controller;

        if is_proper_input(controller, MButton::Left) {
            if let Some(coords) = self.cursor_block(state) {
                self.set_selected(coords);
            }
        } else if is_proper_input(controller, MButton::Right) {
            if let Some(coords) = self.cursor_block(state) {
                self.free_selected(coords);
            }
        } else if is_proper_input(controller, MButton::Middle) {
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
            } else if state.controller.was_key_pressed(Key::Tab) {
                self.selected.switch_layer();
            }

            self.swap_selection(state);
            self.modify_level(state);
            ActivityResult::Active
        }
    }

    fn draw_selected(&self, renderer: &mut Renderer, state: &mut SharedState) {
        let pos = state.controller.mouse().pos();
        let call = PartialDrawCall::new()
            .position(pos)
            .scale(0.70)
            .tick(state.frame)
            .mode(DrawMode::EditorSelection);

        match self.selected {
            Selection::Block(block) => {
                let block = ThemedBlock {
                    block,
                    theme: self.level.theme,
                };
                call.draw_with(&block, renderer).show(&mut state.resources);
            },
            Selection::Background(background) => {
                let themed_bg = ThemedBackgroundElement {
                    element: background,
                    theme:   self.level.theme,
                };
                call.draw_with(&themed_bg, renderer)
                    .show(&mut state.resources);
            },
            Selection::Collectible(collectible) => {
                call.draw_with(&collectible, renderer)
                    .show(&mut state.resources);
            },
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, state: &mut SharedState) {
        renderer
            .draw(&self.level)
            .mode(DrawMode::Editor)
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

impl Selection {
    pub fn switch_layer(&mut self) {
        let new = match self {
            Selection::Block(..) => {
                Selection::Background(BackgroundElement::default_visible())
            },
            Selection::Background(..) => {
                Selection::Collectible(Collectible::Coins(1))
            },
            Selection::Collectible(..) => {
                Selection::Block(Block::default_visible())
            },
        };

        *self = new
    }

    pub fn set_to_next(&mut self) {
        *self = self.next()
    }

    pub fn set_to_prev(&mut self) {
        *self = self.prev()
    }

    pub fn next(self) -> Self {
        match self {
            Selection::Block(block) => Selection::Block(block.next_kind()),
            Selection::Background(bg) => Selection::Background(bg.next()),
            Selection::Collectible(c) => Selection::Collectible(c.next()),
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Selection::Block(block) => Selection::Block(block.prev_kind()),
            Selection::Background(bg) => Selection::Background(bg.prev()),
            Selection::Collectible(c) => Selection::Collectible(c.prev()),
        }
    }
}
