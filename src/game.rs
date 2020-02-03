use crate::controller::*;
use crate::interface::*;
use crate::level::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

pub struct Game {
    camera:     Camera,
    player:     Player,
    score:      Score,
    level_info: LevelInfo,
    level:      PlayableLevel,
    state:      State,
    menu:       ButtonColumn<ButtonEffect>,
}

struct LevelInfo {
    current: usize,
    list:    Vec<String>,
}

#[derive(Clone, Copy)]
enum State {
    Paused,
    LevelLoading(u8),
    Running,
}

const LOADING_SCREEN_TIME: u8 = FPS as u8 * 2;

enum ButtonEffect {
    Resume,
    Menu,
}

pub struct Score {
    lives: u32,
    coins: u32,
}

impl Game {
    fn new_level_loading_screen() -> State {
        State::LevelLoading(LOADING_SCREEN_TIME)
    }

    pub fn new(res: &ResourceManager) -> Game {
        let player = Player::new(10, SCREEN_HEIGHT as i32 - 70);
        let camera = Camera::new(player.rect().x(), player.rect().y());
        let buttons = ButtonColumnBuilder::new()
            .add(("RESUME", ButtonEffect::Resume))
            .add(("MENU", ButtonEffect::Menu))
            .build();

        let level_info = LevelInfo::new(res);
        let level = level_info.load_level(res);

        Game {
            player,
            camera,
            score: Score::new(),
            state: Self::new_level_loading_screen(),
            menu: buttons,
            level_info,
            level,
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
                self.player.apply_movement(&mut self.level);
                self.player.stick_camera(&mut self.camera);
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
        let level_text =
            centered_text!(&self.level_info.list[self.level_info.current]);

        let time = match self.state {
            State::LevelLoading(time) => time,
            _ => {
                panic!(
                    "Drawing loading screen despite being in different state!"
                )
            },
        };
        let progress = ((time as f64 / LOADING_SCREEN_TIME as f64)
            * SCREEN_WIDTH as f64) as u32;
        let progress_bar = rect!(0, 0, SCREEN_WIDTH - progress, 10);
        renderer.draw(&progress_bar).show(&mut state.resources);

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
        renderer
            .draw(&self.level)
            .tick(state.frame)
            .camera(self.camera)
            .show(&mut state.resources);

        renderer
            .draw(&self.player)
            .tick(state.frame)
            .camera(self.camera)
            .show(&mut state.resources);

        match self.state {
            State::Paused => {
                renderer.fill(Color::RGBA(0, 0, 0, 128));
                renderer.draw(&self.menu).show(&mut state.resources);
            },
            State::LevelLoading(..) => {
                self.draw_loading_screen(renderer, state);
            },
            State::Running => {
            },
        }
    }
}

impl Score {
    pub fn new() -> Score {
        Score { lives: 3, coins: 0 }
    }
}

impl LevelInfo {
    pub fn new(res: &ResourceManager) -> LevelInfo {
        let list = res.load_level_list();
        if list.is_empty() {
            panic_with_messagebox!(
                "Error: no levels specified in the levels.json file!"
            );
        }

        LevelInfo { list, current: 0 }
    }

    pub fn load_level(&self, res: &ResourceManager) -> PlayableLevel {
        let prototype = res.load_existing_level(&self.list[self.current]);
        PlayableLevel::from(prototype)
    }
}
