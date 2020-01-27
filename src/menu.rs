use crate::controller::*;
use crate::hitbox::*;
use crate::interface::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use std::mem::replace;

type MainMenuButtonFunc = fn(&mut SharedGameData) -> Option<Activity>;

pub struct MainMenu {
    buttons: Vec<Button<MainMenuButtonFunc>>,
}

impl<'a> OnClick<&mut SharedGameData<'a>, Option<Activity>>
    for Button<MainMenuButtonFunc>
{
    fn on_click(&self, data: &mut SharedGameData) -> Option<Activity> {
        (self.effect)(data)
    }
}

impl MainMenu {
    pub fn new(resource: &ResourceManager) -> MainMenu {
        let on_exit: MainMenuButtonFunc = |data: &mut SharedGameData| {
            data.should_exit = true;
            None
        };

        let on_start: MainMenuButtonFunc = |data: &mut SharedGameData| {
            Some(Activity::new_game(&data.resources))
        };

        let on_editor: MainMenuButtonFunc =
            |data: &mut SharedGameData| Some(Activity::FileInputScreen);

        const BUTTON_WIDTH: u32 = 300;
        const BUTTON_HEIGHT: u32 = 90;
        const BUTTON_DISTANCE: u32 = 20;
        const BUTTONS_Y_OFFSET: i32 = 150;

        let button_info = vec![
            ("START", on_start),
            ("EDITOR", on_editor),
            ("EXIT", on_exit),
        ];

        let buttons = make_button_column(
            button_info,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            BUTTON_DISTANCE,
            (0, BUTTONS_Y_OFFSET),
        );

        MainMenu { buttons }
    }

    pub fn update_and_get_activity(
        &self,
        data: &mut SharedGameData,
    ) -> Option<Activity> {
        if data.controller.is_key_pressed(Key::Escape) {
            data.should_exit = true;
        }
        let mouse_pos = data.controller.mouse().pos();
        if data.controller.mouse().is_left_button_pressed() {
            for button in self.buttons.iter() {
                if mouse_pos.collides(button.rect()) {
                    return button.on_click(data);
                }
            }
        }
        None
    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        for button in self.buttons.iter() {
            button.draw(renderer, &Camera::default(), &data.resources);
        }
    }
}
