use num::{Float, Num};
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[allow(dead_code)]
impl<T> Vector2<T>
where
    T: Num + Copy + MulAssign,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    pub fn multiply_scalar(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

#[allow(dead_code)]
impl<T> Vector2<T>
where
    T: Num + Float + DivAssign,
{
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
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
