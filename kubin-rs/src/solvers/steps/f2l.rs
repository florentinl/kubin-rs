//! # Solve F2Ls in the CFOP method using IDA* search.
//!
//! We use BFS to generate heuristics for each corner/edge case, then use IDA* to
//! solve the F2Ls. We restrict the moves to three move triggers separated by U
//! moves, so that the cross is not disturbed.

use std::collections::HashMap;

use crate::solvers::{
    cube_subsets::{
        Corners, Edges, TwoBackPairsOneEdge, TwoFrontPairsOneEdge, CORNER_CASES, EDGE_CASES,
        TWO_PAIRS_ONE_EDGE_CASES,
    },
    ida_solver::IDAStepSolver,
    utils::generate_heuristic,
};

use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;
use cube::{
    self,
    algorithms::{self, Move},
    Cube,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Solver {
    candidate_moves: Vec<Vec<Move>>,
    corner_cases: HashMap<Corners, usize>,
    edge_cases: HashMap<Edges, usize>,
    two_front_pairs_cases: HashMap<TwoFrontPairsOneEdge, usize>,
    two_back_pairs_cases: HashMap<TwoBackPairsOneEdge, usize>,
}

impl IDAStepSolver for Solver {
    fn get_candidate_moves(&self,  _history: &Vec<Vec<Move>>) -> Vec<Vec<Move>> {
        self.candidate_moves.clone()
    }

    fn populate_candidate_moves(&mut self) {
        for pre_u_move in [Move::U, Move::Up, Move::U2, Move::None].iter() {
            for move_ in [
                Move::R,
                Move::Rp,
                Move::L,
                Move::Lp,
                Move::F,
                Move::Fp,
                Move::B,
                Move::Bp,
            ]
            .iter()
            {
                for u_move in [Move::U, Move::Up, Move::U2].iter() {
                    let alg = if matches!(pre_u_move, Move::None) {
                        vec![
                            move_.clone(),
                            u_move.clone(),
                            algorithms::invert_move(move_),
                        ]
                    } else {
                        vec![
                            pre_u_move.clone(),
                            move_.clone(),
                            u_move.clone(),
                            algorithms::invert_move(move_),
                        ]
                    };
                    self.candidate_moves.push(alg);
                }
            }
        }
    }

    fn populate_heuristics(&mut self) {
        self.corner_cases = generate_heuristic(CORNER_CASES, "F2L/Corners", &self.candidate_moves);
        self.edge_cases = generate_heuristic(EDGE_CASES, "F2L/Edges", &self.candidate_moves);
        self.two_front_pairs_cases = generate_heuristic(
            TWO_PAIRS_ONE_EDGE_CASES,
            "F2L/TwoFrontPairsOneEdge",
            &self.candidate_moves,
        );
        self.two_back_pairs_cases = generate_heuristic(
            TWO_PAIRS_ONE_EDGE_CASES,
            "F2L/TwoBackPairsOneEdge",
            &self.candidate_moves,
        );
    }

    fn assess_distance(&self, cube: &Cube) -> usize {
        // Assess the distance of the cube from the solved state.
        *self
            .corner_cases
            .get(&Corners::from_cube(cube))
            .unwrap()
            .max(self.edge_cases.get(&Edges::from_cube(cube)).unwrap())
            .max(
                self.two_front_pairs_cases
                    .get(&TwoFrontPairsOneEdge::from_cube(cube))
                    .unwrap(),
            )
            .max(
                self.two_back_pairs_cases
                    .get(&TwoBackPairsOneEdge::from_cube(cube))
                    .unwrap(),
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::solvers::solver::StepSolver;

    use super::*;

    #[test]
    fn test_generate_all_cases() {
        let solver = Solver::generate();
        assert_eq!(solver.edge_cases.len(), EDGE_CASES);
        assert_eq!(solver.corner_cases.len(), CORNER_CASES);
    }

    #[test]
    fn test_solving_f2l() {
        let solver = Solver::generate();
        let mut cube = Cube::default();
        let scramble = algorithms::parse_algorithm("R U R' U' R U2 R'");
        cube.execute_algorithm(&scramble);
        let solution = solver.solve(&cube);
        cube.execute_algorithm(&solution);
        assert_eq!(solver.assess_distance(&cube), 0);
    }
}
