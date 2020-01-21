pub struct Hitbox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Hitbox {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Hitbox {
        Hitbox {
            x,
            y,
            width,
            height,
        }
    }
}
