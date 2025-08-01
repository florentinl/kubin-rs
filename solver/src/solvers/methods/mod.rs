use super::solver::Method;

pub mod cfop;
pub mod free_fop;
pub mod one_phase;
pub mod two_phase;

pub fn from_method_name(name: &str) -> Result<Methods, &'static str> {
    match name {
        "cfop" => Ok(Methods::Cfop(cfop::Solver::new())),
        "free_fop" => Ok(Methods::FreeFop(free_fop::Solver::new())),
        "one_phase" => Ok(Methods::OnePhase(one_phase::Solver::new())),
        "two_phase" => Ok(Methods::TwoPhase(two_phase::Solver::new())),
        _ => Err("Unknown method"),
    }
}

#[derive(Clone)]
pub enum Methods {
    Cfop(cfop::Solver),
    FreeFop(free_fop::Solver),
    OnePhase(one_phase::Solver),
    TwoPhase(two_phase::Solver),
}

impl Method for Methods {
    fn solve(&self, cube: &cube::Cube) -> Vec<cube::algorithms::Move> {
        match self {
            Methods::Cfop(solver) => solver.solve(cube),
            Methods::FreeFop(solver) => solver.solve(cube),
            Methods::OnePhase(solver) => solver.solve(cube),
            Methods::TwoPhase(solver) => solver.solve(cube),
        }
    }
}
