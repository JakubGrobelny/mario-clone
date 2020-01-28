use crate::controller::*;
use crate::editor::*;
use crate::game::*;
use crate::menu::*;
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

pub enum Activity {
    Game(Game),
    Editor(Editor),
    FileInputScreen,
    MainMenu(MainMenu),
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
        Activity::Game(Game::new(resources))
    }

    pub fn new_editor(resources: &ResourceManager, name: &str) -> Activity {
        Activity::Editor(Editor::new(resources, name))
    }

    pub fn new_main_menu(resources: &ResourceManager) -> Activity {
        Activity::MainMenu(MainMenu::new(resources))
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
        match &mut self.activity {
            Activity::Game(game) => {
                game.update(&mut self.data);
            }
            activity @ Activity::FileInputScreen => {
                let reading_text = self.data.text_input.is_active();
                if !reading_text {
                    self.data.text_input.start();
                } else if self.data.controller.was_key_pressed(Key::Enter) {
                    let file_name = self.data.text_input.end();
                    replace(
                        activity,
                        Activity::new_editor(&self.data.resources, &file_name),
                    );
                } else if self.data.controller.was_key_pressed(Key::Escape) {
                    self.data.text_input.end();
                    replace(
                        activity,
                        Activity::new_main_menu(&self.data.resources),
                    );
                }
            }
            Activity::Editor(editor) => {
                if editor.update(&mut self.data) {
                    replace(
                        &mut self.activity,
                        Activity::new_main_menu(&self.data.resources),
                    );
                }
            }
            Activity::MainMenu(menu) => {
                let activity = menu.update_and_get_activity(&mut self.data);
                if let Some(activity) = activity {
                    replace(&mut self.activity, activity);
                }
            }
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        match &self.activity {
            Activity::Game(game) => {
                game.draw(renderer, &mut self.data);
            }
            Activity::Editor(editor) => {
                editor.draw(renderer, &mut self.data);
            }
            Activity::MainMenu(menu) => {
                menu.draw(renderer, &mut self.data);
            }
            Activity::FileInputScreen => {
                renderer.clear(Color::RGB(0, 0, 0));
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

                prompt.draw(
                    renderer,
                    &Camera::default(),
                    &mut self.data.resources,
                    self.data.frame,
                );
                text.draw(
                    renderer,
                    &Camera::default(),
                    &mut self.data.resources,
                    self.data.frame,
                );
            }
        }

        renderer.canvas.present();
    }
}
