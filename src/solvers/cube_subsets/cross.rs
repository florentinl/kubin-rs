use serde::{Deserialize, Serialize};

use crate::{
    cube::edge::{self, Edge},
    solvers::cube_subsets::CubeSubset,
};

pub(crate) const CROSS_CASES: usize = 190_080;

/// Associate each cross piece with its index in the edges array and its orientation.
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct Cross {
    df: (u8, u8),
    dr: (u8, u8),
    dl: (u8, u8),
    db: (u8, u8),
}

impl CubeSubset for Cross {
    /// Get cross case from the cube.
    fn from_cube(cube: &crate::cube::Cube) -> Self {
        let mut df = (0, 0);
        let mut dr = (0, 0);
        let mut dl = (0, 0);
        let mut db = (0, 0);

        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                edge::EdgePiece::DF => df = (i, *orientation),
                edge::EdgePiece::DR => dr = (i, *orientation),
                edge::EdgePiece::DL => dl = (i, *orientation),
                edge::EdgePiece::DB => db = (i, *orientation),
                _ => {}
            }
        }

        Self { df, dr, dl, db }
    }
}
