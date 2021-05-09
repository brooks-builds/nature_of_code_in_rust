use std::ops::{AddAssign, DivAssign, MulAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

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
        *self /= self.magnitude();
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
