use cube::{subcases::CubeSubset, Cube};
use serde::{Deserialize, Serialize};

pub(crate) const EO_CASES: usize = usize::pow(2, 11);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct EdgeOrientation {
    uf: u8,
    ur: u8,
    ub: u8,
    ul: u8,
    df: u8,
    dr: u8,
    db: u8,
    dl: u8,
    fr: u8,
    fl: u8,
    br: u8,
    bl: u8,
}

impl CubeSubset for EdgeOrientation {
    fn from_cube(cube: &Cube) -> Self {
        Self {
            uf: cube.edges[cube::UF].orientation,
            ur: cube.edges[cube::UR].orientation,
            ub: cube.edges[cube::UB].orientation,
            ul: cube.edges[cube::UL].orientation,
            df: cube.edges[cube::DF].orientation,
            dr: cube.edges[cube::DR].orientation,
            db: cube.edges[cube::DB].orientation,
            dl: cube.edges[cube::DL].orientation,
            fr: cube.edges[cube::FR].orientation,
            fl: cube.edges[cube::FL].orientation,
            br: cube.edges[cube::BR].orientation,
            bl: cube.edges[cube::BL].orientation,
        }
    }
}
