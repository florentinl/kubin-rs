use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;
use subcube_derive::CubeSubset;

pub(crate) const CROSS_CASES: usize = 190_080;

/// Associate each cross piece with its index in the edges array and its orientation.
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, CubeSubset)]
pub(crate) struct Cross {
    df: (u8, u8),
    dr: (u8, u8),
    dl: (u8, u8),
    db: (u8, u8),
}
