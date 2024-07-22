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

