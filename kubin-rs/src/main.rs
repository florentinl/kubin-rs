use kubin_rs::{
    scramble,
    solvers::{self},
};

use cube::{self};

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

    // let now = std::time::Instant::now();
    // let cfop = solvers::methods::cfop::Solver::new();
    // println!("CFOP Solver initialized in {:?}", now.elapsed());

    let now = std::time::Instant::now();
    let two_phase = solvers::methods::two_phase::Solver::new();
    println!("GroupReduction Solver initialized in {:?}", now.elapsed());

    // let mut cfop_times = vec![];
    // let mut cfop_lengths = vec![];
    let mut group_reduction_times = vec![];
    let mut group_reduction_lengths = vec![];

    for (i, scramble) in scrambles.iter().enumerate() {
        if i % (scrambles.len() / 10) == 0 {
            println!("Solving scrambles: {} / {}", i, scrambles.len());
        }
        let mut cube = cube::Cube::default();
        cube.execute_algorithm(scramble);
        // let cube_cfop = cube.clone();
        let cube_free_fop = cube.clone();

        // let now = std::time::Instant::now();
        // let cfop_solution = cfop.solve(&cube_cfop);
        // let elapsed = now.elapsed();

        let now = std::time::Instant::now();
        let two_phase_solution = two_phase.solve(&cube_free_fop);
        let elapsed2 = now.elapsed();

        // println!("Scramble: {:?}", scramble);
        // // println!("CFOP solution: {:?}", cfop_solution);
        // println!("FreeFOP solution: {:?}", free_fop_solution);

        // cfop_times.push(elapsed.as_millis() as u16);
        group_reduction_times.push(elapsed2.as_millis() as u16);

        // cfop_lengths.push(cfop_solution.len());
        group_reduction_lengths.push(two_phase_solution.len());
    }

    // println!("CFOP:");
    // println!("Median time: {}ms", median(&mut cfop_times));
    // println!(
    //     "Average time: {}ms",
    //     cfop_times.iter().sum::<u16>() / cfop_times.len() as u16
    // );
    // println!("Median length: {}", median(&mut cfop_lengths));
    // println!(
    //     "Average length: {}",
    //     cfop_lengths.iter().sum::<usize>() / cfop_lengths.len()
    // );
    // println!("Worst time: {}ms", cfop_times.iter().max().unwrap());
    // println!("Worst length: {}", cfop_lengths.iter().max().unwrap());

    println!("TwoPhase:");
    println!("Median time: {}ms", median(&mut group_reduction_times));
    println!(
        "Average time: {}ms",
        group_reduction_times.iter().sum::<u16>() / group_reduction_times.len() as u16
    );
    println!("Median length: {}", median(&mut group_reduction_lengths));
    println!(
        "Average length: {}",
        group_reduction_lengths.iter().sum::<usize>() / group_reduction_lengths.len()
    );
    println!(
        "Worst time: {}ms",
        group_reduction_times.iter().max().unwrap()
    );
    println!(
        "Worst length: {}",
        group_reduction_lengths.iter().max().unwrap()
    );
}

fn median<T: Ord + Copy>(v: &mut Vec<T>) -> T {
    v.sort();
    v[v.len() / 2]
}
