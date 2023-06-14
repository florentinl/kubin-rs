mod cross;
mod f2l;
mod oll;
mod pll;

use crate::cube::Cube;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub(super) trait CubeSubset:
    PartialEq + Eq + Hash + Serialize + for<'de> Deserialize<'de>
{
    fn from_cube(cube: &Cube) -> Self;
}

pub(super) use self::cross::Cross;
pub(super) use self::cross::CROSS_CASES;

pub(super) use self::f2l::CornerCase;
pub(super) use self::f2l::EdgeCase;
pub(super) use self::f2l::TwoPairsOneEdgeBackCase;
pub(super) use self::f2l::TwoPairsOneEdgeFrontCase;
pub(super) use self::f2l::CORNER_CASES;
pub(super) use self::f2l::EDGE_CASES;
pub(super) use self::f2l::TWO_PAIRS_ONE_EDGE_CASES;

pub(super) use self::oll::Oll;
pub(super) use self::oll::OLL_CASES;

pub(super) use self::pll::Pll;
pub(super) use self::pll::PLL_CASES;
