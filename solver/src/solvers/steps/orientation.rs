use std::collections::HashMap;

use cube::{algorithms::Move, subcases::CubeSubset};
use serde::{Deserialize, Serialize};

use crate::solvers::{
    cube_subsets::{
        CornerOrientation, EdgeInSlice, EdgeOrientation, CO_CASES, EDGE_IN_SLICE_CASES, EO_CASES,
    },
    ida_solver::IDAStepSolver,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Solver {
    candidate_moves: Vec<Move>,
    edge_orientation: HashMap<EdgeOrientation, usize>,
    corner_orientation: HashMap<CornerOrientation, usize>,
    edge_in_slice: HashMap<EdgeInSlice, usize>,
}

impl IDAStepSolver for Solver {
    fn get_candidate_moves(&self, history: &[Move]) -> Vec<Move> {
        let mut candidate_moves = self.candidate_moves.clone();
        if !history.is_empty() {
            let previous_move = &history[history.len() - 1];
            let same_face_moves = previous_move.same_face_moves();
            candidate_moves.retain(|m| !same_face_moves.contains(m));
        }
        if history.len() > 1 {
            let previous_move = &history[history.len() - 1];
            let previous_previous_move = &history[history.len() - 2];
            let opposit_face_moves = previous_move.opposite_face_moves();
            if opposit_face_moves.contains(previous_previous_move) {
                candidate_moves.retain(|x| !opposit_face_moves.contains(x));
            }
        }
        candidate_moves
    }

    fn assess_distance(&self, cube: &cube::Cube) -> usize {
        let edge_orientation = EdgeOrientation::from_cube(cube);
        let corner_orientation = CornerOrientation::from_cube(cube);
        let edge_in_slice = EdgeInSlice::from_cube(cube);
        let edge_orientation_moves = self.edge_orientation.get(&edge_orientation).unwrap();
        let corner_orientation_moves = self.corner_orientation.get(&corner_orientation).unwrap();
        let edge_in_slice_moves = self.edge_in_slice.get(&edge_in_slice).unwrap();
        *edge_orientation_moves
            .max(corner_orientation_moves)
            .max(edge_in_slice_moves)
    }

    fn populate_candidate_moves(&mut self) {
        self.candidate_moves = cube::algorithms::ALL_MOVES.to_vec()
    }

    fn populate_heuristics(&mut self) {
        self.edge_orientation = self.generate_heuristic(EO_CASES, "Orientation/EO");
        self.corner_orientation = self.generate_heuristic(CO_CASES, "Orientation/CO");
        self.edge_in_slice = self.generate_heuristic(EDGE_IN_SLICE_CASES, "Orientation/EIS");
    }
}
