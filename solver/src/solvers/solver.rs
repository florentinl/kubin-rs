use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
};

use cube::{algorithms::Move, Cube};
use serde::{Deserialize, Serialize};

pub(super) trait Step: Sized + Serialize + for<'de> Deserialize<'de> + Clone {
    fn new(path: &str) -> Self {
        if let Ok(solver) = Self::from_file(path) {
            solver
        } else {
            let solver = Self::generate();
            solver.save_to_file(path).unwrap();
            solver
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

pub trait Method: Clone {
    fn solve(&self, cube: &Cube) -> Vec<Move>;
}
