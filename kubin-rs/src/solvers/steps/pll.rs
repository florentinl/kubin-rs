//! # Solve the PLL step in the CFOP method
//!
//! There are 21 PLL cases, so we can generate a lookup table beforehand and use
//! it to solve the cube in a single hash table lookup.
//!
//! For now the lookup table is hardcoded, but in the future I might be able to
//! generate it programmatically.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::solvers::solver::StepSolver;

use cube::{
    algorithms::{invert_algorithm, invert_move, parse_algorithm, Move},
    subcases::CubeSubset,
    Cube,
};

use crate::solvers::cube_subsets::Pll;
use crate::solvers::cube_subsets::PLL_CASES;

#[derive(Serialize, Deserialize)]
pub struct Solver {
    cases: HashMap<Pll, Vec<Move>>,
}

impl Solver {
    fn get_cases() -> HashMap<Pll, Vec<Move>> {
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
            let case = Pll::from_cube(&cube);
            for case in case.mirror_case() {
                cases.insert(case, alg.clone());
            }
        }

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
            let case = Pll::from_cube(&cube);
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
                            .filter(|m| **m != Move::None)
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
