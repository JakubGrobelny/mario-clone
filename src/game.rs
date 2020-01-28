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
    score: Score,
}

impl Score {
    pub fn new() -> Score {
        Score { lives: 3, coins: 0 }
    }
}

impl Game {
    pub fn new(_res: &ResourceManager) -> Game {
        let player = Player::new(0, 0);
        let camera = Camera::new(player.position_x(), player.position_y());
        Game {
            player,
            camera,
            score: Score::new(),
        }
    }

    pub fn update(&mut self, game_data: &mut SharedGameData) {
        self.player.accelerate(&game_data.controller);
        self.player.apply_speed();
    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        // TODO: remove tests
        renderer.canvas.set_draw_color(Color::RGB(88, 100, 255));
        renderer.canvas.clear();
        self.player.draw(
            renderer,
            &self.camera,
            &mut data.resources,
            data.frame,
        );
    }
}
