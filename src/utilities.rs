use std::ops::{Add, AddAssign};

#[allow(dead_code)]
pub fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
