use sdl2::rect::Rect;

pub type Hitbox = Rect;

pub trait CollidableWith<T> {
    fn collides(&self, other: &T) -> bool;
}

impl CollidableWith<(i32, i32)> for Hitbox {
    fn collides(&self, (x, y): &(i32, i32)) -> bool {
        *x >= self.x()
            && *x <= self.x + self.width() as i32
            && *y >= self.y()
            && *y <= self.y + self.height() as i32
    }
}

impl CollidableWith<Hitbox> for (i32, i32) {
    fn collides(&self, hitbox: &Hitbox) -> bool {
        hitbox.collides(self)
    }
}
