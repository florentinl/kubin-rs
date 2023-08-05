use std::collections::HashMap;

use cube::{algorithms::Move, subcases::CubeSubset};
use serde::{Deserialize, Serialize};

use crate::solvers::{
    cube_subsets::{CornerPermutation, EdgePermutation, CP_CASES, EP_CASES},
    ida_solver::IDAStepSolver,
};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Solver {
    candidate_moves: Vec<Move>,
    corner_permutation: HashMap<CornerPermutation, usize>,
    edge_permutation: HashMap<EdgePermutation, usize>,
}

impl IDAStepSolver for Solver {
    #[inline]
    fn get_all_moves(&self) -> &[Move] {
        &self.candidate_moves
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
            Move::U2,
            Move::Up,
            Move::U,
            Move::D2,
            Move::Dp,
            Move::D,
            Move::R2,
            Move::L2,
            Move::F2,
            Move::B2,
        ]);
    }

    fn populate_heuristics(&mut self) {
        self.corner_permutation = self.generate_heuristic(CP_CASES, "Permutation/Corner");
        self.edge_permutation = self.generate_heuristic(EP_CASES, "Permutation/Edge");
    }
}
