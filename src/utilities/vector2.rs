use num::{Float, Num};
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
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
    T: Num + Float + DivAssign + MulAssign,
{
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
    }

    pub fn limit(&mut self, max: T) {
        if self.magnitude() > max {
            self.normalize();
            self.multiply_scalar(max);
        }
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

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_limit_vectors() {
        let mut velocity = Vector2::new(3.0, 4.0);
        let acceleration = Vector2::new(1.0, 1.0);
        velocity += acceleration;
        velocity.limit(5.0);
        assert_eq!(5.0, velocity.magnitude());
    }
}
