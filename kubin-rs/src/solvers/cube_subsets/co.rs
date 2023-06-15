use cube::{subcases::CubeSubset, Cube};
use serde::{Deserialize, Serialize};

pub(crate) const CO_CASES: usize = usize::pow(3, 7);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct CornerOrientation {
    urf: u8,
    ufl: u8,
    ulb: u8,
    ubr: u8,
    dfr: u8,
    dlf: u8,
    dbl: u8,
    drb: u8,
}

impl CubeSubset for CornerOrientation {
    fn from_cube(cube: &Cube) -> Self {
        Self {
            urf: cube.corners[cube::URF].orientation,
            ufl: cube.corners[cube::UFL].orientation,
            ulb: cube.corners[cube::ULB].orientation,
            ubr: cube.corners[cube::UBR].orientation,
            dfr: cube.corners[cube::DFR].orientation,
            dlf: cube.corners[cube::DLF].orientation,
            dbl: cube.corners[cube::DBL].orientation,
            drb: cube.corners[cube::DRB].orientation,
        }
    }
}
