use num::Num;
use std::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Num + Copy + MulAssign,
{
    #[allow(dead_code)]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    pub fn multiply_scalar(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl<T> Add for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vector2<T>
where
    T: Num + Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign for Vector2<T>
where
    T: Num + Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
