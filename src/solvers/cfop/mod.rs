use crate::cube::algorithms::Move;

pub mod cross;
pub mod f2l;
pub mod oll;
pub mod pll;

pub struct Solver {
    cross_solver: cross::Solver,
    f2l_solver: f2l::Solver,
    oll_solver: oll::Solver,
    pll_solver: pll::Solver,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            cross_solver: cross::Solver::new(),
            f2l_solver: f2l::Solver::new(),
            oll_solver: oll::Solver::new(),
            pll_solver: pll::Solver::new(),
        }
    }

    pub fn solve(&self, cube: &crate::cube::Cube) -> Vec<Move> {
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
