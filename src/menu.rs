use crate::controller::*;
use crate::hitbox::*;
use crate::interface::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use std::mem::replace;

type MainMenuButtonFunc = fn(&mut SharedGameData) -> Option<Activity>;

pub struct MainMenu {
    buttons: ButtonColumn<MainMenuButtonFunc>,
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

        const BUTTONS_Y_OFFSET: i32 = 150;

        let buttons = ButtonColumnBuilder::new()
            .shift_y(BUTTONS_Y_OFFSET)
            .add(("START", on_start))
            .add(("EDITOR", on_editor))
            .add(("EXIT", on_exit))
            .build();

        MainMenu { buttons }
    }

    pub fn update_and_get_activity(
        &self,
        data: &mut SharedGameData,
    ) -> Option<Activity> {
        if data.controller.is_key_active(Key::Escape) {
            data.should_exit = true;
        }
        let mouse_pos = data.controller.mouse().pos();
        self.buttons
            .effect_if_clicked(&data.controller)
            .map(|effect| effect(data))
            .unwrap_or(None)
    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        self.buttons.draw(
            renderer,
            &Camera::default(),
            &mut data.resources,
            data.frame,
        );
    }
}
