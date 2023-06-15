use cube::{edge::Piece, subcases::CubeSubset, Cube};
use serde::{Deserialize, Serialize};

pub(crate) const EDGE_IN_SLICE_CASES: usize = 495;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct EdgeInSlice {
    edge_mask: u16,
}

impl CubeSubset for EdgeInSlice {
    fn from_cube(cube: &Cube) -> Self {
        let mask = !is_slice_edge(&cube.edges[cube::UF].piece)
            ^ !is_slice_edge(&cube.edges[cube::UR].piece) << 1
            ^ !is_slice_edge(&cube.edges[cube::UB].piece) << 2
            ^ !is_slice_edge(&cube.edges[cube::UL].piece) << 3
            ^ !is_slice_edge(&cube.edges[cube::DF].piece) << 4
            ^ !is_slice_edge(&cube.edges[cube::DR].piece) << 5
            ^ !is_slice_edge(&cube.edges[cube::DB].piece) << 6
            ^ !is_slice_edge(&cube.edges[cube::DL].piece) << 7
            ^ is_slice_edge(&cube.edges[cube::FR].piece) << 8
            ^ is_slice_edge(&cube.edges[cube::FL].piece) << 9
            ^ is_slice_edge(&cube.edges[cube::BR].piece) << 10
            ^ is_slice_edge(&cube.edges[cube::BL].piece) << 11;
        Self { edge_mask: mask }
    }
}

fn is_slice_edge(edge: &Piece) -> u16 {
    matches!(edge, Piece::FR | Piece::FL | Piece::BR | Piece::BL).into()
}
