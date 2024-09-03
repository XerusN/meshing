//! This crates aims at creating mesh for CFD use.
//!
//! For now only the 2D and without obstacles algorithm are implemented.
//! While the type system does support any cell shape, only an unconstained delaunay triangulation is implemented (with triangles).
//!
//! This is only WIP and don't hesitate to reach out if you have any advices on the algorithm or on the rust part.
//!

pub use gen_vertices::*;
pub use triangulation::*;
pub use types::*;

pub mod gen_vertices;
pub mod triangulation;
pub mod types;
