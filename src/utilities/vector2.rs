use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

use num::Zero;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn reverse(&mut self) {
        *self *= -1.0;
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        if !magnitude.is_zero() {
            *self /= self.magnitude();
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn as_mint_point2(&self) -> ggez::mint::Point2<f32> {
        ggez::mint::Point2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn perpendicular_left(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn perpendicular_right(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
