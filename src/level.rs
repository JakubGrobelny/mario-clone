use crate::background::*;
use crate::block::*;
use crate::enemy::*;
use crate::entity::*;
use crate::hitbox::*;
use crate::render::*;
use crate::resource::*;
use crate::texture_id::*;

use rand::Rng;

use sdl2::pixels::Color;

use serde::{Deserialize, Serialize};

pub const LEVEL_HEIGHT: usize = 20;
pub const LEVEL_WIDTH: usize = 220;

pub type BlockArray<T> = Box<[[T; LEVEL_WIDTH]; LEVEL_HEIGHT]>;

#[derive(Clone)]
pub struct Level {
    pub theme:  LevelTheme,
    blocks:     BlockArray<Block>,
    background: BlockArray<BackgroundElement>,
    entities:   Vec<EntityPrototype>,
}

#[derive(Deserialize, Serialize)]
pub struct LevelJSON {
    theme:      LevelTheme,
    blocks:     Vec<Block>,
    background: Vec<BackgroundElement>,
    entities:   Vec<EntityPrototype>,
}

#[derive(Clone)]
pub struct PlayableLevel {
    pub prototype: Level,
    pub blocks:    BlockArray<RealBlock>,
    pub entities:  Vec<Entity>,
    pub clouds:    Vec<Cloud>,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Cloud {
    pub position: (i32, i32),
    pub is_small: bool,
}

#[derive(Deserialize, Serialize)]
#[derive(Copy, Clone)]
#[derive(Debug)]
#[repr(u8)]
pub enum LevelTheme {
    Day,
    Underground,
    Night,
}

impl From<&Level> for LevelJSON {
    fn from(lvl: &Level) -> LevelJSON {
        fn array_to_vec<T: Copy>(array: &BlockArray<T>) -> Vec<T> {
            array
                .iter()
                .map(|row| row.iter())
                .flatten()
                .copied()
                .collect()
        }
        let blocks = array_to_vec(&lvl.blocks);
        let background = array_to_vec(&lvl.background);

        LevelJSON {
            theme: lvl.theme,
            blocks,
            background,
            entities: lvl.entities.clone(),
        }
    }
}

impl LevelTheme {
    pub fn next(self) -> LevelTheme {
        match self {
            LevelTheme::Day => LevelTheme::Underground,
            LevelTheme::Underground => LevelTheme::Night,
            LevelTheme::Night => LevelTheme::Day,
        }
    }

    pub fn prev(self) -> LevelTheme {
        match self {
            LevelTheme::Day => LevelTheme::Night,
            LevelTheme::Underground => LevelTheme::Day,
            LevelTheme::Night => LevelTheme::Underground,
        }
    }
}

impl PlayableLevel {
    pub fn block_hitbox(&self, x: usize, y: usize) -> Option<Hitbox> {
        let block = self.blocks[y][x];
        if block.block.is_collidable() {
            Some(Block::hitbox(x, y))
        } else {
            None
        }
    }
}

impl Default for Level {
    fn default() -> Level {
        Level::new()
    }
}

impl From<LevelJSON> for Level {
    fn from(json: LevelJSON) -> Level {
        if json.blocks.len() != LEVEL_HEIGHT * LEVEL_WIDTH {
            panic_with_messagebox!(
                "Corrupted level state (invalid level size)!"
            );
        }

        let mut blocks = Level::default_blocks();

        for (i, block) in json.blocks.into_iter().enumerate() {
            let row = i / LEVEL_WIDTH;
            let col = i % LEVEL_WIDTH;
            blocks[row][col] = block;
        }

        let mut background = Level::default_blocks();
        for (i, bg_elem) in json.background.into_iter().enumerate() {
            let row = i / LEVEL_WIDTH;
            let col = i % LEVEL_WIDTH;
            background[row][col] = bg_elem;
        }

        Level {
            theme: json.theme,
            blocks,
            background,
            entities: json.entities,
        }
    }
}

impl From<Level> for PlayableLevel {
    fn from(lvl: Level) -> PlayableLevel {
        fn generate_clouds() -> Vec<Cloud> {
            const STEP: i32 = 200;
            let mut x: i32 = 0;
            let mut clouds: Vec<Cloud> = vec![];
            let mut rng = rand::thread_rng();

            while x < LEVEL_WIDTH as i32 * BLOCK_SIZE as i32 {
                let step = rng.gen_range(0, 256);
                x += STEP + step;

                let max_y = LEVEL_HEIGHT as i32 * BLOCK_SIZE as i32 * 2 / 3;
                let y = rng.gen_range(0, max_y);
                let is_small: bool = rand::random();

                if !is_small {
                    x += STEP;
                }

                let cloud = Cloud {
                    position: (x, y),
                    is_small,
                };

                clouds.push(cloud);
            }
            clouds
        }
        let mut blocks: BlockArray<RealBlock> = Level::default_blocks();

        for (y, row) in lvl.blocks.iter().enumerate() {
            for (x, block) in row.iter().copied().enumerate() {
                blocks[y][x] = RealBlock::from(block);
            }
        }

        let entities = lvl.entities.iter().copied().map(Entity::from).collect();

        let clouds = generate_clouds();

        PlayableLevel {
            blocks,
            prototype: lvl,
            entities,
            clouds,
        }
    }
}

impl Level {
    fn default_blocks<T: Default + Copy>() -> BlockArray<T> {
        Box::new([[T::default(); LEVEL_WIDTH]; LEVEL_HEIGHT])
    }

