use kubin_rs::{
    cube::{self},
    scramble, solvers,
};

/// Repl to run the program interactively
pub fn main() {
    let mut scrambles = vec![];
    for _ in 0..1000 {
        scrambles.push(scramble::generate_scramble());
    }

    let now = std::time::Instant::now();
    let solver = solvers::cfop::Solver::new();
    println!("Solver created in {:?}", now.elapsed());

    let mut times = vec![];
    let mut lengths = vec![];

    for (i, scramble) in scrambles.iter().enumerate() {
        if i % 100 == 0 {
            println!("{} / {}", i, scrambles.len());
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
