use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Default)]
pub struct Random {
    rng: ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a random number between min and max exclusively
    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        self.rng.gen_range(min..max)
    }
}
