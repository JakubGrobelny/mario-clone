use crate::controller::*;
use crate::interface::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

pub struct Game {
    player:        Player,
    score:         Score,
    level_list:    Vec<String>,
    current_level: usize,
    camera:        Camera,
    state:         State,
    menu:          ButtonColumn<ButtonEffect>,
}

#[derive(Clone, Copy)]
enum State {
    Paused,
    LevelLoading(u8),
    Running,
}

enum ButtonEffect {
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
    fn new_level_loading_screen() -> State {
        const SECONDS: u8 = 2;
        State::LevelLoading(SECONDS * FPS as u8)
    }
    pub fn new(res: &ResourceManager) -> Game {
        let player = Player::new(0, 0);
        let camera = Camera::new(player.x(), player.y());
        let buttons = ButtonColumnBuilder::new()
            .add(("RESUME", ButtonEffect::Resume))
            .add(("MENU", ButtonEffect::Menu))
            .build();

        let level_list = res.load_level_list();
        if level_list.is_empty() {
            panic_with_messagebox!(
                "Error: no levels specified in the levels.json file!"
            );
        }

        let first_level = res.load_level(&level_list[0]);

        Game {
            current_level: 0,
            player,
            camera,
            score: Score::new(),
            state: Self::new_level_loading_screen(),
            menu: buttons,
            level_list,
        }
    }

    fn update_menu(&mut self, state: &mut SharedState) -> ActivityResult {
        match self.menu.effect_if_clicked(&state.controller) {
            Some(ButtonEffect::Menu) => ActivityResult::Exited,
            Some(ButtonEffect::Resume) => {
                self.state = State::Running;
                state.controller.clear_mouse();
                ActivityResult::Active
            },
            _ => ActivityResult::Active,
        }
    }

    pub fn update(&mut self, state: &mut SharedState) -> ActivityResult {
        if state.controller.was_key_pressed(Key::Escape) {
            self.state = match self.state {
                State::Paused => State::Running,
                State::Running => State::Paused,
                state => state,
            }
        }

        match self.state {
            State::Paused => {
                return self.update_menu(state);
            },
            State::Running => {
                self.player.accelerate(&state.controller);
                self.player.apply_speed();
            },
            State::LevelLoading(0) => {
                self.state = State::Running;
            },
            State::LevelLoading(timer) => {
                self.state = State::LevelLoading(timer - 1);
            },
        }

        ActivityResult::Active
    }

    fn draw_loading_screen(
        &self,
        renderer: &mut Renderer,
        state: &mut SharedState,
    ) {
        renderer.clear(Color::RGB(0, 0, 0));
        let level_text = centered_text!(&self.level_list[self.current_level]);
        // TODO: display the actual coin icon
        let score_str =
            format!("Lives: {} Coins: {}", self.score.lives, self.score.coins);
        let score_text = centered_text!(&score_str);

        renderer
            .draw(&level_text)
            .scale(0.5)
            .position(((SCREEN_WIDTH / 2) as i32, (SCREEN_HEIGHT / 2) as i32))
            .show(&mut state.resources);
        renderer
            .draw(&score_text)
            .scale(0.25)
            .position(((SCREEN_WIDTH / 2) as i32, (SCREEN_HEIGHT / 2) as i32))
            .shift((0, 100))
            .show(&mut state.resources);
    }

    pub fn draw(&self, renderer: &mut Renderer, state: &mut SharedState) {
        // TODO: draw level
        renderer.clear(Color::RGB(88, 100, 255));

        match self.state {
            State::Paused => {
                renderer.fill(Color::RGBA(0, 0, 0, 128));
                renderer.draw(&self.menu).show(&mut state.resources);
            },
            State::LevelLoading(..) => {
                self.draw_loading_screen(renderer, state);
            },
            State::Running => {
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
            },
        }
    }
}
