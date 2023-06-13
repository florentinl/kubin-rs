use kubin_rs::{cube::{self, algorithms}, solver, scramble};

/// Repl to run the program interactively
pub fn main() {
    let mut scrambles = vec![];
    for _ in 0..10 {
        scrambles.push(scramble::generate_scramble());
    }

    let now = std::time::Instant::now();

    // Hide the cursor
    print!("\x1B[?25l");
    let cross_solver = solver::cross::Solver::new();
    let f2l_solver = solver::f2l::Solver::new();
    let oll_solver = solver::oll::Solver::new();
    let pll_solver = solver::pll::Solver::new();
    // Show the cursor
    print!("\x1B[?25h");

    let elapsed = now.elapsed();

    println!("Initialized Solvers in {elapsed:?}");

    let mut total_time = std::time::Duration::new(0, 0);
    let mut total_moves = 0;

    for scramble in &scrambles {
        let mut cube = cube::Cube::default();
        cube.execute_algorithm(scramble);
        let now = std::time::Instant::now();
        let cross_solution = cross_solver.solve(&cube);
        cube.execute_algorithm(&cross_solution);
        let f2l_solution = f2l_solver.solve(&cube);
        cube.execute_algorithm(&f2l_solution);
        let oll_solution = oll_solver.solve(&cube);
        cube.execute_algorithm(&oll_solution);
        let pll_solution = pll_solver.solve(&cube);
        cube.execute_algorithm(&pll_solution);
        let elapsed = now.elapsed();
        // assert_eq!(cube, cube::Cube::default());
        let solution_length =
            cross_solution.len() + f2l_solution.len() + oll_solution.len() + pll_solution.len();
        total_time += elapsed;
        total_moves += solution_length;
    }

    let average_time = total_time / scrambles.len() as u32;
    let average_moves = total_moves / scrambles.len();

    println!(
        "Solved {} scrambles: in {total_time:?} / {total_moves} moves ({average_time:?} / {average_moves} moves per scramble)",
        scrambles.len()
    );

    println!("Here is the list of scrambles:");
    for scramble in &scrambles {
        println!("{}", algorithms::algorithm_to_string(scramble));
    }
}
