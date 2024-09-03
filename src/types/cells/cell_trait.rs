use std::collections::binary_heap::Iter;

use crate::types::base::*;

pub trait Cell {
    
    /// Checks if the point is in this cell
    fn include(&self, point: &Point) -> bool;
    
    // //Give an iterator on all edges index of the cell
    // fn iter_edges(&self) -> std::vec::IntoIter<usize>;
    
    /// Gives the centroid of the cell
    fn center(&self) -> Point;
    
    /// Gives the normals to each edge of the cell
    fn normals(&self) -> Vec<Vector>;
}