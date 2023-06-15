use std::io::Write;

// Macro to print progress of BFS.
macro_rules! print_bfs_progress {
    ($name:expr, $current:ident, $total:ident) => {
        if $current % 1000 == 0 {
            let icon = match ($current / 1000) % 4 {
                0 => "◜",
                1 => "◝",
                2 => "◞",
                3 => "◟",
                _ => unreachable!(),
            };
            print!(
                "Generating {} Lookup Table {}: {} / {}\r",
                $name, icon, $current, $total
            );
            std::io::stdout().flush().unwrap();
        }
    };
}
use std::collections::{HashMap, VecDeque};

use cube::{algorithms::Move, subcases::CubeSubset, Cube};
pub(crate) use print_bfs_progress;

macro_rules! print_bfs_terminated {
    ($name:expr, $current:expr, $total:ident) => {
        println!(
            "Generating {} Lookup Table ✅: {} / {}",
            $name, $current, $total
        );
    };
}

pub(crate) use print_bfs_terminated;

pub(crate) fn generate_heuristic<T>(
    case_count: usize,
    name: &str,
    moves: &Vec<Vec<Move>>,
) -> HashMap<T, usize>
where
    T: CubeSubset,
{
    let mut cases = HashMap::with_capacity(case_count);
    let mut queue = VecDeque::with_capacity(case_count);
    queue.push_back((Cube::default(), 0));

    while let Some((cube, distance)) = queue.pop_front() {
        let progress = cases.len();
        print_bfs_progress!(name, progress, case_count);

        let case = CubeSubset::from_cube(&cube);
        if cases.contains_key(&case) {
            continue;
        }

        cases.insert(case, distance);

        for alg in moves {
            let mut cube = cube.clone();
            cube.execute_algorithm(alg);
            queue.push_back((cube, distance + 4));
        }
    }
    print_bfs_terminated!(name, cases.len(), case_count);
    cases
}
