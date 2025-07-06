use std::ops::{Add, Mul, Rem, Sub};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Location {
    pub x: isize,
    pub y: isize,
}

pub type Step = Location;

impl Add<Location> for Location {
    type Output = Location;

    fn add(self, rhs: Location) -> Self::Output {
        Location {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Location> for Location {
    type Output = Location;

    fn sub(self, rhs: Location) -> Self::Output {
        Location {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<isize> for Location {
    type Output = Location;

    fn mul(self, rhs: isize) -> Self::Output {
        Location {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Rem<isize> for Location {
    type Output = Location;

    fn rem(self, rhs: isize) -> Self::Output {
        Location {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl Location {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::default();
        for offset in [-1, 1] {
            neighbors.push(Location {
                x: self.x + offset,
                ..*self
            });
            neighbors.push(Location {
                y: self.y + offset,
                ..*self
            });
        }
        neighbors
    }
}
