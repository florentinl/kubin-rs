use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use solver::{
    scramble,
    solvers::{methods::from_method_name, solver::Method},
};

use cube::{self};

const MOVE_COUNT: usize = 20;

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!(
            "Usage: {} <solver> <number of scrambles> [number of threads]",
            args[0]
        );
        return;
    }

    let solver = from_method_name(&args[1].clone())
        .expect("The solver must be one of cfop|free_fop|one_phase|two_phase");

    let number_of_scrambles = args[2]
        .parse::<usize>()
        .expect("Number of scrambles must be a number");

    let number_of_threads = args.get(3).map_or_else(
        || {
            std::thread::available_parallelism()
                .expect("Failed to get number of threads")
                .into()
        },
        |arg| {
            arg.parse::<usize>()
                .expect("Number of threads must be a number")
        },
    );

    println!("Solving {number_of_scrambles} scrambles");
    println!("Using {number_of_threads} threads");

    let scrambles_per_thread = number_of_scrambles / number_of_threads;

    let method_times = Arc::new(Mutex::new(vec![]));
    let method_lengths = Arc::new(Mutex::new(vec![]));

    let mut handlers = vec![];

    for _ in 0..number_of_threads {
        let solver = solver.clone();
        let method_times = method_times.clone();
        let method_lengths = method_lengths.clone();
        let handler = std::thread::spawn(move || {
            let (times, lengths) = solve_n_scrambles(scrambles_per_thread, &solver);
            method_times.lock().unwrap().extend(times);
            method_lengths.lock().unwrap().extend(lengths);
        });
        handlers.push(handler);
    }

    println!("All threads spawned");

    for handler in handlers {
        handler.join().unwrap();
    }

    let mut method_times = method_times.lock().unwrap().clone();
    let mut method_lengths = method_lengths.lock().unwrap().clone();

    println!("Results:");
    println!("Median time: {:?}", median(&mut method_times));
    println!(
        "Average time: {:?}",
        method_times.iter().sum::<Duration>() / method_times.len().try_into().unwrap()
    );
    println!("Median length: {}", median(&mut method_lengths));
    println!(
        "Average length: {}",
        method_lengths.iter().sum::<usize>() / method_lengths.len()
    );
    println!(
        "95th percentile time: {:?}",
        ninety_five_percentile(&mut method_times)
    );
    println!(
        "95th percentile length: {}",
        ninety_five_percentile(&mut method_lengths)
    );
    println!("Worst time: {:?}", method_times.iter().max().unwrap());
    println!("Worst length: {}", method_lengths.iter().max().unwrap());
}

fn solve_n_scrambles(count: usize, solver: &impl Method) -> (Vec<Duration>, Vec<usize>) {
    let mut times = vec![];
    let mut lengths = vec![];

    for _ in 0..count {
        let scramble = scramble::generate(MOVE_COUNT);
        let mut cube = cube::Cube::default();
        cube.execute_algorithm(&scramble);

        let now = std::time::Instant::now();
        let solution = solver.solve(&cube);
        let elapsed = now.elapsed();

        times.push(elapsed);
        lengths.push(solution.len());
    }

    (times, lengths)
}

fn median<T: Ord + Copy>(v: &mut Vec<T>) -> T {
    v.sort();
    v[v.len() / 2]
}

fn ninety_five_percentile<T: Ord + Copy>(v: &mut Vec<T>) -> T {
    v.sort();
    v[v.len() * 95 / 100]
}
