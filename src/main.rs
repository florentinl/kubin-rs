use kubin_rs::{cube, solver};

/// Repl to run the program interactively
pub fn main() {
    let mut cube = cube::Cube::default();

    let scramble = cube::algorithms::parse_algorithm(
        "B' F L' U2 L2 D' F' B2 R B R' F' U' R2 D2 U B' D' B' R' U2 L R2 F L'",
    );

    cube.execute_algorithm(&scramble);

    let cross_solver = solver::cross::Solver::new();
    let f2l_solver = solver::f2l::Solver::new();

    let cross_solution = cross_solver.solve(&cube).unwrap();
    cube.execute_algorithm(&cross_solution);

    let f2l_solution = f2l_solver.solve(&cube);
    cube.execute_algorithm(&f2l_solution);

    println!("Cross solution: {cross_solution:?}");
    println!("F2L solution: {f2l_solution:?}");
}
