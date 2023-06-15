use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;
use cube::{corner::CornerPiece, edge::EdgePiece, Cube};
pub(crate) const PLL_CASES: usize = 22 * 4;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub(crate) struct Pll {
    ur: u8,
    uf: u8,
    ul: u8,
    ub: u8,
    ufr: u8,
    ubr: u8,
    ufl: u8,
    ulb: u8,
}

impl CubeSubset for Pll {
    fn from_cube(cube: &Cube) -> Self {
        let mut ur = 0;
        let mut uf = 0;
        let mut ul = 0;
        let mut ub = 0;
        let mut ufr = 0;
        let mut ubr = 0;
        let mut ufl = 0;
        let mut ulb = 0;

        for (edge, i) in cube.edges.iter().zip(0..) {
            match edge.piece {
                EdgePiece::UR => ur = i,
                EdgePiece::UF => uf = i,
                EdgePiece::UL => ul = i,
                EdgePiece::UB => ub = i,
                _ => {}
            }
        }

        for (corner, i) in cube.corners.iter().zip(0..) {
            match corner.piece {
                CornerPiece::Urf => ufr = i,
                CornerPiece::Ubr => ubr = i,
                CornerPiece::Ufl => ufl = i,
                CornerPiece::Ulb => ulb = i,
                _ => {}
            }
        }

        Self {
            ur,
            uf,
            ul,
            ub,
            ufr,
            ubr,
            ufl,
            ulb,
        }
    }
}

impl Pll {
    pub(crate) fn mirror_case(&self) -> Vec<Pll> {
        let Pll {
            ur,
            uf,
            ul,
            ub,
            ufr,
            ubr,
            ufl,
            ulb,
        } = self;

        let mut cases: Vec<Pll> = vec![self.clone()];

        // U move offset
        cases.push(Pll {
            ur: *ub,
            uf: *ur,
            ul: *uf,
            ub: *ul,
            ufr: *ubr,
            ubr: *ulb,
            ulb: *ufl,
            ufl: *ufr,
        });

        // U2 move offset
        cases.push(Pll {
            ur: *ul,
            uf: *ub,
            ul: *ur,
            ub: *uf,
            ufr: *ulb,
            ubr: *ufl,
            ulb: *ufr,
            ufl: *ubr,
        });

        // U' move offset
        cases.push(Pll {
            ur: *uf,
            uf: *ul,
            ul: *ub,
            ub: *ur,
            ufr: *ufl,
            ubr: *ufr,
            ulb: *ubr,
            ufl: *ulb,
        });

        cases
    }
}