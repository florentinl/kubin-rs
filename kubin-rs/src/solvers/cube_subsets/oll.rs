use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;

use cube::{self, Cube};

pub(crate) const OLL_CASES: usize = 58;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct Oll {
    uf: u8,
    ur: u8,
    ub: u8,
    ul: u8,
    urf: u8,
    ubr: u8,
    ulb: u8,
    ufl: u8,
}

impl CubeSubset for Oll {
    fn from_cube(cube: &Cube) -> Self {
        Self {
            uf: cube.edges[cube::UF].orientation,
            ur: cube.edges[cube::UR].orientation,
            ub: cube.edges[cube::UB].orientation,
            ul: cube.edges[cube::UL].orientation,
            urf: cube.corners[cube::URF].orientation,
            ubr: cube.corners[cube::UBR].orientation,
            ulb: cube.corners[cube::ULB].orientation,
            ufl: cube.corners[cube::UFL].orientation,
        }
    }
}
