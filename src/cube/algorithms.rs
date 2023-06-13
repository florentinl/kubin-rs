#[derive(Debug, PartialEq)]
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
}

#[must_use] pub fn invert_move(move_: &Move) -> Move {
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
    }
}

#[must_use] pub fn invert_algorithm(algorithm: &[Move]) -> Vec<Move> {
    let mut inverted_algorithm = Vec::with_capacity(algorithm.len());
    for move_ in algorithm.iter().rev() {
        inverted_algorithm.push(invert_move(move_));
    }
    inverted_algorithm
}

#[must_use] pub fn parse_algorithm(algorithm: &str) -> Vec<Move> {
    let mut parsed_algorithm = Vec::with_capacity(algorithm.len());
    let mut chars = algorithm.chars();
    while let Some(c) = chars.next() {
        match c {
            'U' => {
                match chars.next() {
                    Some('2') => parsed_algorithm.push(Move::U2),
                    Some('\'') => parsed_algorithm.push(Move::Up),
                    _ => parsed_algorithm.push(Move::U),
                }
            }
            'D' => {
                match chars.next() {
                    Some('2') => parsed_algorithm.push(Move::D2),
                    Some('\'') => parsed_algorithm.push(Move::Dp),
                    _ => parsed_algorithm.push(Move::D),
                }
            }
            'F' => {
                match chars.next() {
                    Some('2') => parsed_algorithm.push(Move::F2),
                    Some('\'') => parsed_algorithm.push(Move::Fp),
                    _ => parsed_algorithm.push(Move::F),
                }
            }
            'B' => {
                match chars.next() {
                    Some('2') => parsed_algorithm.push(Move::B2),
                    Some('\'') => parsed_algorithm.push(Move::Bp),
                    _ => parsed_algorithm.push(Move::B),
                }
            }
            'R' => {
                match chars.next() {
                    Some('2') => parsed_algorithm.push(Move::R2),
                    Some('\'') => parsed_algorithm.push(Move::Rp),
                    _ => parsed_algorithm.push(Move::R),
                }
            }
            'L' => {
                match chars.next() {
                    Some('2') => parsed_algorithm.push(Move::L2),
                    Some('\'') => parsed_algorithm.push(Move::Lp),
                    _ => parsed_algorithm.push(Move::L),
                }
            }
            _ => {}
        }
    }
    parsed_algorithm
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
            vec![
                Move::U2,
                Move::Dp,
                Move::F2,
                Move::Bp,
                Move::R2,
                Move::Lp
            ]
        );
    }
}
