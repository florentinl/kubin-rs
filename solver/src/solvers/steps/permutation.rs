use std::collections::HashMap;

use cube::{algorithms::Move, subcases::CubeSubset};
use serde::{Deserialize, Serialize};

use crate::solvers::{
    cube_subsets::{CornerPermutation, EdgePermutation, CP_CASES, EP_CASES},
    ida_solver::IDAStepSolver,
    utils::generate_heuristic,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Solver {
    candidate_moves: Vec<Vec<Move>>,
    corner_permutation: HashMap<CornerPermutation, usize>,
    edge_permutation: HashMap<EdgePermutation, usize>,
}

impl IDAStepSolver for Solver {
    fn get_candidate_moves(&self, history: &[Vec<Move>]) -> Vec<Vec<Move>> {
        let mut candidate_moves = self.candidate_moves.clone();
        if !history.is_empty() {
            let previous_move = &history[history.len() - 1];
            let same_face_moves = previous_move.get(0).unwrap().same_face_moves();
            candidate_moves.retain(|x| !same_face_moves.contains(x.get(0).unwrap()));
        }
        if history.len() > 1 {
            let previous_move = &history[history.len() - 1].get(0).unwrap();
            let previous_previous_move = &history[history.len() - 2].get(0).unwrap();
            let opposit_face_moves = previous_move.opposite_face_moves();
            if opposit_face_moves.contains(previous_previous_move) {
                candidate_moves.retain(|x| !opposit_face_moves.contains(x.get(0).unwrap()));
            }
        }
        candidate_moves
    }

    fn assess_distance(&self, cube: &cube::Cube) -> usize {
        let corner_permutation = CornerPermutation::from_cube(cube);
        let edge_permutation = EdgePermutation::from_cube(cube);

        let corner_permutation_moves = self.corner_permutation.get(&corner_permutation).unwrap();
        let edge_permutation_moves = self.edge_permutation.get(&edge_permutation).unwrap();

        *corner_permutation_moves.max(edge_permutation_moves)
    }

    fn populate_candidate_moves(&mut self) {
        self.candidate_moves.extend(vec![
            vec![Move::U2],
            vec![Move::Up],
            vec![Move::U],
            vec![Move::D2],
            vec![Move::Dp],
            vec![Move::D],
            vec![Move::R2],
            vec![Move::L2],
            vec![Move::F2],
            vec![Move::B2],
        ]);
    }

    fn populate_heuristics(&mut self) {
        let candidate_moves = &self.get_candidate_moves(&[]);
        self.corner_permutation =
            generate_heuristic(CP_CASES, "Permutation/Corner", candidate_moves);
        self.edge_permutation = generate_heuristic(EP_CASES, "Permutation/Edge", candidate_moves);
    }
}
