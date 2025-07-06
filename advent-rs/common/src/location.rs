#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Location {
    pub row: isize,
    pub column: isize,
}
impl Location {
    pub fn new(row: isize, column: isize) -> Self {
        Self { row, column }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::default();
        for offset in [-1, 1] {
            neighbors.push(Location {
                row: self.row + offset,
                ..*self
            });
            neighbors.push(Location {
                column: self.column + offset,
                ..*self
            });
        }
        neighbors
    }
}
