use crate::Cube;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait CubeSubset:
    PartialEq + Eq + Hash + Clone + Serialize + for<'de> Deserialize<'de>
{
    fn from_cube(cube: &Cube) -> Self;
}
