use crate::block::*;
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
    Restart,
}

pub struct Score {
    lives: u8,
    coins: u8,
}

const BUMP_FALLOFF: u8 = 3;
const BUMP_FORCE: u8 = BUMP_FALLOFF * 8;

impl Game {
    fn new_level_loading_screen() -> State {
        State::LevelLoading(LOADING_SCREEN_TIME)
    }

    fn restart(&mut self, state: &mut SharedState) {
        self.player = Player::default();
        self.player.stick_camera(&mut self.camera);
        self.level = self.level_info.load_level(&state.resources);
        self.score = Score::new();
    }

    pub fn new(res: &ResourceManager) -> Game {
        let player = Player::default();
        let camera = Camera::new(player.rect().x(), player.rect().y());
        let buttons = ButtonColumnBuilder::new()
            .add(("RESUME", ButtonEffect::Resume))
            .add(("RESTART", ButtonEffect::Restart))
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
            Some(ButtonEffect::Restart) => {
                self.restart(state);
                self.state = State::LevelLoading(LOADING_SCREEN_TIME);
                ActivityResult::Active
            },
            None => ActivityResult::Active,
        }
    }

    fn update_player(&mut self, state: &mut SharedState) {
        self.player.accelerate(&state.controller);
        self.player.apply_movement(&mut self.level);
        self.player.stick_camera(&mut self.camera);
    }

    fn handle_bump(&mut self, (x, y): (usize, usize), state: &mut SharedState) {
        let real_block = &mut self.level.blocks[y][x];

        // TODO: kill enemies above

        if real_block.block.is_empty() {
            if self.player.is_big() {
                real_block.block = Block::default();
                // TODO: spawn particles
            }
            return;
        }
        match real_block.block.get_contents() {
            None => (),
            Some(Collectible::Coins(num)) => {
                self.score.coins += 1;
                if self.score.coins == 100 {
                    self.score.coins = 0;
                    self.score.lives += 1;
                }
                // TODO: spawn particle
            },
            Some(Collectible::Mushroom) => {
                // TODO: spawn entity
            },
            Some(Collectible::Star) => {
                // TODO: spawn entity
            },
        }

        real_block.block.delete_item();

        if real_block.block.is_empty() {
            real_block.block.set_kind(BlockType::QuestionMarkEmpty);
        }
    }

    fn update_blocks(&mut self, state: &mut SharedState) {
        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                let new_state = match self.level.blocks[y][x].state {
                    BlockState::Bumped => {
                        self.handle_bump((x, y), state);
                        BlockState::Moving(BUMP_FORCE)
                    },
                    BlockState::Moving(0) => BlockState::Static,
                    BlockState::Moving(n) => {
                        BlockState::Moving(n - BUMP_FALLOFF)
                    },
                    state => state,
                };

                self.level.blocks[y][x].state = new_state;
            }
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
                self.update_player(state);
                self.update_blocks(state);
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

    pub fn draw_ui(&self, renderer: &mut Renderer, state: &mut SharedState) {
        let lives_str = format!("LIVES: {}", self.score.lives);
        let coins_str = format!("COINS: {}", self.score.coins);

        const MARGIN : i32 = 10;

        let lives_text = text!(&lives_str);
        renderer
            .draw(&lives_text)
            .position((MARGIN, MARGIN))
            .scale(0.25)
            .show(&mut state.resources);

        let coins_text = text_right!(&coins_str);
        renderer
            .draw(&coins_text)
            .position((SCREEN_WIDTH as i32 - MARGIN, MARGIN))
            .scale(0.25)
            .show(&mut state.resources);
    }

    pub fn draw(&self, renderer: &mut Renderer, state: &mut SharedState) {
        renderer
            .draw(&self.level)
            .tick(state.frame)
            .camera(self.camera)
            .mode(DrawMode::Game)
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
            State::Running => self.draw_ui(renderer, state),
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
