use crate::controller::*;
use crate::interface::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

pub struct Game {
    // current_level: usize,
    // levels: Vec<(String, Level)>,
    player: Player,
    paused: bool,
    camera: Camera,
    score:  Score,
    menu:   ButtonColumn<ButtonEffect>,
}

pub enum ButtonEffect {
    Resume,
    Menu,
}

pub struct Score {
    lives: u32,
    coins: u32,
}

impl Score {
    pub fn new() -> Score {
        Score { lives: 3, coins: 0 }
    }
}

impl Game {
    pub fn new(_res: &ResourceManager) -> Game {
        let player = Player::new(0, 0);
        let camera = Camera::new(player.x(), player.y());
        let buttons = ButtonColumnBuilder::new()
            .add(("RESUME", ButtonEffect::Resume))
            .add(("MENU", ButtonEffect::Menu))
            .build();
        Game {
            player,
            camera,
            score: Score::new(),
            paused: false,
            menu: buttons,
        }
    }

    fn update_menu(&mut self, state: &mut SharedState) -> ActivityResult {
        match self.menu.effect_if_clicked(&state.controller) {
            Some(ButtonEffect::Menu) => ActivityResult::Exited,
            Some(ButtonEffect::Resume) => {
                self.paused = false;
                state.controller.clear_mouse();
                ActivityResult::Active
            },
            _ => ActivityResult::Active,
        }
    }

    pub fn update(&mut self, state: &mut SharedState) -> ActivityResult {
        if state.controller.was_key_pressed(Key::Escape) {
            self.paused ^= true;
        }
        if self.paused {
            return self.update_menu(state);
        } else {
            self.player.accelerate(&state.controller);
            self.player.apply_speed();
        }

        ActivityResult::Active
    }

    pub fn draw(&self, renderer: &mut Renderer, state: &mut SharedState) {
        renderer.clear(Color::RGB(0, 0, 0));
        if self.paused {
            renderer.draw(&self.menu).show(&mut state.resources);
        } else {
            let player_rect = rect!(
                self.player.x(),
                self.player.y(),
                PLAYER_WIDTH,
                PLAYER_HEIGHT
            );

            renderer
                .draw(&player_rect)
                .camera(self.camera)
                .show(&mut state.resources);
        }
    }
}
