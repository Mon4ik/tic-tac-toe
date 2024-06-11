use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Pos2 {
    pub x: usize,
    pub y: usize,
}


impl Pos2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Default for Pos2 {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl PartialEq for Pos2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Pos2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Pos2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Pos2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Pos2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}