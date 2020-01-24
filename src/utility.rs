use std::error::Error;
use std::path::PathBuf;

use sdl2::messagebox::*;

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

pub fn panic_with_messagebox(err: &str) -> ! {
    show_simple_message_box(
        sdl2::messagebox::MessageBoxFlag::ERROR,
        "Error!",
        err,
        None,
    )
    .expect("Failed to display an error popup!");
    panic!(err.to_string());
}
