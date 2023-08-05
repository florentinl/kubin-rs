use std::collections::HashMap;

use cube::{algorithms::Move, subcases::CubeSubset};
use serde::{Deserialize, Serialize};

use crate::solvers::{
    cube_subsets::{
        CornerPermutation, Edge6_1Permutation, Edge6_2Permutation, Orientation, CP_CASES,
        E6P_CASES, ORIENTATION_CASES,
    },
    ida_solver::IDAStepSolver,
};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Solver {
    candidate_moves: Vec<Move>,
    orientation: HashMap<Orientation, usize>,
    edge_permutation_1: HashMap<Edge6_1Permutation, usize>,
    edge_permutation_2: HashMap<Edge6_2Permutation, usize>,
    corner_permutation: HashMap<CornerPermutation, usize>,
}

impl IDAStepSolver for Solver {
    #[inline]
    fn get_all_moves(&self) -> &[Move] {
        &self.candidate_moves
    }

    fn assess_distance(&self, cube: &cube::Cube) -> usize {
        let orientation = Orientation::from_cube(cube);
        let corner_permutation = CornerPermutation::from_cube(cube);
        let edge_permutation_1 = Edge6_1Permutation::from_cube(cube);
        let edge_permutation_2 = Edge6_2Permutation::from_cube(cube);

        let orientation_moves = self.orientation.get(&orientation).unwrap();
        let corner_permutation_moves = self.corner_permutation.get(&corner_permutation).unwrap();
        let edge_permutation_moves = self
            .edge_permutation_1
            .get(&edge_permutation_1)
            .unwrap()
            .max(self.edge_permutation_2.get(&edge_permutation_2).unwrap());

        *orientation_moves
            .max(corner_permutation_moves)
            .max(edge_permutation_moves)
    }

    fn populate_candidate_moves(&mut self) {
        self.candidate_moves = cube::algorithms::ALL_MOVES.to_vec();
    }

    fn populate_heuristics(&mut self) {
        self.orientation = self.generate_heuristic(ORIENTATION_CASES, "Orientation");
        self.corner_permutation = self.generate_heuristic(CP_CASES, "Corner Permutation");
        self.edge_permutation_1 = self.generate_heuristic(E6P_CASES, "Edge Permutation/EP1");
        self.edge_permutation_2 = self.generate_heuristic(E6P_CASES, "Edge Permutation/EP2");
    }
}
