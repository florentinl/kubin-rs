use kubin_rs::{cube, solver};

/// Repl to run the program interactively
pub fn main() {
    let mut cube = cube::Cube::default();

    let scramble = cube::algorithms::parse_algorithm(
        "B2 U B2 F R2 B2 F R F' R' D' L D' U' F' U2 B L R2 D2 U2 R F B' L",
    );

    cube.execute_algorithm(&scramble);

    let now = std::time::Instant::now();

    let cross_solver = solver::cross::Solver::new();
    let f2l_solver = solver::f2l::Solver::new();
    let oll_solver = solver::oll::Solver::new();
    let pll_solver = solver::pll::Solver::new();

    let elapsed = now.elapsed();

    println!("Initialized Solvers in {elapsed:?}");

    let global_now = std::time::Instant::now();
    let now = std::time::Instant::now();

    let cross_solution = cross_solver.solve(&cube).unwrap();
    cube.execute_algorithm(&cross_solution);

    let cross_elapsed = now.elapsed();
    let now = std::time::Instant::now();

    let f2l_solution = f2l_solver.solve(&cube);
    cube.execute_algorithm(&f2l_solution);

    let f2l_elapsed = now.elapsed();
    let now = std::time::Instant::now();

    let oll_solution = oll_solver.solve(&cube);
    cube.execute_algorithm(&oll_solution);

    let oll_elapsed = now.elapsed();
    let now = std::time::Instant::now();

    let pll_solution = pll_solver.solve(&cube);
    cube.execute_algorithm(&pll_solution);

    let pll_elapsed = now.elapsed();
    let elapsed = global_now.elapsed();

    let solution_length =
        cross_solution.len() + f2l_solution.len() + oll_solution.len() + pll_solution.len();

    println!("Cross solution: {cross_solution:?} in {cross_elapsed:?}");
    println!("F2L solution: {f2l_solution:?} in {f2l_elapsed:?}");
    println!("OLL solution: {oll_solution:?} in {oll_elapsed:?}");
    println!("PLL solution: {pll_solution:?} in {pll_elapsed:?}");
    println!("Found {solution_length} move solution in {elapsed:?}")
}
