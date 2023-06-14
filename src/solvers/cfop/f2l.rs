//! # Solve F2Ls in the CFOP method using IDA* search.
//!
//! We use BFS to generate heuristics for each corner/edge case, then use IDA* to
//! solve the F2Ls. We restrict the moves to three move triggers separated by U
//! moves, so that the cross is not disturbed.

use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    io::Write,
};

use crate::{
    cube::{
        algorithms::{self, Move},
        corner::{self, Corner},
        edge::{self, Edge},
        Cube,
    },
    solvers::utils::{print_bfs_progress, print_bfs_terminated},
};

const CORNER_CASES: usize = 8 * 7 * 6 * 5 * usize::pow(3, 4);
const EDGE_CASES: usize = 8 * 7 * 6 * 5 * usize::pow(2, 4);
const TWO_PAIRS_ONE_EDGE_CASES: usize = 8 * 7 * 8 * 7 * 6 * usize::pow(3, 2) * usize::pow(2, 3);

trait Case: PartialEq + Eq + Hash + Debug {
    fn from_cube(cube: &Cube) -> Self;
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TwoPairsOneEdgeFrontCase {
    dfr: (u8, u8),
    dlf: (u8, u8),
    fr: (u8, u8),
    fl: (u8, u8),
    br: (u8, u8),
}

impl Case for TwoPairsOneEdgeFrontCase {
    fn from_cube(cube: &Cube) -> Self {
        let mut dfr = (0, 0);
        let mut dlf = (0, 0);

        for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
            match piece {
                corner::CornerPiece::Dfr => dfr = (i, *orientation),
                corner::CornerPiece::Dlf => dlf = (i, *orientation),
                _ => {}
            }
        }

        let mut fr = (0, 0);
        let mut fl = (0, 0);
        let mut br = (0, 0);

        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                edge::EdgePiece::FR => fr = (i, *orientation),
                edge::EdgePiece::FL => fl = (i, *orientation),
                edge::EdgePiece::BR => br = (i, *orientation),
                _ => {}
            }
        }

        Self {
            dfr,
            dlf,
            fr,
            fl,
            br,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TwoPairsOneEdgeBackCase {
    dbl: (u8, u8),
    drb: (u8, u8),
    bl: (u8, u8),
    br: (u8, u8),
    fr: (u8, u8),
}

impl Case for TwoPairsOneEdgeBackCase {
    fn from_cube(cube: &Cube) -> Self {
        let mut dbl = (0, 0);
        let mut drb = (0, 0);

        for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
            match piece {
                corner::CornerPiece::Dbl => dbl = (i, *orientation),
                corner::CornerPiece::Drb => drb = (i, *orientation),
                _ => {}
            }
        }

        let mut bl = (0, 0);
        let mut br = (0, 0);
        let mut fr = (0, 0);

        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                edge::EdgePiece::BL => bl = (i, *orientation),
                edge::EdgePiece::BR => br = (i, *orientation),
                edge::EdgePiece::FR => fr = (i, *orientation),
                _ => {}
            }
        }

        Self {
            dbl,
            drb,
            bl,
            br,
            fr,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct CornerCase {
    dfr: (u8, u8),
    dlf: (u8, u8),
    dbl: (u8, u8),
    drb: (u8, u8),
}

impl Case for CornerCase {
    fn from_cube(cube: &Cube) -> Self {
        let mut dfr = (0, 0);
        let mut dlf = (0, 0);
        let mut dbl = (0, 0);
        let mut drb = (0, 0);

        for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
            match piece {
                corner::CornerPiece::Dfr => dfr = (i, *orientation),
                corner::CornerPiece::Dlf => dlf = (i, *orientation),
                corner::CornerPiece::Dbl => dbl = (i, *orientation),
                corner::CornerPiece::Drb => drb = (i, *orientation),
                _ => {}
            }
        }

        Self { dfr, dlf, dbl, drb }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct EdgeCase {
    fr: (u8, u8),
    fl: (u8, u8),
    bl: (u8, u8),
    br: (u8, u8),
}

impl Case for EdgeCase {
    fn from_cube(cube: &Cube) -> Self {
        let mut fr = (0, 0);
        let mut fl = (0, 0);
        let mut bl = (0, 0);
        let mut br = (0, 0);

        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                edge::EdgePiece::FR => fr = (i, *orientation),
                edge::EdgePiece::FL => fl = (i, *orientation),
                edge::EdgePiece::BL => bl = (i, *orientation),
                edge::EdgePiece::BR => br = (i, *orientation),
                _ => {}
            }
        }

        Self { fr, fl, bl, br }
    }
}

