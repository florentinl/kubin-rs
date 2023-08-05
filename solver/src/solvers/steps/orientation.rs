use std::collections::HashMap;

use cube::{algorithms::Move, subcases::CubeSubset};
use serde::{Deserialize, Serialize};

use crate::solvers::{
    cube_subsets::{EdgeInSlice, Orientation, EDGE_IN_SLICE_CASES, ORIENTATION_CASES},
    ida_solver::IDAStepSolver,
};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Solver {
    candidate_moves: Vec<Move>,
    orientation: HashMap<Orientation, usize>,
    edge_in_slice: HashMap<EdgeInSlice, usize>,
}

impl IDAStepSolver for Solver {
    #[inline]
    fn get_all_moves(&self) -> &[Move] {
        &self.candidate_moves
    }

    fn assess_distance(&self, cube: &cube::Cube) -> usize {
        let orientation = Orientation::from_cube(cube);
        let edge_in_slice = EdgeInSlice::from_cube(cube);
        let orientation_moves = self.orientation.get(&orientation).unwrap();
        let edge_in_slice_moves = self.edge_in_slice.get(&edge_in_slice).unwrap();
        *orientation_moves.max(edge_in_slice_moves)
    }

    fn populate_candidate_moves(&mut self) {
        self.candidate_moves = cube::algorithms::ALL_MOVES.to_vec();
    }

    fn populate_heuristics(&mut self) {
        self.orientation = self.generate_heuristic(ORIENTATION_CASES, "Orientation/EO");
        self.edge_in_slice = self.generate_heuristic(EDGE_IN_SLICE_CASES, "Orientation/EIS");
    }
}
