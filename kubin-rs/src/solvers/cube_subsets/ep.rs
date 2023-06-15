use cube::subcases::CubeSubset;
use serde::{Deserialize, Serialize};
use subcube_derive::CubeSubset;

pub(crate) const EP_CASES: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 4 * 3 * 2;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, CubeSubset)]
pub(crate) struct EdgePermutation {
    ur_p: u8,
    uf_p: u8,
    ul_p: u8,
    ub_p: u8,
    dr_p: u8,
    df_p: u8,
    dl_p: u8,
    db_p: u8,
    fr_p: u8,
    fl_p: u8,
    br_p: u8,
    bl_p: u8,
}