    fn init_blocks() -> BlockArray<Block> {
        let mut blocks = Level::default_blocks();
        blocks[LEVEL_HEIGHT - 2][0] = Block::from(BlockType::GroundLeft);
        blocks[LEVEL_HEIGHT - 2][LEVEL_WIDTH - 1] =
            Block::from(BlockType::GroundRight);

        let ground = Block::from(BlockType::GroundMiddle);

        for col in 1..LEVEL_WIDTH - 1 {
            blocks[LEVEL_HEIGHT - 2][col] = ground;
        }

        blocks[LEVEL_HEIGHT - 1][0] = Block::from(BlockType::GroundBottomLeft);
        blocks[LEVEL_HEIGHT - 1][LEVEL_WIDTH - 1] =
            Block::from(BlockType::GroundBottomRight);
        for col in 1..LEVEL_WIDTH - 1 {
            blocks[LEVEL_HEIGHT - 1][col] =
                Block::from(BlockType::GroundBottomMiddle);
        }

        blocks
    }

    pub fn new() -> Level {
        const DEFAULT_THEME: LevelTheme = LevelTheme::Day;
        let blocks = Level::init_blocks();
        let background = Level::default_blocks();

        const FLAG_HEIGHT: i32 = 640;
        const FLAG_POS: (i32, i32) = (
            BLOCK_SIZE as i32 * 185,
            (LEVEL_HEIGHT - 2) as i32 * BLOCK_SIZE as i32 - FLAG_HEIGHT,
        );

        let flag_pole = EntityPrototype::new(EntityType::EndFlag, FLAG_POS);
        let entities = vec![flag_pole];

        Level {
            blocks,
            theme: DEFAULT_THEME,
            background,
            entities,
        }
    }

    pub fn get_bg(&mut self, (x, y): (usize, usize)) -> BackgroundElement {
        self.background[y][x]
    }

    pub fn set_bg(&mut self, (x, y): (usize, usize), bg: BackgroundElement) {
        self.background[y][x] = bg;
    }

    pub fn get_block(&mut self, (x, y): (usize, usize)) -> Block {
        self.blocks[y][x]
    }

    pub fn set_block(&mut self, (x, y): (usize, usize), block: Block) {
        self.blocks[y][x] = block;
    }

    pub fn fill_block(&mut self, (x, y): (usize, usize), item: Collectible) {
        self.blocks[y][x].insert_item(item);
    }

    pub fn remove_block_contents(&mut self, (x, y): (usize, usize)) {
        self.blocks[y][x].delete_item();
    }

    pub fn insert_entity(&mut self, entity: EntityPrototype) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, pos: (i32, i32)) {
        self.entities.retain(|entity| entity.position != pos);
    }

