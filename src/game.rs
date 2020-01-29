use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

pub struct Score {
    lives: u32,
    coins: u32,
}

pub struct Game {
    // current_level: usize,
    // levels: Vec<(String, Level)>,
    player: Player,
    // paused: bool,
    camera: Camera,
    score:  Score,
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
        Game {
            player,
            camera,
            score: Score::new(),
        }
    }

    pub fn update(&mut self, state: &mut SharedState) {
        self.player.accelerate(&state.controller);
        self.player.apply_speed();
    }

    pub fn draw(&self, renderer: &mut Renderer, state: &mut SharedState) {
        // TODO: remove tests
        renderer.canvas.set_draw_color(Color::RGB(88, 100, 255));
        renderer.canvas.clear();
        renderer
            .draw(&self.player)
            .camera(self.camera)
            .tick(state.frame)
            .show(&mut state.resources);
    }
}
