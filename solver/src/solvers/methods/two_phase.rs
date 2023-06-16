use cube::{algorithms::Move, Cube};

use crate::solvers::{
    solver::{Method, Step},
    steps::{orientation, permutation},
};

#[derive(Clone)]
pub struct Solver {
    orientation_solver: orientation::Solver,
    permutation_solver: permutation::Solver,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    #[must_use]
    pub fn new() -> Self {
        Self {
            orientation_solver: orientation::Solver::new("/tmp/orientation_solver.ron"),
            permutation_solver: permutation::Solver::new("/tmp/permutation_solver.ron"),
        }
    }
}

impl Method for Solver {
    fn solve(&self, cube: &Cube) -> Vec<Move> {
        let mut cube = cube.clone();

        let orientation_solution = self.orientation_solver.solve(&cube);
        cube.execute_algorithm(&orientation_solution);

        let permutation_solution = self.permutation_solver.solve(&cube);
        cube.execute_algorithm(&permutation_solution);

        assert_eq!(cube, Cube::default());

        let mut solution = orientation_solution;
        solution.extend(permutation_solution);
        solution
    }
}
