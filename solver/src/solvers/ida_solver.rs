use crate::solvers::utils::{print_progress, print_terminated};
use cube::{algorithms::Move, subcases::CubeSubset, Cube};
use std::io::Write;

use super::solver::Step;
use std::collections::{HashMap, VecDeque};
pub(super) trait IDAStepSolver: Step + Default {
    fn get_candidate_moves(&self, history: &[Move]) -> Vec<Move>;
    fn assess_distance(&self, cube: &Cube) -> usize;
    fn populate_candidate_moves(&mut self);
    fn populate_heuristics(&mut self);

    fn generate_heuristic<T>(&self, case_count: usize, name: &str) -> HashMap<T, usize>
    where
        T: CubeSubset,
    {
        let mut cases = HashMap::with_capacity(case_count);
        let mut queue = VecDeque::with_capacity(case_count);
        queue.push_back((Cube::default(), 0));

        while let Some((cube, distance)) = queue.pop_front() {
            let progress = cases.len();
            print_progress!("Generating lookup table for", name, progress, case_count);

            let case = CubeSubset::from_cube(&cube);
            if cases.contains_key(&case) {
                continue;
            }

            cases.insert(case, distance);

            for alg in self.get_candidate_moves(&[]) {
                let mut cube = cube.clone();
                cube.execute_move(&alg);
                queue.push_back((cube, distance + 4));
            }
        }
        print_terminated!("Generating lookup table for", name, cases.len(), case_count);
        cases
    }

    fn search(
        &self,
        cube: &mut Cube,
        bound: usize,
        path: &mut Vec<Move>,
    ) -> usize {
        let distance = self.assess_distance(cube);
        let local_lower_bound = path.len() + distance;
        if local_lower_bound > bound {
            return local_lower_bound;
        }
        if distance == 0 {
            return 0;
        }
        let mut min = usize::MAX;

        for alg in self.get_candidate_moves(&path) {
            let mut cube = cube.clone();
            cube.execute_move(&alg);
            path.push(alg);
            let t = self.search(&mut cube, bound, path);
            if t == 0 {
                return 0;
            }
            if t < min {
                min = t;
            }
            path.pop();
        }
        min
    }
}

impl<T: IDAStepSolver> Step for T {
    fn generate() -> Self {
        let mut solver = Self::default();
        solver.populate_candidate_moves();
        solver.populate_heuristics();
        solver
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
