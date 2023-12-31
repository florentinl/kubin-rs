use std::collections::HashMap;
use std::ops::Add;

use cube::algorithms::Move;
use cube::subcases::CubeSubset;
use serde::{Deserialize, Serialize};

use crate::solvers::cube_subsets::BLOCK_CASES;
use crate::solvers::{
    cube_subsets::{BackLeftBlock, BackRightBlock, FrontLeftBlock, FrontRightBlock},
    ida_solver::IDAStepSolver,
};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Solver {
    candidate_moves: Vec<Move>,
    front_left_block: HashMap<FrontLeftBlock, usize>,
    front_right_block: HashMap<FrontRightBlock, usize>,
    back_left_block: HashMap<BackLeftBlock, usize>,
    back_right_block: HashMap<BackRightBlock, usize>,
}

impl IDAStepSolver for Solver {
    #[inline]
    fn get_all_moves(&self) -> &[Move] {
        &self.candidate_moves
    }

    fn populate_candidate_moves(&mut self) {
        self.candidate_moves = cube::algorithms::ALL_MOVES.to_vec();
    }

    fn populate_heuristics(&mut self) {
        self.front_left_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/FLB");
        self.front_right_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/FRB");
        self.back_left_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/BLB");
        self.back_right_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/BRB");
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

        front_left_block_distance
            .add(front_right_block_distance)
            .add(back_left_block_distance)
            .add(back_right_block_distance)
    }
}
