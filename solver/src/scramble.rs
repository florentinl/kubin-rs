use cube::algorithms::{self, Move};

#[must_use]
pub fn generate(count: usize) -> Vec<Move> {
    let mut scramble = Vec::with_capacity(20);
    let moves = algorithms::ALL_MOVES.to_vec();

    for _ in 0..count {
        let mut candidate_moves = moves.clone();
        if !scramble.is_empty() {
            let previous_move: &Move = &scramble[scramble.len() - 1];
            let same_face_moves = previous_move.same_face_moves();
            candidate_moves.retain(|m| !same_face_moves.contains(m));
        }
        if scramble.len() > 1 {
            let previous_move = &scramble[scramble.len() - 1];
            let previous_previous_move = &scramble[scramble.len() - 2];
            let opposit_face_moves = previous_move.opposite_face_moves();
            if opposit_face_moves.contains(previous_previous_move) {
                candidate_moves.retain(|x| !opposit_face_moves.contains(x));
            }
        }
        scramble.push(candidate_moves[rand::random::<usize>() % candidate_moves.len()].clone());
    }

    scramble
}
