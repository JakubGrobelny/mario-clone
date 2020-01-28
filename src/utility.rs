use std::error::Error;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
    }
}
