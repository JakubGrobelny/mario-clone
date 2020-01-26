use crate::controller::*;
use crate::hitbox::*;
use crate::interface::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use sdl2::keyboard::TextInputUtil;
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::{event::Event, EventPump};

pub struct GameState<'a> {
    pub should_exit: bool,
    controller: Controller,
    resources: ResourceManager<'a>,
    activity: Activity,
    event_pump: EventPump,
    text_input: TextInput<'a>,
    frame: u32,
}

pub struct TextInput<'a> {
    util: &'a TextInputUtil,
    text: String,
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

impl TextInput<'_> {
    pub fn new(util: &TextInputUtil) -> TextInput {
        TextInput {
            util,
            text: String::new(),
        }
    }

    pub fn start(&self) {
        self.util.start();
    }

    pub fn end(&self) -> &str {
        self.util.stop();
        self.text()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn clear(&mut self) {
        self.text = String::new();
    }

    pub fn input(&mut self, text: &String) {
        self.text += text;
    }

    pub fn edit(&mut self, text: &String) {
        self.text += text;
    }

    pub fn is_active(&self) -> bool {
        self.util.is_active()
    }
}

impl Activity {
    pub fn new_game(resources: &ResourceManager) -> Activity {
        let player = Player::new(200, 200);
        let camera = Camera::new(player.position_x(), player.position_y());

        Activity::Game { player, camera }
    }

    pub fn new_editor(resources: &ResourceManager) -> Activity {
        Activity::Editor {
            camera: Camera::default(),
            paused: false,
        }
    }

    pub fn new_main_menu(resources: &ResourceManager) -> Activity {
        fn exit_button_on_click(state: &mut GameState) {
            state.should_exit = true;
        }

        fn start_button_on_click(state: &mut GameState) {
            state.activity = Activity::new_game(state.resources());
            eprintln!("START!");
        }

        fn editor_button_on_click(state: &mut GameState) {
            state.activity = Activity::new_editor(state.resources());
            eprintln!("EDITOR!");
            state.text_input.start();
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
        resources: ResourceManager<'a>,
        context: &Sdl,
        text_input: TextInput<'a>,
    ) -> Result<GameState<'a>> {
        let event_pump = context.event_pump()?;
        let activity = Activity::new_main_menu(&resources);

        Ok(GameState {
            should_exit: false,
            controller: Controller::new(),
            event_pump,
            resources,
            activity,
            frame: 0,
            text_input,
        })
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.resources
    }

    fn process_events(&mut self, events: &[Event]) {
        for event in events.iter() {
            match event {
                Event::Quit { .. } => {
                    self.should_exit = true;
                    break;
                }
                Event::TextInput { text, .. } => {
                    self.text_input.input(text);
                }
                Event::TextEditing { text, .. } => {
                    self.text_input.edit(text);
                }
                _ => (),
            }
        }
    }

    fn update_activity(&mut self) {
        let mut effects: fn(&mut GameState) = |&mut _| {};

        match &mut self.activity {
            Activity::Game { player, .. } => {
                player.accelerate(&self.controller);
                player.apply_speed();
            }
            Activity::Editor { camera, .. } => {
                let x_movement = self.controller.mouse().scroll() * -100;
                let y_movement = if self.controller.is_key_pressed(Key::Up) {
                    -10
                } else if self.controller.is_key_pressed(Key::Down) {
                    10
                } else {
                    0
                };
                camera.shift((x_movement, y_movement));
                if self.text_input.is_active()
                    && self.controller.is_key_pressed(Key::Enter)
                {
                    self.text_input.end();
                }
            }
            Activity::MainMenu { buttons } => {
                let mouse_pos = self.controller.mouse().pos();
                for button in buttons.iter() {
                    if self.controller.is_key_pressed(Key::Escape) {
                        self.should_exit = true;
                    }

                    if mouse_pos.collides(button.rect())
                        && self.controller.mouse().is_left_button_pressed()
                    {
                        effects = button.effect;
                        break;
                    }
                }
            }
        }

        effects(self);
    }

    pub fn update(&mut self) {
        if self.frame > 1_024 {
            self.frame = 0;
        } else {
            self.frame += 1;
        }

        let events: Vec<_> = self.event_pump.poll_iter().collect();

        self.controller.update(&events);
        self.process_events(&events);

        if self.should_exit {
            return;
        }

        self.update_activity();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.canvas.set_draw_color(Color::RGB(88, 100, 255));
        renderer.canvas.clear();

        match &self.activity {
            Activity::Game { player, camera, .. } => {
                // TODO: remove tests
                let frame = self.frame % 60;
                {
                    let texture = if frame > 30 {
                        self.resources.texture("test1")
                    } else {
                        self.resources.texture("test2")
                    };
                    renderer.canvas.copy(&texture, None, None).unwrap();
                }
                player.draw(renderer, &camera, &self.resources());
            }
            Activity::Editor { camera, .. } => {
                draw_grid(renderer, &camera);
                // TODO: remove tests
                let (x, y) =
                    camera.translate_coords(self.controller.mouse().pos());
                if !self.text_input.is_active() {
                    let input = self.text_input.text();
                    let text = PositionedText::new(
                        input,
                        (x, y),
                        TextAlignment::Center,
                        0.25,
                        Color::RGB(255, 255, 255),
                    );
                    text.draw(renderer, camera, &self.resources);
                }
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
