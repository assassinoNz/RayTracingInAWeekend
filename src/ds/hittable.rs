use crate::ds::interval::Interval;
use crate::ds::point::Point3;
use crate::ds::ray::Ray3;
use crate::ds::vec::{UnitVec3};

pub struct HitRecord {
    pub point: Point3,
    pub distance: f64,
    pub is_front_face: bool,

    /** The normal against the ray direction */
    pub normal: UnitVec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray3, hit_range: &Interval) -> Option<HitRecord>;
}
