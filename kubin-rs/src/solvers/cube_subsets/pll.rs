use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;

use subcube_derive::CubeSubset;
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

impl Pll {
    pub(crate) fn mirror_case(&self) -> Vec<Pll> {
        let Pll {
            ur_p: ur,
            uf_p: uf,
            ul_p: ul,
            ub_p: ub,
            urf_p: ufr,
            ubr_p: ubr,
            ufl_p: ufl,
            ulb_p: ulb,
        } = self;

        let mut cases: Vec<Pll> = vec![self.clone()];

        // U move offset
        cases.push(Pll {
            ur_p: *ub,
            uf_p: *ur,
            ul_p: *uf,
            ub_p: *ul,
            urf_p: *ubr,
            ubr_p: *ulb,
            ulb_p: *ufl,
            ufl_p: *ufr,
        });

        // U2 move offset
        cases.push(Pll {
            ur_p: *ul,
            uf_p: *ub,
            ul_p: *ur,
            ub_p: *uf,
            urf_p: *ulb,
            ubr_p: *ufl,
            ulb_p: *ufr,
            ufl_p: *ubr,
        });

        // U' move offset
        cases.push(Pll {
            ur_p: *uf,
            uf_p: *ul,
            ul_p: *ub,
            ub_p: *ur,
            urf_p: *ufl,
            ubr_p: *ufr,
            ulb_p: *ubr,
            ufl_p: *ulb,
        });

        cases
    }
}
