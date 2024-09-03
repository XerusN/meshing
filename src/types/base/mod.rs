//! Implementation of the basic types needed,
//! such as points, vectors and neighbor definition.
//! Everything is in f64 since the goal is to use this code for scientific computing.

pub use neighbor::*;
pub use point::*;
pub use vector::*;

pub mod neighbor;
pub mod point;
pub mod vector;
