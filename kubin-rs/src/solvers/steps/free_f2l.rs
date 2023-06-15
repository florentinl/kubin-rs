use std::collections::HashMap;
use std::vec;

use cube::algorithms::Move;
use cube::subcases::CubeSubset;
use serde::{Deserialize, Serialize};

use crate::solvers::cube_subsets::BLOCK_CASES;
use crate::solvers::{
    cube_subsets::{BackLeftBlock, BackRightBlock, FrontLeftBlock, FrontRightBlock},
    ida_solver::IDAStepSolver,
    utils::generate_heuristic,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Solver {
    candidate_moves: Vec<Vec<Move>>,
    front_left_block: HashMap<FrontLeftBlock, usize>,
    front_right_block: HashMap<FrontRightBlock, usize>,
    back_left_block: HashMap<BackLeftBlock, usize>,
    back_right_block: HashMap<BackRightBlock, usize>,
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

    fn populate_candidate_moves(&mut self) {
        self.candidate_moves = cube::algorithms::ALL_MOVES
            .iter()
            .map(|x| vec![x.clone()])
            .collect();
    }

    fn populate_heuristics(&mut self) {
        let candidate_moves = &self.get_candidate_moves(&[]);
        self.front_left_block = generate_heuristic(BLOCK_CASES, "FreeF2L/FLB", candidate_moves);
        self.front_right_block = generate_heuristic(BLOCK_CASES, "FreeF2L/FRB", candidate_moves);
        self.back_left_block = generate_heuristic(BLOCK_CASES, "FreeF2L/BLB", candidate_moves);
        self.back_right_block = generate_heuristic(BLOCK_CASES, "FreeF2L/BRB", candidate_moves);
    }

    fn assess_distance(&self, cube: &cube::Cube) -> usize {
        let front_left_block = FrontLeftBlock::from_cube(cube);
        let front_right_block = FrontRightBlock::from_cube(cube);
        let back_left_block = BackLeftBlock::from_cube(cube);
        let back_right_block = BackRightBlock::from_cube(cube);

        let front_left_block_distance = self.front_left_block.get(&front_left_block).unwrap();
        let front_right_block_distance = self.front_right_block.get(&front_right_block).unwrap();
        let back_left_block_distance = self.back_left_block.get(&back_left_block).unwrap();
        let back_right_block_distance = self.back_right_block.get(&back_right_block).unwrap();

        **[
            front_left_block_distance,
            front_right_block_distance,
            back_left_block_distance,
            back_right_block_distance,
        ]
        .iter()
        .max()
        .unwrap()
    }
}
