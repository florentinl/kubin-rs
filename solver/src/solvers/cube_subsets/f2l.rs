use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;

use cube_macros::CubeSubset;

pub(crate) const TWO_PAIRS_ONE_EDGE_CASES: usize = 1_354_752; // (8 * 7 * 3²) * (8 * 7 * 6 * 2³);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, CubeSubset)]
pub struct TwoFrontPairsOneEdge {
    dfr: (u8, u8),
    dlf: (u8, u8),
    fr: (u8, u8),
    fl: (u8, u8),
    br: (u8, u8),
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, CubeSubset)]
pub struct TwoBackPairsOneEdge {
    dbl: (u8, u8),
    drb: (u8, u8),
    bl: (u8, u8),
    br: (u8, u8),
    fr: (u8, u8),
}

pub(crate) const CORNER_CASES: usize = 8 * 7 * 6 * 5 * usize::pow(3, 4);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, CubeSubset)]
pub struct Corners {
    dfr: (u8, u8),
    dlf: (u8, u8),
    dbl: (u8, u8),
    drb: (u8, u8),
}

pub(crate) const EDGE_CASES: usize = 8 * 7 * 6 * 5 * usize::pow(2, 4);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, CubeSubset)]
pub struct Edges {
    fr: (u8, u8),
    fl: (u8, u8),
    bl: (u8, u8),
    br: (u8, u8),
}
