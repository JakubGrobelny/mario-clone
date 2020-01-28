use crate::controller::*;
use crate::interface::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

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
    pub fn new(_res: &ResourceManager) -> MainMenu {
        let on_exit: MainMenuButtonFunc = |data: &mut SharedGameData| {
            data.should_exit = true;
            None
        };

        let on_start: MainMenuButtonFunc = |data: &mut SharedGameData| {
            Some(Activity::new_game(&data.resources))
        };

        let on_editor: MainMenuButtonFunc =
            |_: &mut SharedGameData| Some(Activity::FileInputScreen);

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
        if data.controller.was_key_pressed(Key::Escape) {
            data.should_exit = true;
        }
        self.buttons
            .effect_if_clicked(&data.controller)
            .map(|effect| effect(data))
            .unwrap_or(None)
    }

    pub fn draw(&self, renderer: &mut Renderer, data: &mut SharedGameData) {
        renderer.canvas.set_draw_color(Color::RGB(88, 100, 255));
        renderer.canvas.clear();
        self.buttons.draw(
            renderer,
            &Camera::default(),
            &mut data.resources,
            data.frame,
        );
    }
}
