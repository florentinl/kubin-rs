use cube::subcases::CubeSubset;
use cube_macros::CubeSubset;
use serde::{Deserialize, Serialize};
pub(crate) const CP_CASES: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * (usize::pow(3, 4));

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub(crate) struct CornerPermutation {
    urf: (u8, u8),
    ubr: (u8, u8),
    ufl: (u8, u8),
    ulb: (u8, u8),
    dfr_p: u8,
    drb_p: u8,
    dlf_p: u8,
    dbl_p: u8,
}
