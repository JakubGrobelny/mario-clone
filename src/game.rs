use crate::block::*;
use crate::controller::*;
use crate::enemy::*;
use crate::entity::*;
use crate::hitbox::*;
use crate::interface::*;
use crate::level::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;
use crate::utility::*;

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
    GameFinished,
    GameOver,
}

const LOADING_SCREEN_TIME: u8 = FPS as u8 * 2;

enum ButtonEffect {
    Resume,
    Menu,
    Restart,
}

#[derive(Clone, Copy)]
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

    fn next_level(&mut self, state: &mut SharedState) {
        let variant = self.player.variant;
        self.player = Player::default();
        self.player.variant = variant;
        self.player.stick_camera(&mut self.camera);

        match self.level_info.next_level(&state.resources) {
            Some(level) => self.level = level,
            None => self.state = State::GameFinished,
        }
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
        if self.player.invincibility > 0 {
            self.player.invincibility -= 1;
        }
    }

    fn bump_entities(
        &mut self,
        (x, y): (usize, usize),
        state: &mut SharedState,
    ) {
        let entities_num = self.level.entities.len();
        let pos = (
            (x * BLOCK_SIZE as usize) as i32,
            ((y - 1) * BLOCK_SIZE as usize) as i32,
        );
        let bump_hitbox = rect!((pos.0), (pos.1), BLOCK_SIZE, BLOCK_SIZE);
        for i in 0..entities_num {
            if self.level.entities[i].body.hitbox.collides(&bump_hitbox) {
                match self.level.entities[i].kind {
                    EntityType::Enemy(..) => {
                        // TODO: add particle
                        self.level.entities[i] = Entity::dead();
                    },
                    EntityType::Collectible(Collectible::Coins(..)) => (),
                    EntityType::Collectible(..) => {
                        let mut body = self.level.entities[i].body;
                        body.accelerate(vec2d!(0.0, -10.0));
                        self.level.entities[i].body = body;
                    },
                    _ => (),
                }
            }
        }
    }

    fn handle_bump(&mut self, (x, y): (usize, usize), state: &mut SharedState) {
        self.bump_entities((x, y), state);
        let real_block = &mut self.level.blocks[y][x];

        if real_block.block.is_empty() {
            if self.player.is_big() {
                real_block.spawn_particles(
                    self.level.prototype.theme,
                    (x, y),
                    &mut self.level.entities,
                );
                real_block.block = Block::default();
            }
            return;
        }

        match real_block.block.get_contents() {
            Some(Collectible::Coins(num)) => {
                self.score.coins += 1;
                if self.score.coins == 100 {
                    self.score.coins = 0;
                    self.score.lives += 1;
                }

                let coin = Entity::spawn_coin((x, y - 1));
                self.level.entities.push(coin);
            },
            Some(Collectible::Mushroom) => {
                let entity = Entity::spawn(
                    EntityType::Collectible(Collectible::Mushroom),
                    (x, y - 1),
                );

                self.level.entities.push(entity);
            },
            Some(Collectible::Star) => {
                let entity = Entity::spawn(
                    EntityType::Collectible(Collectible::Star),
                    (x, y - 1),
                );

                self.level.entities.push(entity);
            },
            _ => (),
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

    fn too_far(player: &Player, entity: &Entity) -> bool {
        const MARGIN: i32 = BLOCK_SIZE as i32 * 5 + SCREEN_WIDTH as i32 / 2;
        let (player_x, _) = player.position();
        let (entity_x, _) = entity.body.position();
        (entity_x - player_x).abs() > MARGIN
    }

    fn update_entities(&mut self, state: &mut SharedState) {
        let len = self.level.entities.len();
        for i in 0..len {
            if Self::too_far(&self.player, &self.level.entities[i]) {
                continue;
            }

            match self.level.entities[i].kind {
                EntityType::Particle(particle) => {
                    particle.update(i, &mut self.level);
                },
                EntityType::Collectible(Collectible::Flower) => {
                    unimplemented!();
                },
                EntityType::Collectible(Collectible::Mushroom) => {
                    let mut body = self.level.entities[i].body;
                    if body.hitbox.collides(&self.player.body.hitbox) {
                        if self.player.is_big() {
                            self.score.lives += 1;
                        } else {
                            self.player.grow();
                        }
                        self.level.entities[i] = Entity::dead();
                        continue;
                    }
                    body.accelerate_or_bounce(MUSHROOM_ACCEL, &self.level);
                    body.apply_movement(&mut self.level, false);
                    self.level.entities[i].body = body;
                },
                EntityType::Collectible(Collectible::Star) => {
                    let mut body = self.level.entities[i].body;
                    if body.hitbox.collides(&self.player.body.hitbox) {
                        self.player.invincibility = INVINCIBILITY_TIME;
                        self.level.entities[i] = Entity::dead();
                        continue;
                    }

                    let accel = if body.grounded {
                        vec2d!(STAR_ACCEL, STAR_JUMP)
                    } else {
                        vec2d!(STAR_ACCEL, 0.0)
                    };

                    body.accelerate(accel);
                    body.accelerate_or_bounce(0.0, &self.level);

                    body.apply_movement(&mut self.level, false);
                    self.level.entities[i].body = body;
                },
                EntityType::Enemy(EnemyType::Goomba) => {
                    let mut body = self.level.entities[i].body;
                    if body.hitbox.collides(&self.player.body.hitbox) {
                        if self.player.body.speed_y() > 0.0
                            || self.player.invincibility > 0
                        {
                            self.level.entities[i] = Entity::dead();
                            self.player.body.accelerate(vec2d!(
                                0.0,
                                ENEMY_KILL_BOUNCE - self.player.body.speed_y()
                            ));
                            // TODO: spawn particle
                            continue;
                        } else {
                            // TODO: display death animation
                            if self.score.lives <= 1 {
                                self.state = State::GameOver;
                                return;
                            } else {
                                let prev_score = self.score;
                                self.restart(state);
                                self.score = prev_score;
                                self.score.lives -= 1;
                                self.state =
                                    State::LevelLoading(LOADING_SCREEN_TIME);
                                return;
                            }
                        }
                    }

                    body.accelerate_or_bounce(GOOMBA_ACCELERATION, &self.level);
                    body.apply_movement(&mut self.level, false);
                    self.level.entities[i].body = body;
                },
                EntityType::EndFlag => {
                    let hitbox = self.level.entities[i].body.hitbox;
                    if hitbox.collides(&self.player.body.hitbox) {
                        self.state = State::LevelLoading(LOADING_SCREEN_TIME);
                        self.next_level(state);
                        return;
                    }
                },
                _ => (),
            }
        }

        self.level
            .entities
            .retain(|entity| !entity.body.out_of_bounds() && !entity.is_dead())
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
                self.update_entities(state);
            },
            State::LevelLoading(0) => {
                self.state = State::Running;
            },
            State::LevelLoading(timer) => {
                self.state = State::LevelLoading(timer - 1);
            },
            State::GameFinished | State::GameOver => {
                if state.controller.was_key_pressed(Key::Enter) {
                    return ActivityResult::Exited;
                }
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

        const MARGIN: i32 = 10;

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
            State::GameFinished => {
                renderer.clear(Color::RGB(0, 0, 0));
                let text =
                    centered_text!("CONGRATULATIONS, YOU FINISHED THE GAME!");
                renderer
                    .draw(&text)
                    .position((
                        SCREEN_WIDTH as i32 / 2,
                        SCREEN_HEIGHT as i32 / 2,
                    ))
                    .scale(0.2)
                    .show(&mut state.resources);
            },
            State::GameOver => {
                renderer.clear(Color::RGB(0, 0, 0));
                let text = centered_text!("GAME OVER");
                renderer
                    .draw(&text)
                    .position((
                        SCREEN_WIDTH as i32 / 2,
                        SCREEN_HEIGHT as i32 / 2,
                    ))
                    .show(&mut state.resources);
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

    pub fn next_level(
        &mut self,
        res: &ResourceManager,
    ) -> Option<PlayableLevel> {
        self.current += 1;
        if self.current >= self.list.len() {
            None
        } else {
            Some(self.load_level(res))
        }
    }

    pub fn load_level(&self, res: &ResourceManager) -> PlayableLevel {
        let prototype = res.load_existing_level(&self.list[self.current]);
        PlayableLevel::from(prototype)
    }
}
