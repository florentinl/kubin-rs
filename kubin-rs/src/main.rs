use std::sync::{Arc, Mutex};

use kubin_rs::{
    scramble,
    solvers::{self, solver::MethodSolver},
};

use cube::{self};

pub fn main() {
    let number_of_scrambles = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "100".to_string())
        .parse::<usize>()
        .unwrap();

    let number_of_threads = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "1".to_string())
        .parse::<usize>()
        .unwrap();

    let scrambles_per_thread = number_of_scrambles / number_of_threads;

    let method_times = Arc::new(Mutex::new(vec![]));
    let method_lengths = Arc::new(Mutex::new(vec![]));

    let mut handlers = vec![];

    for _ in 0..number_of_threads {
        let solver = solvers::methods::free_fop::Solver::new();
        let method_times = method_times.clone();
        let method_lengths = method_lengths.clone();
        let handler = std::thread::spawn(move || {
            let (times, lengths) = solve_n_scrambles(scrambles_per_thread, solver);
            method_times.lock().unwrap().extend(times);
            method_lengths.lock().unwrap().extend(lengths);
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    let mut method_times = method_times.lock().unwrap().clone();
    let mut method_lengths = method_lengths.lock().unwrap().clone();

    println!("Results:");
    println!("Median time: {}ms", median(&mut method_times));
    println!(
        "Average time: {}ms",
        method_times.iter().sum::<u16>() / method_times.len() as u16
    );
    println!("Median length: {}", median(&mut method_lengths));
    println!(
        "Average length: {}",
        method_lengths.iter().sum::<usize>() / method_lengths.len()
    );
    println!("Worst time: {}ms", method_times.iter().max().unwrap());
    println!("Worst length: {}", method_lengths.iter().max().unwrap());
}

fn solve_n_scrambles(count: usize, solver: impl MethodSolver) -> (Vec<u16>, Vec<usize>) {
    let mut times = vec![];
    let mut lengths = vec![];

    for _ in 0..count {
        let scramble = scramble::generate_scramble();
        let mut cube = cube::Cube::default();
        cube.execute_algorithm(&scramble);

        let now = std::time::Instant::now();
        let solution = solver.solve(&cube);
        let elapsed = now.elapsed();

        times.push(elapsed.as_millis() as u16);
        lengths.push(solution.len());
    }

    (times, lengths)
}

fn median<T: Ord + Copy>(v: &mut Vec<T>) -> T {
    v.sort();
    v[v.len() / 2]
}
