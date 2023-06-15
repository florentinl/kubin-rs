use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Move {
    U,
    U2,
    Up,
    D,
    D2,
    Dp,
    F,
    F2,
    Fp,
    B,
    B2,
    Bp,
    R,
    R2,
    Rp,
    L,
    L2,
    Lp,
    None,
}

impl Move {
    pub fn same_face_moves(&self) -> Vec<Move> {
        match self {
            Move::U | Move::U2 | Move::Up => vec![Move::U, Move::U2, Move::Up],
            Move::D | Move::D2 | Move::Dp => vec![Move::D, Move::D2, Move::Dp],
            Move::F | Move::F2 | Move::Fp => vec![Move::F, Move::F2, Move::Fp],
            Move::B | Move::B2 | Move::Bp => vec![Move::B, Move::B2, Move::Bp],
            Move::R | Move::R2 | Move::Rp => vec![Move::R, Move::R2, Move::Rp],
            Move::L | Move::L2 | Move::Lp => vec![Move::L, Move::L2, Move::Lp],
            Move::None => vec![Move::None],
        }
    }

    pub fn opposite_face_moves(&self) -> Vec<Move> {
        match self {
            Move::U | Move::U2 | Move::Up => vec![Move::D, Move::D2, Move::Dp],
            Move::D | Move::D2 | Move::Dp => vec![Move::U, Move::U2, Move::Up],
            Move::F | Move::F2 | Move::Fp => vec![Move::B, Move::B2, Move::Bp],
            Move::B | Move::B2 | Move::Bp => vec![Move::F, Move::F2, Move::Fp],
            Move::R | Move::R2 | Move::Rp => vec![Move::L, Move::L2, Move::Lp],
            Move::L | Move::L2 | Move::Lp => vec![Move::R, Move::R2, Move::Rp],
            Move::None => vec![Move::None],
        }
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        match self {
            Move::U => "U".to_string(),
            Move::U2 => "U2".to_string(),
            Move::Up => "U'".to_string(),
            Move::D => "D".to_string(),
            Move::D2 => "D2".to_string(),
            Move::Dp => "D'".to_string(),
            Move::F => "F".to_string(),
            Move::F2 => "F2".to_string(),
            Move::Fp => "F'".to_string(),
            Move::B => "B".to_string(),
            Move::B2 => "B2".to_string(),
            Move::Bp => "B'".to_string(),
            Move::R => "R".to_string(),
            Move::R2 => "R2".to_string(),
            Move::Rp => "R'".to_string(),
            Move::L => "L".to_string(),
            Move::L2 => "L2".to_string(),
            Move::Lp => "L'".to_string(),
            Move::None => "".to_string(),
        }
    }
}

pub const ALL_MOVES: [Move; 18] = [
    Move::U,
    Move::U2,
    Move::Up,
    Move::D,
    Move::D2,
    Move::Dp,
    Move::F,
    Move::F2,
    Move::Fp,
    Move::B,
    Move::B2,
    Move::Bp,
    Move::R,
    Move::R2,
    Move::Rp,
    Move::L,
    Move::L2,
    Move::Lp,
];

pub fn invert_move(move_: &Move) -> Move {
    match move_ {
        Move::U => Move::Up,
        Move::U2 => Move::U2,
        Move::Up => Move::U,
        Move::D => Move::Dp,
        Move::D2 => Move::D2,
        Move::Dp => Move::D,
        Move::F => Move::Fp,
        Move::F2 => Move::F2,
        Move::Fp => Move::F,
        Move::B => Move::Bp,
        Move::B2 => Move::B2,
        Move::Bp => Move::B,
        Move::R => Move::Rp,
        Move::R2 => Move::R2,
        Move::Rp => Move::R,
        Move::L => Move::Lp,
        Move::L2 => Move::L2,
        Move::Lp => Move::L,
        Move::None => Move::None,
    }
}

pub fn invert_algorithm(algorithm: &[Move]) -> Vec<Move> {
    let mut inverted_algorithm = Vec::with_capacity(algorithm.len());
    for move_ in algorithm.iter().rev() {
        inverted_algorithm.push(invert_move(move_));
    }
    inverted_algorithm
}

pub fn parse_algorithm(algorithm: &str) -> Vec<Move> {
    let mut parsed_algorithm = Vec::with_capacity(algorithm.len());
    let mut chars = algorithm.chars();
    while let Some(c) = chars.next() {
        match c {
            'U' => match chars.next() {
                Some('2') => parsed_algorithm.push(Move::U2),
                Some('\'') => parsed_algorithm.push(Move::Up),
                _ => parsed_algorithm.push(Move::U),
            },
            'D' => match chars.next() {
                Some('2') => parsed_algorithm.push(Move::D2),
                Some('\'') => parsed_algorithm.push(Move::Dp),
                _ => parsed_algorithm.push(Move::D),
            },
            'F' => match chars.next() {
                Some('2') => parsed_algorithm.push(Move::F2),
                Some('\'') => parsed_algorithm.push(Move::Fp),
                _ => parsed_algorithm.push(Move::F),
            },
            'B' => match chars.next() {
                Some('2') => parsed_algorithm.push(Move::B2),
                Some('\'') => parsed_algorithm.push(Move::Bp),
                _ => parsed_algorithm.push(Move::B),
            },
            'R' => match chars.next() {
                Some('2') => parsed_algorithm.push(Move::R2),
                Some('\'') => parsed_algorithm.push(Move::Rp),
                _ => parsed_algorithm.push(Move::R),
            },
            'L' => match chars.next() {
                Some('2') => parsed_algorithm.push(Move::L2),
                Some('\'') => parsed_algorithm.push(Move::Lp),
                _ => parsed_algorithm.push(Move::L),
            },
            _ => {}
        }
    }
    parsed_algorithm
}

pub fn algorithm_to_string(alg: &[Move]) -> String {
    alg.iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_move() {
        assert_eq!(invert_move(&Move::U), Move::Up);
        assert_eq!(invert_move(&Move::U2), Move::U2);
        assert_eq!(invert_move(&Move::Up), Move::U);
        assert_eq!(invert_move(&Move::D), Move::Dp);
        assert_eq!(invert_move(&Move::D2), Move::D2);
        assert_eq!(invert_move(&Move::Dp), Move::D);
        assert_eq!(invert_move(&Move::F), Move::Fp);
        assert_eq!(invert_move(&Move::F2), Move::F2);
        assert_eq!(invert_move(&Move::Fp), Move::F);
        assert_eq!(invert_move(&Move::B), Move::Bp);
        assert_eq!(invert_move(&Move::B2), Move::B2);
        assert_eq!(invert_move(&Move::Bp), Move::B);
        assert_eq!(invert_move(&Move::R), Move::Rp);
        assert_eq!(invert_move(&Move::R2), Move::R2);
        assert_eq!(invert_move(&Move::Rp), Move::R);
        assert_eq!(invert_move(&Move::L), Move::Lp);
        assert_eq!(invert_move(&Move::L2), Move::L2);
        assert_eq!(invert_move(&Move::Lp), Move::L);
    }

    #[test]
    fn test_invert_algorithm() {
        assert_eq!(
            invert_algorithm(&parse_algorithm("U2 D' F2 B' R2 L'")),
            parse_algorithm("L R2 B F2 D U2")
        );
    }

    #[test]
    fn test_parse_algorithm() {
        assert_eq!(
            parse_algorithm("U2 D' F2 B' R2 L'"),
            vec![Move::U2, Move::Dp, Move::F2, Move::Bp, Move::R2, Move::Lp]
        );
    }
}
