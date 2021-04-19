use std::ops::AddAssign;

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
            self.multiply_scalar(max);
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
    Multiply the x and y values of the vector by the scalar. This will scale up or down the vector.

    ```
    use nature_of_code_in_rust::utilities::vector2::Vector2;

    let mut vector2 = Vector2::new(1.0, 2.0);
    vector2.multiply_scalar(2.0);
    assert_eq!(vector2.x, 2.0);
    assert_eq!(vector2.y, 4.0);
    ```
    */
    pub fn multiply_scalar(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
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
