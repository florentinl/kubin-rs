use cube::algorithms::{self, Move};

#[must_use]
pub fn generate() -> Vec<Move> {
    let mut scramble = Vec::with_capacity(20);

    for _ in 0..20 {
        scramble.push(algorithms::ALL_MOVES[rand::random::<usize>() % 18].clone());
    }

    scramble
}
