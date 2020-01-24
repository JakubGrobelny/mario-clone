use crate::state::*;
use crate::render::*;

use sdl2::rect::Rect;

pub struct Button {
    text: String,
    rect: Rect,
    on_click: fn(&mut GameState),
}

pub trait InterfaceElement {
    fn on_click(&self, game_state: &mut GameState);
}

pub fn make_button_column(
    buttons: Vec<(&str, fn(&mut GameState))>,
    width: u32,
    height: u32,
    separation: u32,
    scr_width: u32,
    scr_height: u32,
    x_shift: i32,
    y_shift: i32,
) -> Vec<Button> {
    let num_of_buttons = buttons.len() as u32;
    let free_height = scr_height
        - height * num_of_buttons
        - separation * (num_of_buttons - 1);
    let y_offset = free_height as i32 / 2;
    let x = (scr_width - width) as i32 / 2 + x_shift;

    buttons.into_iter().enumerate().map(|(i, (text, on_click))| {
        let y = y_offset + i as i32 * (height + separation) as i32 + y_shift;
        Button::new(String::from(text), x, y, width, height, on_click)
    }).collect()
}

impl Button {
    pub fn new(
        text: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        on_click: fn(&mut GameState),
    ) -> Button {
        Button {
            text,
            rect: Rect::new(x, y, width, height),
            on_click,
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

}

impl InterfaceElement for Button {
    fn on_click(&self, game_state: &mut GameState) {
        (self.on_click)(game_state);
    }
}
