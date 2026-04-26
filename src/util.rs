use crate::{
    point::Point3,
    vec::{UnitVec3, Vec3},
};

pub fn rand_f64() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn rand_f64_clamped(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}

pub fn schlick_reflectrance(cos: f64, rel_ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - rel_ref_idx) / (1.0 + rel_ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub const fn deg_2_rad(deg: f64) -> f64 {
    deg * core::f64::consts::PI / 180.0
}
