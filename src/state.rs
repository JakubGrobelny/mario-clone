use crate::controller::*;
use crate::editor::*;
use crate::hitbox::*;
use crate::interface::*;
use crate::level::*;
use crate::player::*;
use crate::render::*;
use crate::resource::*;
use crate::utility::*;

use sdl2::keyboard::{Keycode, TextInputUtil};
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::{event::Event, EventPump};

use std::mem::replace;

pub struct GameState<'a> {
    activity: Activity,
    event_pump: EventPump,
    data: SharedGameData<'a>,
}

pub struct SharedGameData<'a> {
    pub should_exit: bool,
    pub controller: Controller,
    pub resources: ResourceManager<'a>,
    pub text_input: TextInput<'a>,
    pub frame: u32,
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
        score: Score,
    },
    Editor(Editor),
    FileInputScreen,
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

    pub fn start(&mut self) {
        self.text = String::new();
        self.util.start();
    }

    pub fn end(&mut self) -> String {
        self.util.stop();
        replace(&mut self.text, String::new())
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn input(&mut self, text: &str) {
        self.text += text;
    }

    pub fn backspace(&mut self) {
        if !self.text().is_empty() {
            self.text.pop();
        }
    }

    pub fn is_active(&self) -> bool {
        self.util.is_active()
    }
}

impl Activity {
    pub fn new_game(resources: &ResourceManager) -> Activity {
        let player = Player::new(200, 200);
        let camera = Camera::new(player.position_x(), player.position_y());
        let score = Score::new();

        Activity::Game {
            player,
            camera,
            score,
        }
    }

    pub fn new_editor(resources: &ResourceManager, name: &str) -> Activity {
        Activity::Editor(Editor::new(resources, name))
    }

    pub fn new_main_menu(resources: &ResourceManager) -> Activity {
        fn exit_button_on_click(state: &mut GameState) {
            state.data.should_exit = true;
        }

        fn start_button_on_click(state: &mut GameState) {
            state.activity = Activity::new_game(state.resources());
        }

        fn editor_button_on_click(state: &mut GameState) {
            state.activity = Activity::FileInputScreen;
        }

        const BUTTON_WIDTH: u32 = 300;
        const BUTTON_HEIGHT: u32 = 90;
        const BUTTON_DISTANCE: u32 = 20;
        const BUTTONS_Y_OFFSET: i32 = 150;

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

impl SharedGameData<'_> {
    pub fn new<'a>(
        resources: ResourceManager<'a>,
        text_input: TextInput<'a>,
    ) -> SharedGameData<'a> {
        SharedGameData {
            should_exit: false,
            controller: Controller::new(),
            resources,
            frame: 0,
            text_input,
        }
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
        let shared_data = SharedGameData::new(resources, text_input);

        Ok(GameState {
            event_pump,
            activity,
            data: shared_data,
        })
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.data.resources
    }

    pub fn frame(&self) -> u32 {
        self.data.frame
    }

    pub fn controller(&self) -> &Controller {
        &self.data.controller
    }

    pub fn should_exit(&self) -> bool {
        self.data.should_exit
    }

    pub fn update(&mut self) {
        if self.data.frame > 1_024 {
            self.data.frame = 0;
        } else {
            self.data.frame += 1;
        }

        let events: Vec<_> = self.event_pump.poll_iter().collect();

        self.data.controller.update(&events);
        self.process_events(&events);

        if self.should_exit() {
            return;
        }

        self.update_activity();
    }

    fn process_events(&mut self, events: &[Event]) {
        for event in events.iter() {
            match event {
                Event::Quit { .. } => {
                    self.data.should_exit = true;
                    break;
                }
                Event::TextInput { text, .. } => {
                    self.data.text_input.input(text);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if self.data.text_input.is_active() {
                        self.data.text_input.backspace();
                    }
                }
                _ => (),
            }
        }
    }

    fn update_activity(&mut self) {
        let mut effects: Option<fn(&mut GameState)> = None;
        match &mut self.activity {
            Activity::Game { player, .. } => {
                player.accelerate(&self.data.controller);
                player.apply_speed();
            }
            activity @ Activity::FileInputScreen => {
                let reading_text = self.data.text_input.is_active();
                if !reading_text {
                    self.data.text_input.start();
                } else if self.data.controller.is_key_pressed(Key::Enter) {
                    let file_name = self.data.text_input.end();
                    std::mem::replace(
                        activity,
                        Activity::new_editor(&self.data.resources, &file_name),
                    );
                }
            }
            Activity::Editor(editor) => {
                editor.update(&mut self.data);
            }
            Activity::MainMenu { buttons } => {
                if self.data.controller.is_key_pressed(Key::Escape) {
                    self.data.should_exit = true;
                }
                let mouse_pos = self.data.controller.mouse().pos();
                if self.data.controller.mouse().is_left_button_pressed() {
                    for button in buttons.iter() {
                        if mouse_pos.collides(button.rect()) {
                            effects = Some(button.effect);
                            break;
                        }
                    }
                }
            }
        }

        if let Some(effect) = effects {
            effect(self);
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.canvas.set_draw_color(Color::RGB(88, 100, 255));
        renderer.canvas.clear();

        match &self.activity {
            Activity::Game { player, camera, .. } => {
                // TODO: remove tests
                let frame = self.frame() % 60;
                {
                    let texture = if frame > 30 {
                        self.data.resources.texture("test1")
                    } else {
                        self.data.resources.texture("test2")
                    };
                    renderer.canvas.copy(&texture, None, None).unwrap();
                }
                player.draw(renderer, &camera, &self.resources());
            }
            Activity::Editor(editor) => {
                editor.draw(renderer, &mut self.data);
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
            Activity::FileInputScreen => {
                renderer.clear(&Color::RGB(0, 0, 0));
                let prompt =
                    TextBuilder::new("Level name (without extension):")
                        .position(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 100)
                        .scale(0.25)
                        .alignment(TextAlignment::TotalCenter)
                        .build();

                let input = self.data.text_input.text();
                let text = TextBuilder::new(&input)
                    .position(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
                    .alignment(TextAlignment::TotalCenter)
                    .scale(0.2)
                    .build();

                prompt.draw(renderer, &Camera::default(), self.resources());
                text.draw(renderer, &Camera::default(), self.resources());
            }
        }

        renderer.canvas.present();
    }
}
