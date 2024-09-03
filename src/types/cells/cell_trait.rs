use crate::types::base::*;

/// This trait defines the methods needed for each cell.
/// The cfd code based on this should be able to support any shape.
pub trait Cell {
    /// Checks if the point is in this cell
    fn include(&self, point: &Point) -> bool;

    /// Gives an iterator on all vertices of the cell
    fn iter_vertices(&self) -> std::slice::Iter<Point>;

    /// Gives an iterator on all neighbors of the cell
    fn iter_adjacencies(&self) -> std::slice::Iter<Option<Neighbor>>;

    /// Gives the centroid of the cell
    fn center(&self) -> Point;

    /// Gives the normals to each edge of the cell
    fn normals(&self) -> Vec<Vector>;

    /// Gives the siged area of the cell.
    /// Is positive if the vertices are defined counter-clockwise.
    fn signed_area(&self) -> f64;
}
