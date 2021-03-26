pub fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}
