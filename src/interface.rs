use crate::render::*;
use crate::state::*;
use crate::resource::*;

use sdl2::rect::Rect;
use sdl2::pixels::Color;

// pub type ButtonCallback =
//     fn(&mut SharedGameData, &mut Activity) -> Option<Activity>;

pub struct Button<Effect> {
    text: String,
    rect: Rect,
    pub effect: Effect,
}

type ButtonsInfo<'a, Effect> = Vec<(&'a str, Effect)>;

pub fn make_button_column<T>(
    buttons: ButtonsInfo<T>,
    width: u32,
    height: u32,
    separation: u32,
    shift: (i32, i32),
) -> Vec<Button<T>> {
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

impl<T> Button<T> {
    pub fn new(
        text: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        effect: T,
    ) -> Button<T> {
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

pub trait OnClick<Arg, Val> {
    fn on_click(&self, arg: Arg) -> Val;
}

impl<T> Drawable for Button<T> {
    fn draw(
        &self,
        renderer: &mut Renderer,
        cam: &Camera,
        res: &mut ResourceManager,
        frame: u32,
    ) {
        let button_color = Color::RGB(255, 153, 0);
        renderer.canvas.set_draw_color(button_color);
        renderer
            .canvas
            .fill_rect(*self.rect())
            .expect("Failed to draw a button!");

        let center = self.rect().center();
        let text = PositionedText::new(
            self.text(),
            (center.x(), center.y()),
            TextAlignment::TotalCenter,
            0.25,
            Color::RGB(255, 255, 255),
        );

        text.draw(renderer, cam, res, frame);
    }
}

