use kubin_rs::{scramble, solvers};

use cube::{self};

/// Repl to run the program interactively
pub fn main() {
    let number_of_scrambles = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "100".to_string())
        .parse::<usize>()
        .unwrap();

    let mut scrambles = vec![];
    for _ in 0..number_of_scrambles {
        scrambles.push(scramble::generate_scramble());
    }

    let now = std::time::Instant::now();
    let solver = solvers::cfop::Solver::new();
    println!("Solver initialized in {:?}", now.elapsed());

    let mut times = vec![];
    let mut lengths = vec![];

    for (i, scramble) in scrambles.iter().enumerate() {
        if i % (scrambles.len() / 10) == 0 {
            println!("Solving scrambles: {} / {}", i, scrambles.len());
        }
        let mut cube = cube::Cube::default();
        cube.execute_algorithm(scramble);

        let now = std::time::Instant::now();
        let solution = solver.solve(&cube);
        let elapsed = now.elapsed();
        times.push(elapsed.as_millis() as u16);
        lengths.push(solution.len());
    }

    println!("Median time: {}ms", median(&mut times));
    println!(
        "Average time: {}ms",
        times.iter().sum::<u16>() / times.len() as u16
    );
    println!("Median length: {}", median(&mut lengths));
    println!(
        "Average length: {}",
        lengths.iter().sum::<usize>() / lengths.len()
    );
    println!("Worst time: {}ms", times.iter().max().unwrap());
    println!("Worst length: {}", lengths.iter().max().unwrap());
}

fn median<T: Ord + Copy>(v: &mut Vec<T>) -> T {
    v.sort();
    v[v.len() / 2]
}
