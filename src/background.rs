use serde::{Deserialize, Serialize};

#[derive(Copy, Clone)]
#[derive(Deserialize, Serialize, Hash)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum BackgroundElement {
    Air,
    Water,
    Fence,
    TreeTopSmall,
    TreeTopBigUpper,
    TreeTopBigLower,
    TreeBottom,
    GrassLeft,
    GrassRight,
    GrassMiddle,
    Castle,
}

impl Default for BackgroundElement {
    fn default() -> Self {
        Self::Air
    }
}
