use crate::render::*;
use crate::state::*;

use sdl2::rect::Rect;

pub struct Button {
    text: String,
    rect: Rect,
    pub effect: fn(&mut GameState),
}

pub trait InterfaceElement {
    fn on_click(&self, game_state: &mut GameState);
}

type ButtonDescr<'a> = Vec<(&'a str, fn(&mut GameState))>;

pub fn make_button_column(
    buttons: ButtonDescr,
    width: u32,
    height: u32,
    separation: u32,
    shift: (i32, i32),
) -> Vec<Button> {
    let num_of_buttons = buttons.len() as u32;
    let free_height = SCREEN_HEIGHT
        - height * num_of_buttons
        - separation * (num_of_buttons - 1);
    let y_offset = free_height as i32 / 2;
    let x = (SCREEN_WIDTH - width) as i32 / 2 + shift.0;

    buttons
        .into_iter()
        .enumerate()
        .map(|(i, (text, on_click))| {
            let y =
                y_offset + i as i32 * (height + separation) as i32 + shift.1;
            Button::new(String::from(text), x, y, width, height, on_click)
        })
        .collect()
}

impl Button {
    pub fn new(
        text: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        effect: fn(&mut GameState),
    ) -> Button {
        Button {
            text,
            rect: Rect::new(x, y, width, height),
            effect,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}

impl InterfaceElement for Button {
    fn on_click(&self, game_state: &mut GameState) {
        (self.effect)(game_state);
    }
}