    pub fn get_entity(&mut self, pos: (i32, i32)) -> EnemyType {
        self.entities
            .iter()
            .find(|entity| entity.position == pos)
            .and_then(|entity| {
                match entity.kind {
                    EntityType::Enemy(enemy) => Some(enemy),
                    _ => None,
                }
            })
            .unwrap_or(EnemyType::Goomba)
    }
}

impl From<LevelTheme> for Color {
    fn from(theme: LevelTheme) -> Color {
        match theme {
            LevelTheme::Day => Color::RGB(88, 100, 255),
            LevelTheme::Night => Color::RGB(0, 0, 0),
            LevelTheme::Underground => Color::RGB(0, 0, 0),
        }
    }
}

impl Drawable for Level {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        if data.mode != DrawMode::Game {
            let color = Color::from(data.object.theme);
            data.renderer.canvas.set_draw_color(color);
            data.renderer.canvas.clear();
        }

        for (y, row) in data.object.background.iter().enumerate() {
            for (x, &bg) in row.iter().enumerate() {
                let x = x as i32 * BLOCK_SIZE as i32;
                let y = y as i32 * BLOCK_SIZE as i32;
                let themed_bg = ThemedBackgroundElement {
                    element: bg,
                    theme:   data.object.theme,
                };

                pass_draw!(data, &themed_bg).position((x, y)).show(res);
            }
        }

        if data.mode != DrawMode::Game {
            for (y, row) in data.object.blocks.iter().enumerate() {
                for (x, &block) in row.iter().enumerate() {
                    let x = x as i32 * BLOCK_SIZE as i32;
                    let y = y as i32 * BLOCK_SIZE as i32;
                    let block = ThemedBlock {
                        block,
                        theme: data.object.theme,
                    };
                    pass_draw!(data, &block).position((x, y)).show(res);
                }
            }
        }

        // TODO: draw entity prototypes if mode is editor
        if data.mode != DrawMode::Game {
            for entity in data.object.entities.iter() {
                pass_draw!(data, entity).show(res);
            }
        }
    }
}

impl Drawable for PlayableLevel {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let color = Color::from(data.object.prototype.theme);
        data.renderer.canvas.set_draw_color(color);
        data.renderer.canvas.clear();

        for cloud in data.object.clouds.iter() {
            pass_draw!(data, cloud)
                .scale(5.0)
                .camera(data.camera.cloud_camera())
                .show(res);
        }

        pass_draw!(data, &data.object.prototype).show(res);

        for (y, row) in data.object.blocks.iter().enumerate() {
            for (x, &block) in row.iter().enumerate() {
                let bump_amount = match block.state {
                    BlockState::Moving(amount) => amount as i32,
                    _ => 0,
                };

                let x = x as i32 * BLOCK_SIZE as i32;
                let y = y as i32 * BLOCK_SIZE as i32 - bump_amount;

                let block = ThemedBlock {
                    block: block.block,
                    theme: data.object.prototype.theme,
                };

                pass_draw!(data, &block).position((x, y)).show(res);
            }
        }

        for entity in data.object.entities.iter() {
            pass_draw!(data, entity).show(res);
        }
    }
}

impl Drawable for Cloud {
    fn show(data: DrawCall<Self>, res: &mut ResourceManager) {
        let texture_id = if data.object.is_small {
            TextureId::SmallCloud
        } else {
            TextureId::BigCloud
        };

        let info = res.entity_texture_info(texture_id);
        let (x, y) = data.object.position;

        let width = (info.width as f64 * data.scale) as u32;
        let height = (info.height as f64 * data.scale) as u32;

        if !data.camera.in_view(rect!(x, y, width, height)) {
            return;
        }

        let (cam_x, cam_y) = data.camera.translate_coords((x, y));
        let dest = rect!(cam_x, cam_y, width, height);

        let path = info.path.clone();

        let src_region = rect!(0, 0, info.width, info.height);

        data.renderer
            .canvas
            .copy(&res.texture(&path), src_region, dest)
            .expect("Failed to draw a cloud! :(");
    }
}
