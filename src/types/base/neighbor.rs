/// Enum which define adjacencies for triangles.
/// For now very simple adjacencies but can be complexified.
///
/// Cell is constructed to store the index of the adjacent cell.
/// A pointer is not used since any change in the Cell array would bring an error.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Neighbor {
    Boundary,
    Cell(usize),
}

impl Default for Neighbor {
    fn default() -> Self {
        Neighbor::Cell(0)
    }
}
