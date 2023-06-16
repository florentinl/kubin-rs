use cube::subcases::CubeSubset;
use cube_macros::CubeSubset;
use serde::{Deserialize, Serialize};
pub(crate) const CP_CASES: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub(crate) struct CornerPermutation {
    urf_p: u8,
    ubr_p: u8,
    ufl_p: u8,
    ulb_p: u8,
    dfr_p: u8,
    drb_p: u8,
    dlf_p: u8,
    dbl_p: u8,
}
