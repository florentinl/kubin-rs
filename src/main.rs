use std::io::Write;

use kubin_rs::cube;

/// Repl to run the program interactively
pub fn main() {
    let mut cube = cube::Cube::default();
    let mut input = String::new();
    loop {
        print!("Enter an algorithm: ");
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let algorithm = cube::algorithms::parse_algorithm(input);
        cube.execute_algorithm(&algorithm);
        println!("{cube:?}");
    }
}
