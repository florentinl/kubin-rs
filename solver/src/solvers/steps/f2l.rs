//! # Solve F2Ls in the CFOP method using IDA* search.
//!
//! We use BFS to generate heuristics for each corner/edge case, then use IDA* to
//! solve the F2Ls. We restrict the moves to three move triggers separated by U
//! moves, so that the cross is not disturbed.

use std::{collections::HashMap, ops::Add};

use crate::solvers::{
    cube_subsets::{
        BackLeftBlock, BackRightBlock, Cross, FrontLeftBlock, FrontRightBlock, BLOCK_CASES,
        CROSS_CASES,
    },
    ida_solver::IDAStepSolver,
};

use serde::{Deserialize, Serialize};

use cube::subcases::CubeSubset;
use cube::{self, algorithms::Move, Cube};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Solver {
    candidate_moves: Vec<Move>,
    cross_cases: HashMap<Cross, usize>,
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
        self.cross_cases = self.generate_heuristic(CROSS_CASES, "Cross");
        self.front_left_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/FLB");
        self.front_right_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/FRB");
        self.back_left_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/BLB");
        self.back_right_block = self.generate_heuristic(BLOCK_CASES, "FreeF2L/BRB");
    }

    fn assess_distance(&self, cube: &Cube) -> usize {
        // Assess the distance of the cube from the solved state.
        let cross = Cross::from_cube(cube);
        let front_left_block = FrontLeftBlock::from_cube(cube);
        let front_right_block = FrontRightBlock::from_cube(cube);
        let back_left_block = BackLeftBlock::from_cube(cube);
        let back_right_block = BackRightBlock::from_cube(cube);

        let cross_distance = self.cross_cases.get(&cross).unwrap();
        let front_left_block_distance = self.front_left_block.get(&front_left_block).unwrap();
        let front_right_block_distance = self.front_right_block.get(&front_right_block).unwrap();
        let back_left_block_distance = self.back_left_block.get(&back_left_block).unwrap();
        let back_right_block_distance = self.back_right_block.get(&back_right_block).unwrap();

        cross_distance
            .add(front_left_block_distance)
            .add(front_right_block_distance)
            .add(back_left_block_distance)
            .add(back_right_block_distance)
    }
}
