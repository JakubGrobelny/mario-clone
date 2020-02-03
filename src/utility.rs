use crate::render::*;

use std::error::Error;
use std::path::PathBuf;

use sdl2::rect::Rect;

use vector2d::Vector2D;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum XDirection {
    Left,
    Right,
    Still,
}

pub struct Frequency {
    times: u32,
    phases: u32,
}

impl Frequency {
    pub fn new(phases: u32, times: u32) -> Frequency {
        Frequency { phases, times }
    }

    pub fn phase(&self, tick: u32) -> u32 {
        tick * self.times / FPS % self.phases
    }
}

pub fn get_base_path() -> Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path = path.parent().unwrap().to_path_buf();

    while !path.ends_with(env!("CARGO_PKG_NAME")) {
        if !path.pop() {
            return Err("Couldn't reach base game directory!".into());
        }
    }

    Ok(path)
}

#[macro_export]
macro_rules! panic_with_messagebox {
    ($format:expr $( , $args:expr )* ) => {
        {
            sdl2::messagebox::show_simple_message_box(
                sdl2::messagebox::MessageBoxFlag::ERROR,
                "Error!",
                &format!($format, $( $args ),*),
                None,
            )
            .expect("Failed to display an error popup!");
            panic!($format, $( $args ),*)
        }
    }
}

#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h: expr) => {
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}


#[macro_export]
macro_rules! vec2d {
    ($x:expr, $y:expr) => {
        vector2d::Vector2D { x: $x, y: $y }
    };
}

pub fn approx_eq(v0: Vector2D<f64>, v1: Vector2D<f64>) -> bool {
    vec_map(&v0, |x| x.round()) == vec_map(&v1, |x| x.round())
}

pub fn vec_map<A, B>(vector: &Vector2D<A>, f: fn(A) -> B) -> Vector2D<B>
where
    A: Copy,
{
    vec2d!(f(vector.x), f(vector.y))
}
