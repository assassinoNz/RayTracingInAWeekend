pub fn rand_f64() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn rand_f64_clamped(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}

pub fn deg_2_rad(deg: f64) -> f64 {
    deg * core::f64::consts::PI / 180.0
}