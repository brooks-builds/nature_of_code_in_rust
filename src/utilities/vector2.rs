use std::ops::{AddAssign, Mul, MulAssign, Sub};

use num::Zero;
use rand::random;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /**
    Creates a new Vector 2 with random values and a magnitude of 1;

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let vector2 = Vector2::random();
    assert!(vector2.x > -1.0 && vector2.x < 1.0);
    ```
    */
    pub fn random() -> Self {
        let x = random::<f32>() - 0.5;
        let y = random::<f32>() - 0.5;
        let mut vector2 = Self::new(x, y);
        vector2.normalize();
        vector2
    }

    /**
    Take in a vector2 and a scalar, multiply them together and return a new Vector2.

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let v = Vector2::new(1.0, 5.0);
    let u = Vector2::multiply_scalar_static(v, 2.0);
    assert_eq!(v, Vector2::new(1.0, 5.0));
    assert_eq!(u, Vector2::new(2.0, 10.0));
    ```
    */
    pub fn multiply_scalar_static(vector2: Self, scalar: f32) -> Self {
        let mut vector2 = vector2;
        vector2 *= scalar;
        vector2
    }

    /**
    Get the length of the vector, where the length is the hypotenuse of the triangle that each arm of the vector represents.

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let vector2 = Vector2::new(3.0, 4.0);
    let length = vector2.magnitude();
    assert_eq!(length, 5.0);
    ```
    */
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /**
    Clamp the length of the vector to the given value

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let mut velocity = Vector2::new(3.0, 4.0);
    assert_eq!(velocity.magnitude(), 5.0);
    let acceleration = Vector2::new(1.0, 1.0);
    velocity += acceleration;
    velocity.limit(5.0);
    assert_eq!(velocity.magnitude(), 5.0);

    ```
    */
    pub fn limit(&mut self, max: f32) {
        if self.magnitude() > max {
            self.normalize();
            *self *= max;
        }
    }

    /**
    Return an array representing the vector as [x, y]. This is mostly useful with ggez
    as the framework uses [x, y] format for Mint Point2's.

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let vector2 = Vector2::new(1.0, 2.0);
    assert_eq!(vector2.to_array(), [1.0, 2.0]);
    ```
    */
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    /**
    Mutate the vector so that it's length equals 1. It does this by dividing the x and y
    by the magnitude of the vector.

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let mut vector2 = Vector2::new(3.0, 4.0);
    assert_eq!(vector2.magnitude(), 5.0);
    vector2.normalize();
    assert_eq!(vector2.magnitude(), 1.0);
    ```
    */
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        if magnitude.is_zero() {
            return;
        }
        self.x /= magnitude;
        self.y /= magnitude;
    }

    /**
    Divide the x and y values of the vector by the scalar. This will scale up or down the vector.

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let mut vector2 = Vector2::new(2.0, 4.0);
    vector2.divide_scalar(2.0);
    assert_eq!(vector2.x, 1.0);
    assert_eq!(vector2.y, 2.0);
    ```
    */
    pub fn divide_scalar(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl AddAssign for Vector2 {
    /**
    When adding two vectors together the x and y values are added.

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let mut location = Vector2::new(0.0, 0.0);
    let velocity = Vector2::new(1.0, 2.0);
    location += velocity;
    assert_eq!(location, Vector2::new(1.0, 2.0));
    ```
    */
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
    Translate the following pseudocode to code using static or non-static functions where appropriate.

    The PVector v equals (1,5).

    The PVector u equals v multiplied by 2.

    The PVector w equals v minus u.

    Divide the PVector w by 3.


    */
    #[test]
    fn exercise_1_7() {
        let v = Vector2::new(1.0, 5.0);
        let u = Vector2::multiply_scalar_static(v, 2.0);
        let mut w = v - u;
        w.divide_scalar(3.0);
    }
}