pub struct Solver {
    trigger_algs: Vec<Vec<Move>>,
    corner_cases: HashMap<CornerCase, usize>,
    edge_cases: HashMap<EdgeCase, usize>,
    two_pairs_front_cases: HashMap<TwoPairsOneEdgeFrontCase, usize>,
    two_pairs_back_cases: HashMap<TwoPairsOneEdgeBackCase, usize>,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Self {
        let corners = Self::generate_heuristic(CORNER_CASES, "F2L/Corners");
        let edges = Self::generate_heuristic(EDGE_CASES, "F2L/Edges");
        let two_pairs_front =
            Self::generate_heuristic(TWO_PAIRS_ONE_EDGE_CASES, "F2L/Two Pairs Front");
        let two_pairs_back =
            Self::generate_heuristic(TWO_PAIRS_ONE_EDGE_CASES, "F2L/Two Pairs Back");
        Self {
            trigger_algs: Self::generate_trigger_algs(),
            corner_cases: corners,
            edge_cases: edges,
            two_pairs_front_cases: two_pairs_front,
            two_pairs_back_cases: two_pairs_back,
        }
    }

    fn generate_trigger_algs() -> Vec<Vec<Move>> {
        let mut algs = vec![];
        for pre_u_move in [Move::U, Move::Up, Move::U2, Move::None].iter() {
            for move_ in [
                Move::R,
                Move::Rp,
                Move::L,
                Move::Lp,
                Move::F,
                Move::Fp,
                Move::B,
                Move::Bp,
            ]
            .iter()
            {
                for u_move in [Move::U, Move::Up, Move::U2].iter() {
                    let alg = if matches!(pre_u_move, Move::None) {
                        vec![
                            move_.clone(),
                            u_move.clone(),
                            algorithms::invert_move(move_),
                        ]
                    } else {
                        vec![
                            pre_u_move.clone(),
                            move_.clone(),
                            u_move.clone(),
                            algorithms::invert_move(move_),
                        ]
                    };
                    algs.push(alg);
                }
            }
        }
        algs
    }

    fn generate_heuristic<T>(case_count: usize, name: &str) -> HashMap<T, usize>
    where
        T: Case,
    {
        let mut cases = HashMap::with_capacity(case_count);
        let mut queue = VecDeque::with_capacity(case_count);
        queue.push_back((Cube::default(), 0));

        while let Some((cube, distance)) = queue.pop_front() {
            let progress = cases.len();
            print_bfs_progress!(name, progress, case_count);

            let case = Case::from_cube(&cube);
            if cases.contains_key(&case) {
                continue;
            }

            cases.insert(case, distance);

            for alg in &Self::generate_trigger_algs() {
                let mut cube = cube.clone();
                cube.execute_algorithm(alg);
                queue.push_back((cube, distance + 4));
            }
        }
        print_bfs_terminated!(name, cases.len(), case_count);
        cases
    }

    fn assess_distance(&self, cube: &Cube) -> usize {
        // Assess the distance of the cube from the solved state.
        let corner_case = CornerCase::from_cube(cube);
        let edge_case = EdgeCase::from_cube(cube);
        let two_pairs_front_case = TwoPairsOneEdgeFrontCase::from_cube(cube);
        let two_pairs_back_case = TwoPairsOneEdgeBackCase::from_cube(cube);
        let corner_distance = self.corner_cases.get(&corner_case).unwrap();
        let edge_distance = self.edge_cases.get(&edge_case).unwrap();
        let two_pairs_front_distance = self
            .two_pairs_front_cases
            .get(&two_pairs_front_case)
            .unwrap();
        let two_pairs_back_distance = self.two_pairs_back_cases.get(&two_pairs_back_case).unwrap();
        *[
            *corner_distance,
            *edge_distance,
            *two_pairs_front_distance,
            *two_pairs_back_distance,
        ]
        .iter()
        .max()
        .unwrap()
    }

    pub fn solve(&self, cube: &Cube) -> Vec<Move> {
        // Solve the cube using IDA* with the max of the corner and edge heuristics.
        let mut cube = cube.clone();
        let mut bound = self.assess_distance(&cube);
        let mut path = vec![];
        loop {
            let t = self.search(&mut cube, bound, &mut path);
            if t == 0 {
                return path.iter().filter(|x| **x != Move::None).cloned().collect();
            }
            if t == usize::MAX {
                return vec![];
            }
            bound = t;
        }
    }

    fn search(&self, cube: &mut Cube, bound: usize, path: &mut Vec<Move>) -> usize {
        let distance = self.assess_distance(cube);
        let local_lower_bound = path.len() + distance;
        if local_lower_bound > bound {
            return local_lower_bound;
        }
        if distance == 0 {
            return 0;
        }
        let mut min = usize::MAX;

        for alg in &self.trigger_algs {
            let mut cube = cube.clone();
            cube.execute_algorithm(alg);
            path.extend(alg.iter().cloned());
            let t = self.search(&mut cube, bound, path);
            if t == 0 {
                return 0;
            }
            if t < min {
                min = t;
            }
            for _ in 0..alg.len() {
                path.pop();
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_all_cases() {
        let solver = Solver::new();
        assert_eq!(solver.edge_cases.len(), EDGE_CASES);
        assert_eq!(solver.corner_cases.len(), CORNER_CASES);
    }

    #[test]
    fn test_solving_f2l() {
        let solver = Solver::new();
        let mut cube = Cube::default();
        let scramble = algorithms::parse_algorithm("R U R' U' R U2 R'");
        cube.execute_algorithm(&scramble);
        let solution = solver.solve(&cube);
        cube.execute_algorithm(&solution);
        assert_eq!(solver.assess_distance(&cube), 0);
    }
}
