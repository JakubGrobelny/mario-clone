use crate::block::*;
use crate::hitbox::*;
use crate::level::*;
use crate::physics::*;
use crate::utility::*;

use vector2d::Vector2D;

const CHECK_DISTANCE: usize = 4;

#[derive(Debug)]
struct Surroundings {
    from_x: usize,
    from_y: usize,
    to_x:   usize,
    to_y:   usize,
}

impl PhysicalBody {
    fn surroundings(hitbox: Hitbox) -> Surroundings {
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

        Surroundings {
            from_x,
            from_y,
            to_x,
            to_y,
        }
    }

    fn would_collide(
        &self,
        world: &PlayableLevel,
        mov: Vector2D<f64>,
        range: &Surroundings,
    ) -> bool {
        let speed = vec_map(&mov, |x| x.round() as i32);
        let mut hitbox = self.hitbox;
        hitbox.offset(speed.x, speed.y);

        for y in range.from_y..=range.to_y {
            for x in range.from_x..=range.to_x {
                if let Some(block_hitbox) = world.block_hitbox(x, y) {
                    if hitbox.collides(&block_hitbox) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_best(
        &self,
        world: &PlayableLevel,
        speed: Vector2D<f64>,
        range: &Surroundings,
    ) -> bool {
        let normalized = vec_map(&speed, |x| x.signum());
        let rounded = vec_map(&speed, |x| x.round());
        self.would_collide(world, normalized + rounded, range)
    }

    fn bisect_movement(
        &self,
        world: &mut PlayableLevel,
        player: bool,
        range: &Surroundings,
    ) -> Vector2D<f64> {
        fn approx_eq(v0: Vector2D<f64>, v1: Vector2D<f64>) -> bool {
            vec_map(&v0, |x| x.round()) == vec_map(&v1, |x| x.round())
        }

        const MAX_ITER: usize = 32;
        let mut min = vec2d!(0.0, 0.0);
        let mut max = self.speed();

        if !(self.would_collide(world, max, range)) {
            return max;
        }

        for _ in 0..MAX_ITER {
            if approx_eq(min, max) {
                break;
            }

            let between = (min + max) / 2.0;

            if self.would_collide(world, between, range) {
                max = between;
            } else if self.is_best(world, between, range) {
                return between;
            } else {
                min = between;
            }
        }

        min
    }

    fn bump_blocks(&self, world: &mut PlayableLevel, range: &Surroundings) {
        let mut hitbox = self.hitbox;
        let movement = vec_map(&self.speed(), |x| x.round() as i32);
        hitbox.offset(movement.x, movement.y);
        
        for y in range.from_y..=range.to_y {
            for x in range.from_x..=range.to_x {
                if !world.blocks[y][x].block.is_bumpable() {
                    continue;
                }

                if let Some(block_hitbox) = world.block_hitbox(x, y) {
                    if block_hitbox.center().y() > self.hitbox.center().y() {
                        continue;
                    }

                    if hitbox.collides(&block_hitbox) {
                        world.blocks[y][x].state = BlockState::Bumped;
                    }
                }
            }
        }
    }

    fn is_grounded(&self, world: &PlayableLevel, range: &Surroundings) -> bool {
        self.would_collide(world, vec2d!(0.0, 1.0), range)
    }

    fn try_move_x(
        &mut self,
        speed: f64,
        world: &PlayableLevel,
        range: &Surroundings,
    ) {
        let speed_vec = vec2d!(speed, 0.0);
        if self.would_collide(world, speed_vec, range) {
            self.stop_x();
        } else {
            self.move_by_vec(vec2d!(speed.round() as i32, 0));
        }
    }

    fn try_move_y(
        &mut self,
        speed: f64,
        world: &PlayableLevel,
        range: &Surroundings,
    ) {
        let speed_vec = vec2d!(0.0, speed);
        if self.would_collide(world, speed_vec, range) {
            self.stop_y();
        } else {
            self.move_by_vec(vec2d!(0, speed.round() as i32));
        }
    }

    pub fn apply_movement(&mut self, world: &mut PlayableLevel, player: bool) {
        let surroundings = Self::surroundings(self.hitbox);
        let speed = self.speed();
        if player {
            self.bump_blocks(world, &surroundings);
        }

        let acceptable_speed =
            self.bisect_movement(world, player, &surroundings);

        self.move_by_vec(vec_map(&acceptable_speed, |x| x.round() as i32));
        let difference = speed - acceptable_speed;

        self.try_move_y(difference.y, world, &surroundings);
        self.try_move_x(difference.x, world, &surroundings);

        if self.is_grounded(world, &surroundings) {
            self.grounded = true;
            self.stop_y();
        } else {
            self.grounded = false;
        }

        self.clear_speed();
    }
}
