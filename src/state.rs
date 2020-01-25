use crate::controller::*;
use crate::interface::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::Sdl;
use sdl2::{event::Event, EventPump};

pub struct GameState<'a> {
    pub should_exit: bool,
    controller: Controller,
    resources: ResourceManager<'a>,
    activity: Activity,
    event_pump: EventPump,
    frame: u32,
}

pub struct Score {
    lives: u32,
    coins: u32,
}

pub enum Activity {
    Game {
        // current_level: usize,
        // levels: Vec<(String, Level)>,
        player: Player,
        // paused: bool,
        camera: Camera,
        // score: Score,
    },
    Editor {
        camera: Camera,
        // level: Level,
        // level_name: String,
        paused: bool,
    },
    MainMenu {
        buttons: Vec<Button>,
    },
}

impl Activity {
    pub fn new_game(resources: &ResourceManager) -> Activity {
        let player = Player::new(200, 200);
        let camera = Camera::new(
            player.position_x(),
            player.position_y(),
            resources.config().window_width(),
            resources.config().window_height(),
        );

        Activity::Game { player, camera }
    }

    pub fn new_editor(resources: &ResourceManager) -> Activity {
        let scr_h = resources.config().window_height();
        let scr_w = resources.config().window_width();
        let cam_x = scr_w as i32 / 2;
        let cam_y = scr_h as i32 / 2;
        let camera = Camera::new(cam_x, cam_y, scr_w, scr_h);
        Activity::Editor {
            camera,
            paused: false,
        }
    }

    pub fn new_main_menu(resources: &ResourceManager) -> Activity {
        fn exit_button_on_click(state: &mut GameState) {
            state.should_exit = true;
        }

        fn start_button_on_click(state: &mut GameState) {
            state.activity = Activity::new_game(state.resources());
        }

        fn editor_button_on_click(state: &mut GameState) {
            state.activity = Activity::new_editor(state.resources());
        }

        const BUTTON_WIDTH: u32 = 300;
        const BUTTON_HEIGHT: u32 = 120;
        const BUTTON_DISTANCE: u32 = 20;
        const BUTTONS_Y_OFFSET: i32 = 100;

        let button_info = vec![
            ("START", start_button_on_click as fn(&mut GameState)),
            ("EDITOR", editor_button_on_click),
            ("EXIT", exit_button_on_click),
        ];

        let buttons = make_button_column(
            button_info,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            BUTTON_DISTANCE,
            resources.config().window_width(),
            resources.config().window_height(),
            (0, BUTTONS_Y_OFFSET),
        );

        Activity::MainMenu { buttons }
    }
}

impl Score {
    pub fn new() -> Score {
        Score { lives: 3, coins: 0 }
    }
}

impl GameState<'_> {
    pub fn new<'a>(
        context: &'a Sdl,
        ttf: &'a Sdl2TtfContext,
    ) -> Result<GameState<'a>> {
        let resources = ResourceManager::new(ttf)?;
        let event_pump = context.event_pump()?;
        let activity = Activity::new_main_menu(&resources);

        Ok(GameState {
            should_exit: false,
            controller: Controller::new(),
            event_pump,
            resources,
            activity,
            frame: 0,
        })
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.resources
    }

    fn process_events(&mut self, events: &[Event]) {
        for event in events.iter() {
            if let Event::Quit { .. } = event {
                self.should_exit = true;
                break;
            }
        }
    }

    fn update_activity(&mut self) {
        match &mut self.activity {
            Activity::Game { player, .. } => {
                player.accelerate(&self.controller);
                player.apply_speed();
            }
            Activity::Editor { camera, .. } => {
                let scroll = self.controller.mouse().scroll();
                camera.shift((scroll * -10, 0));
            }
            Activity::MainMenu { .. } => {}
        }
    }

    pub fn update(&mut self) {
        if self.frame > 1_024 {
            self.frame = 0;
        }

        let events: Vec<_> = self.event_pump.poll_iter().collect();

        self.controller.update(&events);
        self.process_events(&events);

        if self.controller.is_key_pressed(Key::Escape) {
            self.should_exit = true;
        }

        if self.should_exit {
            return;
        }

        self.update_activity();
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.canvas.set_draw_color(Color::RGB(88, 100, 255));
        renderer.canvas.clear();

        match &self.activity {
            Activity::Game { player, camera, .. } => {
                player.draw(renderer, &camera, &self.resources());
            }
            Activity::Editor { camera, .. } => {
                draw_grid(renderer, &camera);
            }
            Activity::MainMenu { buttons } => {
                for button in buttons {
                    button.draw(
                        renderer,
                        &Camera::default(),
                        &self.resources(),
                    );
                }
            }
        }

        renderer.canvas.present();
    }
}
