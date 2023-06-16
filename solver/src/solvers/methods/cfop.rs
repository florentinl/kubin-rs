use cube::{algorithms::Move, Cube};

use crate::solvers::{
    solver::{Method, Step},
    steps::{cross, f2l, oll, pll},
};

#[derive(Clone)]
pub struct Solver {
    cross_solver: cross::Solver,
    f2l_solver: f2l::Solver,
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
            cross_solver: cross::Solver::new("/tmp/cross_solver.ron"),
            f2l_solver: f2l::Solver::new("/tmp/f2l_solver.ron"),
            oll_solver: oll::Solver::new("/tmp/oll_solver.ron"),
            pll_solver: pll::Solver::new("/tmp/pll_solver.ron"),
        }
    }
}

impl Method for Solver {
    fn solve(&self, cube: &Cube) -> Vec<Move> {
        let mut cube = cube.clone();

        let cross_solution = self.cross_solver.solve(&cube);
        cube.execute_algorithm(&cross_solution);

        let f2l_solution = self.f2l_solver.solve(&cube);
        cube.execute_algorithm(&f2l_solution);

        let oll_solution = self.oll_solver.solve(&cube);
        cube.execute_algorithm(&oll_solution);

        let pll_solution = self.pll_solver.solve(&cube);
        cube.execute_algorithm(&pll_solution);

        let mut solution = cross_solution;
        solution.extend(f2l_solution);
        solution.extend(oll_solution);
        solution.extend(pll_solution);
        solution
    }
}
