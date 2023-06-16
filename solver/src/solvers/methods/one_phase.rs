use cube::{algorithms::Move, Cube};

use crate::solvers::{
    solver::{Method, Step},
    steps::all,
};

#[derive(Clone)]
pub struct Solver {
    all_solver: all::Solver,
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
            all_solver: all::Solver::new("/tmp/all_solver.ron"),
        }
    }
}

impl Method for Solver {
    fn solve(&self, cube: &Cube) -> Vec<Move> {
        let mut cube = cube.clone();

        let solution = self.all_solver.solve(&cube);

        cube.execute_algorithm(&solution);

        assert_eq!(cube, Cube::default());

        solution
    }
}
