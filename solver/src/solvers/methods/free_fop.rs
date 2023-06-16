use cube::{algorithms::Move, Cube};

use crate::solvers::{
    solver::{Method, Step},
    steps::{free_f2l, oll, pll},
};

#[derive(Clone)]
pub struct Solver {
    free_f2l_solver: free_f2l::Solver,
    oll_solver: oll::Solver,
    pll_solver: pll::Solver,
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
            free_f2l_solver: free_f2l::Solver::new("/tmp/free_f2l_solver.ron"),
            oll_solver: oll::Solver::new("/tmp/oll_solver.ron"),
            pll_solver: pll::Solver::new("/tmp/pll_solver.ron"),
        }
    }
}

impl Method for Solver {
    fn solve(&self, cube: &Cube) -> Vec<Move> {
        let mut cube = cube.clone();

        let free_f2l_solution = self.free_f2l_solver.solve(&cube);
        cube.execute_algorithm(&free_f2l_solution);

        let oll_solution = self.oll_solver.solve(&cube);
        cube.execute_algorithm(&oll_solution);

        let pll_solution = self.pll_solver.solve(&cube);
        cube.execute_algorithm(&pll_solution);

        let mut solution = free_f2l_solution;
        solution.extend(oll_solution);
        solution.extend(pll_solution);
        solution
    }
}
