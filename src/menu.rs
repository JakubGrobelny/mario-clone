use crate::controller::*;
use crate::interface::*;
use crate::render::*;
use crate::resource::*;
use crate::state::*;

use sdl2::pixels::Color;

type MainMenuButtonFunc = fn(&mut SharedState) -> Option<Activity>;

pub struct MainMenu {
    buttons: ButtonColumn<MainMenuButtonFunc>,
}

impl<'a> OnClick<&mut SharedState<'a>, Option<Activity>>
    for Button<MainMenuButtonFunc>
{
    fn on_click(&self, state: &mut SharedState) -> Option<Activity> {
        (self.effect)(state)
    }
}

impl MainMenu {
    pub fn new(_res: &ResourceManager) -> MainMenu {
        let on_exit: MainMenuButtonFunc = |state: &mut SharedState| {
            state.should_exit = true;
            None
        };

        let on_start: MainMenuButtonFunc = |state: &mut SharedState| {
            Some(Activity::new_game(&state.resources))
        };

        let on_editor: MainMenuButtonFunc =
            |_: &mut SharedState| Some(Activity::FileInputScreen);

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
        state: &mut SharedState,
    ) -> Option<Activity> {
        if state.controller.was_key_pressed(Key::Escape) {
            state.should_exit = true;
        }
        self.buttons
            .effect_if_clicked(&state.controller)
            .map(|effect| effect(state))
            .unwrap_or(None)
    }

    pub fn draw(&self, renderer: &mut Renderer, state: &mut SharedState) {
        renderer.canvas.set_draw_color(Color::RGB(88, 100, 255));
        renderer.canvas.clear();
        self.buttons.draw(
            renderer,
            &Camera::default(),
            &mut state.resources,
            state.frame,
        );
    }
}
