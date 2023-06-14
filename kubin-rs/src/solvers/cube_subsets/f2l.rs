use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;

use cube::{
    corner::{self, Corner},
    edge::{self, Edge},
    Cube,
};

pub(crate) const CORNER_CASES: usize = 8 * 7 * 6 * 5 * usize::pow(3, 4);
pub(crate) const EDGE_CASES: usize = 8 * 7 * 6 * 5 * usize::pow(2, 4);
pub(crate) const TWO_PAIRS_ONE_EDGE_CASES: usize =
    8 * 7 * 8 * 7 * 6 * usize::pow(3, 2) * usize::pow(2, 3);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TwoFrontPairsOneEdge {
    dfr: (u8, u8),
    dlf: (u8, u8),
    fr: (u8, u8),
    fl: (u8, u8),
    br: (u8, u8),
}

impl CubeSubset for TwoFrontPairsOneEdge {
    fn from_cube(cube: &Cube) -> Self {
        let mut dfr = (0, 0);
        let mut dlf = (0, 0);

        for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
            match piece {
                corner::CornerPiece::Dfr => dfr = (i, *orientation),
                corner::CornerPiece::Dlf => dlf = (i, *orientation),
                _ => {}
            }
        }

        let mut fr = (0, 0);
        let mut fl = (0, 0);
        let mut br = (0, 0);

        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                edge::EdgePiece::FR => fr = (i, *orientation),
                edge::EdgePiece::FL => fl = (i, *orientation),
                edge::EdgePiece::BR => br = (i, *orientation),
                _ => {}
            }
        }

        Self {
            dfr,
            dlf,
            fr,
            fl,
            br,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TwoBackPairsOneEdge {
    dbl: (u8, u8),
    drb: (u8, u8),
    bl: (u8, u8),
    br: (u8, u8),
    fr: (u8, u8),
}

impl CubeSubset for TwoBackPairsOneEdge {
    fn from_cube(cube: &Cube) -> Self {
        let mut dbl = (0, 0);
        let mut drb = (0, 0);

        for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
            match piece {
                corner::CornerPiece::Dbl => dbl = (i, *orientation),
                corner::CornerPiece::Drb => drb = (i, *orientation),
                _ => {}
            }
        }

        let mut bl = (0, 0);
        let mut br = (0, 0);
        let mut fr = (0, 0);

        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                edge::EdgePiece::BL => bl = (i, *orientation),
                edge::EdgePiece::BR => br = (i, *orientation),
                edge::EdgePiece::FR => fr = (i, *orientation),
                _ => {}
            }
        }

        Self {
            dbl,
            drb,
            bl,
            br,
            fr,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Corners {
    dfr: (u8, u8),
    dlf: (u8, u8),
    dbl: (u8, u8),
    drb: (u8, u8),
}

impl CubeSubset for Corners {
    fn from_cube(cube: &Cube) -> Self {
        let mut dfr = (0, 0);
        let mut dlf = (0, 0);
        let mut dbl = (0, 0);
        let mut drb = (0, 0);

        for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
            match piece {
                corner::CornerPiece::Dfr => dfr = (i, *orientation),
                corner::CornerPiece::Dlf => dlf = (i, *orientation),
                corner::CornerPiece::Dbl => dbl = (i, *orientation),
                corner::CornerPiece::Drb => drb = (i, *orientation),
                _ => {}
            }
        }

        Self { dfr, dlf, dbl, drb }
    }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Edges {
    fr: (u8, u8),
    fl: (u8, u8),
    bl: (u8, u8),
    br: (u8, u8),
}

impl CubeSubset for Edges {
    fn from_cube(cube: &Cube) -> Self {
        let mut fr = (0, 0);
        let mut fl = (0, 0);
        let mut bl = (0, 0);
        let mut br = (0, 0);

        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                edge::EdgePiece::FR => fr = (i, *orientation),
                edge::EdgePiece::FL => fl = (i, *orientation),
                edge::EdgePiece::BL => bl = (i, *orientation),
                edge::EdgePiece::BR => br = (i, *orientation),
                _ => {}
            }
        }

        Self { fr, fl, bl, br }
    }
}
