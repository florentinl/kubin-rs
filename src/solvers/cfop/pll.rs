//! # Solve the PLL step in the CFOP method
//!
//! There are 21 PLL cases, so we can generate a lookup table beforehand and use
//! it to solve the cube in a single hash table lookup.
//!
//! For now the lookup table is hardcoded, but in the future I might be able to
//! generate it programmatically.

use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{
    cube::{
        algorithms::{invert_algorithm, invert_move, parse_algorithm, Move},
        corner::CornerPiece,
        edge::EdgePiece,
        Cube,
    },
    solvers::solver::{Case, StepSolver},
};

const PLL_CASES: usize = 22 * 4;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
struct PllCase {
    ur: usize,
    uf: usize,
    ul: usize,
    ub: usize,
    ufr: usize,
    ubr: usize,
    ufl: usize,
    ulb: usize,
}

impl Case for PllCase {
    fn from_cube(cube: &Cube) -> Self {
        let mut ur = 0;
        let mut uf = 0;
        let mut ul = 0;
        let mut ub = 0;
        let mut ufr = 0;
        let mut ubr = 0;
        let mut ufl = 0;
        let mut ulb = 0;

        for (i, edge) in cube.edges.iter().enumerate() {
            match edge.piece {
                EdgePiece::UR => ur = i,
                EdgePiece::UF => uf = i,
                EdgePiece::UL => ul = i,
                EdgePiece::UB => ub = i,
                _ => {}
            }
        }

        for (i, corner) in cube.corners.iter().enumerate() {
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

#[derive(Serialize, Deserialize)]
pub struct Solver {
    cases: HashMap<PllCase, Vec<Move>>,
}

impl Solver {
    fn get_cases() -> HashMap<PllCase, Vec<Move>> {
        let mut cases = HashMap::with_capacity(PLL_CASES);

        let pll_algs = vec![
            "",
            "R' F R' B2 R F' R' B2 R2",
            "R2 B2 R F R' B2 R F' R",
            "R U' R U R U R U' R' U' R2",
            "R2 U R U R' U' R' U' R' U R'",
            "R U R' U' R' F R2 U' R' U' R U R' F'",
            "L R2 U R U R2 U' R' U' R2 U' R U2 L' U R'",
            "R U R D R' U R D' R' U' R D R' U' R D' R2",
            "L U2 L' U2 L F' L' U' L U L F L2",
            "R' U2 R U2 R' F R U R' U' R' F' R2' U'",
            "R2 U R' U R' U' R U' R2 D U' R' U R D'",
            "R' U' R U D' R2 U R' U R U' R U' R2 D",
            "R2 U' R U' R U R' U R2 D' U R U' R' D",
            "R U R' U' D R2 U' R U' R' U R' U R2 D'",
            "L U' R' U L' U2 R U' R' U2 R",
            "R U R' F' R U R' U' R' F R2 U' R' U'",
            "R U R' U R U R' F' R U R' U' R' F R2 U' R' U2 R U' R'",
            "R' U R U' R' F' U' F R U R' F R' F' R U' R",
            "R2 U2 R U2 R2 U2 R2 U2 R U2 R2",
            "R' U' R U' R U R U' R' U R U R2 U' R'",
            "R' U' R U' L R U2 R' U' R U2 L' U R2 U R",
            "R' U R' U' B' R' B2 U' B' U B' R B R",
        ];

        for alg in pll_algs {
            let alg = parse_algorithm(alg);
            let mut cube = Cube::default();
            cube.execute_algorithm(&invert_algorithm(&alg));
            let case = PllCase::from_cube(&cube);
            for case in Self::mirror_cases(&case) {
                cases.insert(case, alg.clone());
            }
        }

        cases
    }

    fn mirror_cases(case: &PllCase) -> Vec<PllCase> {
        let PllCase {
            ur,
            uf,
            ul,
            ub,
            ufr,
            ubr,
            ufl,
            ulb,
        } = case;

        let mut cases: Vec<PllCase> = vec![case.clone()];

        // U move offset
        cases.push(PllCase {
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
        cases.push(PllCase {
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
        cases.push(PllCase {
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

    fn is_solved(&self, cube: &Cube) -> bool {
        *cube == Cube::default()
    }
}

impl StepSolver for Solver {
    fn generate() -> Self {
        let cases = Self::get_cases();
        Self { cases }
    }

    fn solve(&self, cube: &Cube) -> Vec<Move> {
        let mut cube = cube.clone();
        for pre_u_move in [Move::None, Move::U, Move::U2, Move::Up].iter() {
            cube.execute_move(pre_u_move);
            let case = PllCase::from_cube(&cube);
            if let Some(alg) = self.cases.get(&case) {
                cube.execute_algorithm(alg);
                // Adjust U face
                for post_u_move in [Move::None, Move::U, Move::U2, Move::Up] {
                    cube.execute_move(&post_u_move);
                    if self.is_solved(&cube) {
                        let solution = [pre_u_move.clone()]
                            .iter()
                            .chain(alg.iter())
                            .chain([post_u_move.clone()].iter())
                            .cloned()
                            .collect::<Vec<Move>>();

                        return solution;
                    }
                    cube.execute_move(&invert_move(&post_u_move));
                }
            }
            cube.execute_move(&invert_move(pre_u_move));
        }
        vec![]
    }
}
