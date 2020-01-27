use sdl2::rect::{Point, Rect};

pub type Hitbox = Rect;

pub trait CollidableWith<T> {
    fn collides(&self, other: &T) -> bool;
}

impl CollidableWith<(i32, i32)> for Hitbox {
    fn collides(&self, (x, y): &(i32, i32)) -> bool {
        self.contains_point(Point::new(*x, *y))
    }
}

impl CollidableWith<Hitbox> for (i32, i32) {
    fn collides(&self, hitbox: &Hitbox) -> bool {
        hitbox.collides(self)
    }
}
