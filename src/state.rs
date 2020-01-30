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
    activity:   Activity,
    event_pump: EventPump,
    state:      SharedState<'a>,
}

pub struct SharedState<'a> {
    pub should_exit: bool,
    pub controller:  Controller,
    pub resources:   ResourceManager<'a>,
    pub text_input:  TextInput<'a>,
    pub frame:       u32,
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

#[derive(PartialEq, Eq)]
pub enum ActivityResult {
    Exited,
    Active,
}

impl ActivityResult {
    pub fn exited(self) -> bool {
        self == ActivityResult::Exited
    }
}

impl TextInput<'_> {
    pub fn new(util: &TextInputUtil) -> TextInput {
        util.stop();
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

impl SharedState<'_> {
    pub fn new<'a>(
        resources: ResourceManager<'a>,
        text_input: TextInput<'a>,
    ) -> SharedState<'a> {
        dbg!(text_input.is_active());
        SharedState {
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
        let shared_state = SharedState::new(resources, text_input);

        Ok(GameState {
            event_pump,
            activity,
            state: shared_state,
        })
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.state.resources
    }

    pub fn frame(&self) -> u32 {
        self.state.frame
    }

    pub fn controller(&self) -> &Controller {
        &self.state.controller
    }

    pub fn should_exit(&self) -> bool {
        self.state.should_exit
    }

    pub fn update_tick(&mut self) {
        const SECONDS_UNTIL_RESET : u32 = 10;
        if self.state.frame >= FPS * SECONDS_UNTIL_RESET {
            self.state.frame = 0;
        } else {
            self.state.frame += 1;
        }
    }

    pub fn update(&mut self) {
        self.update_tick();
        let events: Vec<_> = self.event_pump.poll_iter().collect();

        self.state.controller.update(&events);
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
                    self.state.should_exit = true;
                    break;
                },
                Event::TextInput { text, .. } => {
                    self.state.text_input.input(text);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if self.state.text_input.is_active() {
                        self.state.text_input.backspace();
                    }
                },
                _ => (),
            }
        }
    }

    fn update_activity(&mut self) {
        match &mut self.activity {
            Activity::Game(game) => {
                game.update(&mut self.state);
            },
            activity @ Activity::FileInputScreen => {
                let reading_text = self.state.text_input.is_active();
                if !reading_text {
                    self.state.text_input.start();
                } else if self.state.controller.was_key_pressed(Key::Enter) {
                    let file_name = self.state.text_input.end();
                    replace(
                        activity,
                        Activity::new_editor(&self.state.resources, &file_name),
                    );
                } else if self.state.controller.was_key_pressed(Key::Escape) {
                    self.state.text_input.end();
                    replace(
                        activity,
                        Activity::new_main_menu(&self.state.resources),
                    );
                }
            },
            Activity::Editor(editor) => {
                if editor.update(&mut self.state).exited() {
                    replace(
                        &mut self.activity,
                        Activity::new_main_menu(&self.state.resources),
                    );
                }
            },
            Activity::MainMenu(menu) => {
                let activity = menu.update_and_get_activity(&mut self.state);
                if let Some(activity) = activity {
                    replace(&mut self.activity, activity);
                }
            },
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        match &self.activity {
            Activity::Game(game) => {
                game.draw(renderer, &mut self.state);
            },
            Activity::Editor(editor) => {
                editor.draw(renderer, &mut self.state);
            },
            Activity::MainMenu(menu) => {
                menu.draw(renderer, &mut self.state);
            },
            Activity::FileInputScreen => {
                renderer.clear(Color::RGB(0, 0, 0));
                let prompt = centered_text!("Level name: ");
                let input = centered_text!(self.state.text_input.text());

                renderer
                    .draw(&prompt)
                    .position((
                        SCREEN_WIDTH as i32 / 2,
                        SCREEN_HEIGHT as i32 / 2 - 100,
                    ))
                    .scale(0.2)
                    .show(&mut self.state.resources);

                renderer
                    .draw(&input)
                    .position((
                        SCREEN_WIDTH as i32 / 2,
                        SCREEN_HEIGHT as i32 / 2,
                    ))
                    .scale(0.25)
                    .show(&mut self.state.resources);
            },
        }

        renderer.canvas.present();
    }
}
