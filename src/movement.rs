use crate::block::*;
use crate::hitbox::*;
use crate::level::*;
use crate::physics::*;
use crate::utility::*;

use vector2d::Vector2D;

const CHECK_DISTANCE: usize = 6;

impl PhysicalBody {
    fn surroundings(hitbox: Hitbox) -> ((usize, usize), (usize, usize)) {
        let block_x = hitbox.center().x as usize / BLOCK_SIZE as usize;
        let block_y = hitbox.center().y as usize / BLOCK_SIZE as usize;

        let from_x = block_x as isize - CHECK_DISTANCE as isize;
        let from_y = block_y as isize - CHECK_DISTANCE as isize;
        
        let from_x = if from_x < 0 { 0 } else { from_x as usize };
        let from_y = if from_y < 0 { 0 } else { from_y as usize };

        let to_x = block_x + CHECK_DISTANCE;
        let to_x = if to_x >= LEVEL_WIDTH {
            LEVEL_WIDTH - 1
        } else {
            to_x
        };

        let to_y = block_y + CHECK_DISTANCE;
        let to_y = if to_y >= LEVEL_HEIGHT {
            LEVEL_HEIGHT - 1
        } else {
            to_y
        };

        ((from_x, to_x), (from_y, to_y))
    }

    fn would_collide(&self, world: &PlayableLevel, mov: Vector2D<f64>) -> bool {
        let speed = vec_map(&mov, |x| x.round() as i32);
        let mut hitbox = self.hitbox;
        hitbox.offset(speed.x, speed.y);

        let ((from_x, to_x), (from_y, to_y)) = Self::surroundings(hitbox);

        for y in from_y..=to_y {
            for x in from_x..=to_x {
                if let Some(block_hitbox) = world.block_hitbox(x, y) {
                    if hitbox.collides(&block_hitbox) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn bisect_movement(
        &mut self,
        world: &mut PlayableLevel,
        player: bool,
    ) -> Vector2D<f64> {
        fn approx_eq(v0: Vector2D<f64>, v1: Vector2D<f64>) -> bool {
            vec_map(&v0, |x| x.round()) == vec_map(&v1, |x| x.round())
        }

        const MAX_ITER: usize = 32;
        let mut min = vec2d!(0.0, 0.0);
        let mut max = self.speed();

        if !(self.would_collide(world, max)) {
            return max;
        }

        for _ in 0..MAX_ITER {
            if approx_eq(min, max) {
                break;
            }

            let between = vec_map(&(min + max), |x| x / 2.0);

            if self.would_collide(world, max) {
                max = between;
            } else {
                min = between;
            }
        }

        min
    }

    fn bump_blocks(&self, world: &mut PlayableLevel) {
        let ((from_x, to_x), (from_y, to_y)) = Self::surroundings(self.hitbox);

        for y in from_y..=to_y {
            for x in from_x..=to_x {
                if !world.blocks[y][x].block.is_bumpable() {
                    continue;
                }

                if let Some(block_hitbox) = world.block_hitbox(x, y) {
                    if block_hitbox.center().y() > self.hitbox.center().y() {
                        continue;
                    }
                    
                    if self.hitbox.collides(&block_hitbox) {
                        world.blocks[y][x].state = BlockState::Bumped;
                    }
                }
            }
        }
    }

    pub fn apply_movement(&mut self, world: &mut PlayableLevel, player: bool) {
        let speed = self.speed();
        if player {
            self.bump_blocks(world);
        }

        let acceptable_speed = self.bisect_movement(world, player);
        self.move_by_vec(vec_map(&acceptable_speed, |x| x.round() as i32));
    }
}
