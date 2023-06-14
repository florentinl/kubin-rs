//! # Solve the OLL step in the CFOP method.
//!
//! There are 57 OLL cases, so we can generate a lookup table beforehand and use
//! it to solve the cube in a single hash table lookup.
//!
//! For now the lookup table is hardcoded, but in the future I might be able to
//! generate it programmatically.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::solvers::{cube_subsets::CubeSubset, solver::StepSolver};

use cube::{
    algorithms::{invert_algorithm, invert_move, parse_algorithm, Move},
    Cube,
};

use crate::solvers::cube_subsets::Oll;
use crate::solvers::cube_subsets::OLL_CASES;

#[derive(Serialize, Deserialize)]
pub struct Solver {
    cases: HashMap<Oll, Vec<Move>>,
}

impl Solver {
    fn get_cases() -> HashMap<Oll, Vec<Move>> {
        let mut cases = HashMap::with_capacity(OLL_CASES);

        let oll_algs = vec![
            "",
            "R U2 R2' F R F' U2' R' F R F'",
            "B L' B' L U L2 F' L' F U' L'",
            "B U L U' L' B' U B L U L' U' B'",
            "B U L U' L' B' U' B L U L' U' B'",
            "F R U R' U' F' U' F R U R' U' F'",
            "R U R2 F R F2 U F",
            "L' U2 L U2 L F' L' F",
            "R U2 R' U2 R' F R F'",
            "R U R' U' R' F R2 U R' U' F'",
            "F U F' R' F R U' R' F' R",
            "F' L' U' L U F U F R U R' U' F'",
            "F R U R' U' F' U F R U R' U' F'",
            "F U R U2 R' U' R U R' F'",
            "R' F R U R' F' R F U' F'",
            "L' B' L R' U' R U L' B L",
            "R' F R U R' U' F' R U' R' U2 R",
            "R U R' U R' F R F' U2 R' F R F'",
            "R U2 R' F' L' U2 L F R U2 R'",
            "R' U2 F R U R' U' F2 U2 F R",
            "F U R U' R' F' U2 R' U' R' F R F' U R",
            "R U R' U R U' R' U R U2 R'",
            "R U2 R2' U' R2 U' R2' U2 R",
            "R2' D' R U2 R' D R U2 R",
            "L F R' F' L' F R F'",
            "R' F' L' F R F' L F",
            "L' U' L U' L' U2 L",
            "R U R' U R U2 R'",
            "F R U R' U' F2 L' U' L U F",
            "B' R B' R2 U R U R' U' R B2",
            "R2 U R' B' R U' R2 U R B R'",
            "L' U' B U L U' L' B' L",
            "R U B' U' R' U R B R'",
            "R U R' U' R' F R F'",
            "R U R' U' B' R' F R F' B",
            "R U2 R2 F R F' R U2 R'",
            "R U R' U' F' U2 F U R U R'",
            "R B U' B' U' B U B' R'",
            "L U L' U L U' L' U' L' B L B'",
            "L F' L' U' L U F U' L'",
            "R' F R U R' U' F' U R",
            "L U L' U L U2 L' F' L' U' L U F",
            "R' U' R U' R' U2 R F R U R' U' F'",
            "B' U' R' U R B",
            "B U L U' L' B'",
            "F R U R' U' F'",
            "R' U' R' F R F' U R",
            "F' L' U' L U L' U' L U F",
            "F R U R' U' R U R' U' F'",
            "R B' R2 F R2 B R2 F' R",
            "R B' R B R2 U2 F R' F' R",
            "F U R U' R' U R U' R' F'",
            "R' U' R U' R' U F' U F R",
            "F R U R' U' R U' R' U R U R' F'",
            "R U' L' U R' U L U2 L F' L' F",
            "R U2 R2 U' R U' R' U2 F R F'",
            "F R U R' U' R F' L F R' F' L'",
            "L' R U R' U' L R' F R F'",
        ];

        for alg in oll_algs {
            let alg = parse_algorithm(alg);
            let mut cube = Cube::default();
            cube.execute_algorithm(&invert_algorithm(&alg));
            let case = Oll::from_cube(&cube);
            cases.insert(case, alg);
        }

        cases
    }
}

impl StepSolver for Solver {
    fn generate() -> Self {
        let cases = Self::get_cases();
        Self { cases }
    }

    fn solve(&self, cube: &Cube) -> Vec<Move> {
        let mut cube = cube.clone();
        for u_move in [Move::None, Move::U, Move::U2, Move::Up].iter() {
            cube.execute_move(u_move);
            let case = Oll::from_cube(&cube);
            if let Some(alg) = self.cases.get(&case) {
                if !matches!(u_move, Move::None) {
                    return [u_move.clone()].iter().chain(alg.iter()).cloned().collect();
                } else {
                    return alg.clone();
                }
            }
            cube.execute_move(&invert_move(u_move));
        }
        vec![]
    }
}
