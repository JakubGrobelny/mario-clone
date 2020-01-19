struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32
}

pub enum Hitbox {
    Single(Rect),
    Compound(Vec<Rect>),
}