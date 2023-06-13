use kubin_rs::{cube, solver};

/// Repl to run the program interactively
pub fn main() {
    let mut cube = cube::Cube::default();

    let scramble = cube::algorithms::parse_algorithm(
        "B' F L' U2 L2 D' F' B2 R B R' F' U' R2 D2 U B' D' B' R' U2 L R2 F L'",
    );

    cube.execute_algorithm(&scramble);

    let cross_solver = solver::cross::Solver::new();
    let cross_solution = cross_solver.solve(&cube);
    println!("Cross solution: {cross_solution:?}");
}
