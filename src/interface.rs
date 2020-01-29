use crate::controller::*;
use crate::hitbox::*;
use crate::render::*;
use crate::resource::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub const BUTTON_WIDTH: u32 = 300;
pub const BUTTON_HEIGHT: u32 = 90;
pub const BUTTON_DISTANCE: u32 = 20;

pub struct Button<Effect> {
    text:       String,
    rect:       Rect,
    pub effect: Effect,
}

type ButtonInfo<'a, Effect> = (&'a str, Effect);
type ButtonColumnInfo<'a, Effect> = Vec<ButtonInfo<'a, Effect>>;

pub struct ButtonColumn<T> {
    buttons: Vec<Button<T>>,
}

pub struct ButtonColumnBuilder<'a, T> {
    buttons:    ButtonColumnInfo<'a, T>,
    width:      u32,
    height:     u32,
    separation: u32,
    shift:      (i32, i32),
}

impl<T> ButtonColumn<T> {
    pub fn effect_if_clicked(&self, controller: &Controller) -> Option<&T> {
        if !controller.mouse().was_left_button_pressed() {
            return None;
        }

        let mouse_pos = controller.mouse().pos();

        for button in self.buttons.iter() {
            if mouse_pos.collides(button.rect()) {
                return Some(&button.effect);
            }
        }

        None
    }
}

impl<'a, T> ButtonColumnBuilder<'a, T> {
    pub fn new() -> Self {
        ButtonColumnBuilder {
            buttons:    vec![],
            width:      BUTTON_WIDTH,
            height:     BUTTON_HEIGHT,
            separation: BUTTON_DISTANCE,
            shift:      (0, 0),
        }
    }

    pub fn add(mut self, button: ButtonInfo<'a, T>) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn separation(mut self, separation: u32) -> Self {
        self.separation = separation;
        self
    }

    pub fn shift(mut self, shift: (i32, i32)) -> Self {
        self.shift_x(shift.0).shift_y(shift.1)
    }

    pub fn shift_x(mut self, shift: i32) -> Self {
        self.shift.0 += shift;
        self
    }

    pub fn shift_y(mut self, shift: i32) -> Self {
        self.shift.1 += shift;
        self
    }

    pub fn build(self) -> ButtonColumn<T> {
        let num_of_buttons = self.buttons.len() as u32;
        let free_height = SCREEN_HEIGHT
            - self.height * num_of_buttons
            - self.separation * (num_of_buttons - 1);
        let y_offset = free_height as i32 / 2;
        let x = (SCREEN_WIDTH - self.width) as i32 / 2 + self.shift.0;

        let mut buttons: Vec<Button<T>> = vec![];
        for (i, (text, effect)) in self.buttons.into_iter().enumerate() {
            let y = y_offset
                + i as i32 * (self.height + self.separation) as i32
                + self.shift.1;
            buttons.push(Button::new(
                text.to_string(),
                x,
                y,
                self.width,
                self.height,
                effect,
            ));
        }

        ButtonColumn { buttons }
    }
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
        tick: u32,
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

        text.draw(renderer, cam, res, tick);
    }
}

impl<T> Drawable for ButtonColumn<T> {
    fn draw(
        &self,
        renderer: &mut Renderer,
        cam: &Camera,
        res: &mut ResourceManager,
        tick: u32,
    ) {
        for button in self.buttons.iter() {
            button.draw(renderer, cam, res, tick);
        }
    }
}
