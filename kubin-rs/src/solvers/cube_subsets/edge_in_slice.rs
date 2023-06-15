use cube::{edge::EdgePiece, subcases::CubeSubset, Cube};
use serde::{Deserialize, Serialize};

pub(crate) const EDGE_IN_SLICE_CASES: usize = 495;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct EdgeInSlice {
    uf: bool,
    ur: bool,
    ub: bool,
    ul: bool,
    df: bool,
    dr: bool,
    db: bool,
    dl: bool,
    fr: bool,
    fl: bool,
    br: bool,
    bl: bool,
}

impl CubeSubset for EdgeInSlice {
    fn from_cube(cube: &Cube) -> Self {
        Self {
            uf: !is_slice_edge(&cube.edges[cube::UF].piece),
            ur: !is_slice_edge(&cube.edges[cube::UR].piece),
            ub: !is_slice_edge(&cube.edges[cube::UB].piece),
            ul: !is_slice_edge(&cube.edges[cube::UL].piece),
            df: !is_slice_edge(&cube.edges[cube::DF].piece),
            dr: !is_slice_edge(&cube.edges[cube::DR].piece),
            db: !is_slice_edge(&cube.edges[cube::DB].piece),
            dl: !is_slice_edge(&cube.edges[cube::DL].piece),
            fr: is_slice_edge(&cube.edges[cube::FR].piece),
            fl: is_slice_edge(&cube.edges[cube::FL].piece),
            br: is_slice_edge(&cube.edges[cube::BR].piece),
            bl: is_slice_edge(&cube.edges[cube::BL].piece),
        }
    }
}

fn is_slice_edge(edge: &EdgePiece) -> bool {
    matches!(
        edge,
        EdgePiece::FR | EdgePiece::FL | EdgePiece::BR | EdgePiece::BL
    )
}
