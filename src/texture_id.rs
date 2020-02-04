use serde::{Deserialize};

#[derive(PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum TextureId {
    CollectibleCoin,
    CollectibleMushroom,
    CollectibleStar,
    CollectibleFlower,
    PlayerStanding,
    PlayerRunning,
    PlayerJumping,
    BigPlayerStanding,
    BigPlayerRunning,
    BigPlayerJumping,
    SmallCloud,
    BigCloud,
}
