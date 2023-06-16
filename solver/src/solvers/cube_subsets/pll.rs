use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;

use cube_macros::CubeSubset;
pub(crate) const PLL_CASES: usize = 22 * 4;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub(crate) struct Pll {
    ur_p: u8,
    uf_p: u8,
    ul_p: u8,
    ub_p: u8,
    urf_p: u8,
    ubr_p: u8,
    ufl_p: u8,
    ulb_p: u8,
}
