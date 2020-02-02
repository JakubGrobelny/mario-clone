use serde::{Deserialize, Serialize};

// https://www.mariowiki.com/List_of_enemies_by_game#Super_Mario_Bros.
#[derive(Deserialize, Serialize)]
#[derive(Copy, Clone)]
pub enum EnemyType {
    Koopa,
    Goomba,
    BuzzyBeetle,
    Spiny,
    HammerBro,
}
