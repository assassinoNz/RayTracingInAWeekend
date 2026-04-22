use crate::ds::interval::Interval;
use crate::ds::point::Point3;
use crate::ds::ray::Ray3;
use crate::ds::vec::{UnitVec3};

pub struct HitRecord {
    pub ray_step: f64,
    pub hit_point: Point3,
    pub is_front_face: bool,

    /** The normal against the ray direction */
    pub normal: UnitVec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray3, ray_t: &Interval) -> Option<HitRecord>;
}
