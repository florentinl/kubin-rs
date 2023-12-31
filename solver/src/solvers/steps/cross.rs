//! # Solve the cross in the CFOP method.
//!
//! Because there are only a few thousand possible states of the cross, we can
//! generate a lookup table for all of them, using a breadth-first search.

use std::{
    collections::{HashMap, VecDeque},
    io::Write,
};

use crate::solvers::{
    cube_subsets::Cross, cube_subsets::CROSS_CASES, solver::Step, utils::print_progress,
};
use cube::subcases::CubeSubset;
use cube::{self, algorithms::Move, Cube};
use serde::{Deserialize, Serialize};

use crate::solvers::utils::print_terminated;

#[derive(Serialize, Deserialize, Clone)]
pub struct Solver {
    cases: HashMap<Cross, Vec<Move>>,
}

impl Solver {
    fn generate_solutions(&mut self) {
        // Using a breadth-first search, generate a lookup table for all possible cross cases.
        let mut queue = VecDeque::with_capacity(CROSS_CASES);
        queue.push_back((cube::Cube::default(), Vec::new()));

        while let Some((cube, solution)) = queue.pop_front() {
            let progress = self.cases.len();
            print_progress!(
                "Generating lookup table for",
                "Cross",
                progress,
                CROSS_CASES
            );

            let case = Cross::from_cube(&cube);
            if self.cases.contains_key(&case) {
                continue;
            }

            self.cases
                .insert(case, cube::algorithms::invert_algorithm(&solution));

            for move_ in cube::algorithms::ALL_MOVES {
                let mut cube = cube.clone();
                cube.execute_move(&move_);
                let mut solution = solution.clone();
                solution.push(move_);
                queue.push_back((cube, solution));
            }
        }
        print_terminated!(
            "Generating lookup table for",
            "Cross",
            self.cases.len(),
            CROSS_CASES
        );
    }
}

impl Step for Solver {
    fn generate() -> Self {
        let mut cross_solver = Self {
            cases: HashMap::with_capacity(CROSS_CASES),
        };
        cross_solver.generate_solutions();
        cross_solver
    }

    fn solve(&self, cube: &Cube) -> Vec<Move> {
        let case = Cross::from_cube(cube);
        self.cases.get(&case).unwrap_or(&Vec::new()).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_solutions() {
        let solver = Solver::generate();
        assert_eq!(solver.cases.len(), CROSS_CASES);
    }

    #[test]
    fn solve_3_move_cross() {
        let solver = Solver::generate();

        let mut cube = cube::Cube::default();
        let scramble = cube::algorithms::parse_algorithm("R F B'");
        cube.execute_algorithm(&scramble);

        let solution = solver.solve(&cube);
        assert_eq!(solution.len(), 3);
    }
}
