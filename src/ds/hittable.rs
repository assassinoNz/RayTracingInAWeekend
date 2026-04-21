use crate::ds::interval::Interval;
use crate::ds::point::Point3;
use crate::ds::ray::Ray3;
use crate::ds::vec::Vec3;

pub struct HitResult {
    pub t: f64,
    pub ray_at_t: Point3,
    pub is_front_face: bool,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray3, ray_t: &Interval) -> Option<HitResult>;
}
