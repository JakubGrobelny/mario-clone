use crate::utility::*;

pub const BLOCK_SIZE: u32 = 64;

#[derive(Copy, Clone, Debug)]
pub enum Block {
    Air,
    Bricks,
    QuestionMark(BlockContents),
}

#[derive(Copy, Clone, Debug)]
pub enum BlockContents {
    Coins(u8),
    Mushroom,
    Empty,
}

impl Default for Block {
    fn default() -> Self {
        Block::Air
    }
}

impl From<char> for Block {
    fn from(c: char) -> Block {
        if c.is_digit(10) {
            return Block::QuestionMark(BlockContents::Coins(
                c.to_digit(10).unwrap() as u8,
            ));
        }

        match c {
            '?' => Block::QuestionMark(BlockContents::Mushroom),
            '#' => Block::Bricks,
            ' ' => Block::Air,
            _ => {
                panic_with_messagebox(&format!(
                    "Corrupted level data (invalid block type {}",
                    c
                ));
            }
        }
    }
}
