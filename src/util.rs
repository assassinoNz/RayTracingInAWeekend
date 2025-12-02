use rand::{Rng, SeedableRng, rngs::SmallRng};

pub fn deg2rad(deg: f64) -> f64 {
    deg * core::f64::consts::PI / 180.0
}

pub fn rand_f64() -> f64 {
    rand::random_range(0.0..1.0)
}