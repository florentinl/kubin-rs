use cube::{algorithms::Move, Cube};

use super::solver::StepSolver;

pub(super) trait IDAStepSolver: StepSolver + Default {
    fn get_candidate_moves(&self, previous_move: Option<&Vec<Move>>) -> Vec<Vec<Move>>;
    fn assess_distance(&self, cube: &Cube) -> usize;
    fn populate_candidate_moves(&mut self);
    fn populate_heuristics(&mut self);
    fn search(&self, cube: &mut Cube, bound: usize, path: &mut Vec<Move>, previous: Option<&Vec<Move>>) -> usize {
        let distance = self.assess_distance(cube);
        let local_lower_bound = path.len() + distance;
        if local_lower_bound > bound {
            return local_lower_bound;
        }
        if distance == 0 {
            return 0;
        }
        let mut min = usize::MAX;

        for alg in self.get_candidate_moves(previous) {
            let mut cube = cube.clone();
            cube.execute_algorithm(&alg);
            path.extend(alg.iter().cloned());
            let t = self.search(&mut cube, bound, path, Some(&alg));
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

impl<T: IDAStepSolver> StepSolver for T {
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
            let t = self.search(&mut cube, bound, &mut path, None);
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
