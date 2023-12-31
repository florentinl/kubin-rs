use cube::subcases::CubeSubset;
use cube_macros::CubeSubset;
use serde::{Deserialize, Serialize};

pub(crate) const BLOCK_CASES: usize = (8 * 3) * (12 * 11 * 10 * usize::pow(2, 3));

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub struct FrontLeftBlock {
    dlf: (u8, u8),
    df: (u8, u8),
    fl: (u8, u8),
    dl: (u8, u8),
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub struct FrontRightBlock {
    dfr: (u8, u8),
    df: (u8, u8),
    fr: (u8, u8),
    dr: (u8, u8),
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub struct BackLeftBlock {
    dbl: (u8, u8),
    db: (u8, u8),
    bl: (u8, u8),
    dl: (u8, u8),
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub struct BackRightBlock {
    drb: (u8, u8),
    db: (u8, u8),
    br: (u8, u8),
    dr: (u8, u8),
}
