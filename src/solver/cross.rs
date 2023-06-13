//! # Solve the cross in the CFOP method.
//!
//! Because there are only a few thousand possible states of the cross, we can
//! generate a lookup table for all of them, using a breadth-first search.

use std::{
    collections::{HashMap, VecDeque},
    io::Write,
};

const CROSS_CASES: usize = 190_080;

use crate::cube::{
    self,
    algorithms::Move,
    edge::{self, Edge},
};

/// Associate each cross piece with its index in the edges array and its orientation.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Case {
    df: (usize, usize),
    dr: (usize, usize),
    dl: (usize, usize),
    db: (usize, usize),
}

impl Case {
    /// Get cross case from the cube.
    #[must_use]
    pub fn from_cube(cube: &crate::cube::Cube) -> Self {
        let mut df = (0, 0);
        let mut dr = (0, 0);
        let mut dl = (0, 0);
        let mut db = (0, 0);

        for (i, Edge { piece, orientation }) in cube.edges.iter().enumerate() {
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

pub struct Solver {
    solutions: HashMap<Case, Vec<Move>>,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    #[must_use]
    pub fn new() -> Self {
        let mut cross_solver = Self {
            solutions: HashMap::with_capacity(CROSS_CASES),
        };
        cross_solver.generate_solutions();
        cross_solver
    }

    fn generate_solutions(&mut self) {
        // Using a breadth-first search, generate a lookup table for all possible cross cases.
        let mut queue = VecDeque::with_capacity(CROSS_CASES);
        queue.push_back((cube::Cube::default(), Vec::new()));

        while let Some((cube, solution)) = queue.pop_front() {
            self.print_progress();

            let case = Case::from_cube(&cube);
            if self.solutions.contains_key(&case) {
                continue;
            }

            self.solutions
                .insert(case, cube::algorithms::invert_algorithm(&solution));

            for move_ in cube::algorithms::ALL_MOVES {
                let mut cube = cube.clone();
                cube.execute_move(&move_);
                let mut solution = solution.clone();
                solution.push(move_);
                queue.push_back((cube, solution));
            }
        }
        println!(
            "Generating Cross Lookup Table ✅: {} / {}",
            self.solutions.len(),
            CROSS_CASES
        );
    }

    fn print_progress(&self) {
        if self.solutions.len() % 1000 == 0 {
            let icon = match (self.solutions.len() / 1000) % 4 {
                0 => "◜",
                1 => "◝",
                2 => "◞",
                3 => "◟",
                _ => unreachable!(),
            };
            print!(
                "Generating Cross Lookup Table {}: {} / {}\r",
                icon,
                self.solutions.len(),
                CROSS_CASES
            );
            std::io::stdout().flush().unwrap();
        }
    }

    #[must_use]
    pub fn solve(&self, cube: &crate::cube::Cube) -> Option<Vec<Move>> {
        let case = Case::from_cube(cube);
        self.solutions.get(&case).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_solutions() {
        let solver = Solver::new();
        assert_eq!(solver.solutions.len(), CROSS_CASES);
    }

    #[test]
    fn solve_3_move_cross() {
        let solver = Solver::new();

        let mut cube = cube::Cube::default();
        let scramble = cube::algorithms::parse_algorithm("R F B'");
        cube.execute_algorithm(&scramble);

        let solution = solver.solve(&cube);
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_eq!(solution.len(), 3);
    }
}
