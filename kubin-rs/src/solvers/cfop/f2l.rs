//! # Solve F2Ls in the CFOP method using IDA* search.
//!
//! We use BFS to generate heuristics for each corner/edge case, then use IDA* to
//! solve the F2Ls. We restrict the moves to three move triggers separated by U
//! moves, so that the cross is not disturbed.

use std::{
    collections::{HashMap, VecDeque},
    io::Write,
};

use crate::solvers::{
    cube_subsets::{
        Corners, Edges, TwoBackPairsOneEdge, TwoFrontPairsOneEdge, CORNER_CASES,
        EDGE_CASES, TWO_PAIRS_ONE_EDGE_CASES,
    },
    solver::StepSolver,
};

use serde::{Deserialize, Serialize};

use crate::{
    solvers::utils::{print_bfs_progress, print_bfs_terminated},
};

use cube::{self, algorithms::{Move, self}, Cube};
use cube::subcases::CubeSubset;

#[derive(Serialize, Deserialize)]
pub struct Solver {
    trigger_algs: Vec<Vec<Move>>,
    corner_cases: HashMap<Corners, usize>,
    edge_cases: HashMap<Edges, usize>,
    two_front_pairs_cases: HashMap<TwoFrontPairsOneEdge, usize>,
    two_back_pairs_cases: HashMap<TwoBackPairsOneEdge, usize>,
}

impl Solver {
    fn generate_trigger_algs() -> Vec<Vec<Move>> {
        let mut algs = vec![];
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
                    algs.push(alg);
                }
            }
        }
        algs
    }

    fn generate_heuristic<T>(case_count: usize, name: &str) -> HashMap<T, usize>
    where
        T: CubeSubset,
    {
        let mut cases = HashMap::with_capacity(case_count);
        let mut queue = VecDeque::with_capacity(case_count);
        queue.push_back((Cube::default(), 0));

        while let Some((cube, distance)) = queue.pop_front() {
            let progress = cases.len();
            print_bfs_progress!(name, progress, case_count);

            let case = CubeSubset::from_cube(&cube);
            if cases.contains_key(&case) {
                continue;
            }

            cases.insert(case, distance);

            for alg in &Self::generate_trigger_algs() {
                let mut cube = cube.clone();
                cube.execute_algorithm(alg);
                queue.push_back((cube, distance + 4));
            }
        }
        print_bfs_terminated!(name, cases.len(), case_count);
        cases
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

    fn search(&self, cube: &mut Cube, bound: usize, path: &mut Vec<Move>) -> usize {
        let distance = self.assess_distance(cube);
        let local_lower_bound = path.len() + distance;
        if local_lower_bound > bound {
            return local_lower_bound;
        }
        if distance == 0 {
            return 0;
        }
        let mut min = usize::MAX;

        for alg in &self.trigger_algs {
            let mut cube = cube.clone();
            cube.execute_algorithm(alg);
            path.extend(alg.iter().cloned());
            let t = self.search(&mut cube, bound, path);
            if t == 0 {
                return 0;
            }
            if t < min {
                min = t;
            }
            for _ in 0..alg.len() {
                path.pop();
            }
        }
        min
    }
}

impl StepSolver for Solver {
    fn generate() -> Self {
        let corners = Self::generate_heuristic(CORNER_CASES, "F2L/Corners");
        let edges = Self::generate_heuristic(EDGE_CASES, "F2L/Edges");
        let two_pairs_front =
            Self::generate_heuristic(TWO_PAIRS_ONE_EDGE_CASES, "F2L/Two Pairs Front");
        let two_pairs_back =
            Self::generate_heuristic(TWO_PAIRS_ONE_EDGE_CASES, "F2L/Two Pairs Back");
        Self {
            trigger_algs: Self::generate_trigger_algs(),
            corner_cases: corners,
            edge_cases: edges,
            two_front_pairs_cases: two_pairs_front,
            two_back_pairs_cases: two_pairs_back,
        }
    }

    fn solve(&self, cube: &Cube) -> Vec<Move> {
        // Solve the cube using IDA* with the max of the corner and edge heuristics.
        let mut cube = cube.clone();
        let mut bound = self.assess_distance(&cube);
        let mut path = vec![];
        loop {
            let t = self.search(&mut cube, bound, &mut path);
            if t == 0 {
                return path.iter().filter(|x| **x != Move::None).cloned().collect();
            }
            if t == usize::MAX {
                return vec![];
            }
            bound = t;
        }
    }
}

#[cfg(test)]
mod tests {
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
