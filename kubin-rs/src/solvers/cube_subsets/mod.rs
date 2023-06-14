mod cross;
mod f2l;
mod oll;
mod pll;

use cube::Cube;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub(super) trait CubeSubset:
    PartialEq + Eq + Hash + Serialize + for<'de> Deserialize<'de>
{
    fn from_cube(cube: &Cube) -> Self;
}

pub(super) use self::cross::Cross;
pub(super) use self::cross::CROSS_CASES;

pub(super) use self::f2l::Corners;
pub(super) use self::f2l::Edges;
pub(super) use self::f2l::TwoBackPairsOneEdge;
pub(super) use self::f2l::TwoFrontPairsOneEdge;
pub(super) use self::f2l::CORNER_CASES;
pub(super) use self::f2l::EDGE_CASES;
pub(super) use self::f2l::TWO_PAIRS_ONE_EDGE_CASES;

pub(super) use self::oll::Oll;
pub(super) use self::oll::OLL_CASES;

pub(super) use self::pll::Pll;
pub(super) use self::pll::PLL_CASES;
