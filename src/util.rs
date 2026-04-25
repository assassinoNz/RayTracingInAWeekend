pub fn rand_f64() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn rand_f64_clamped(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}

pub fn schlick_reflectrance(cos: f64, rel_ref_idx: f64) -> f64 {
    let r0 = (1.0 - rel_ref_idx) / (1.0 + rel_ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub fn deg_2_rad(deg: f64) -> f64 {
    deg * core::f64::consts::PI / 180.0
}
