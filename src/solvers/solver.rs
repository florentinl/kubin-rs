use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufReader, Read, Write},
};

use crate::cube::{algorithms::Move, Cube};
use core::hash::Hash;
use serde::{Deserialize, Serialize};

pub(super) trait StepSolver: Sized + Serialize + for<'de> Deserialize<'de> {
    fn new(path: &str) -> Self {
        match Self::from_file(path) {
            Ok(solver) => solver,
            Err(_) => {
                let solver = Self::generate();
                solver.save_to_file(path).unwrap();
                solver
            }
        }
    }
    fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let serialized = ron::to_string(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let deserialized: Self = ron::from_str(&contents)?;
        Ok(deserialized)
    }

    /// Generate heuristics programmatically
    fn generate() -> Self;
    /// Solve the step for the given cube
    fn solve(&self, cube: &Cube) -> Vec<Move>;
}

pub(super) trait Case:
    PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>
{
    fn from_cube(cube: &Cube) -> Self;
}
