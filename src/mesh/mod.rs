use crate::bound::Interval;
use crate::point::Point3;
use crate::ray::Ray3;
use crate::vec::UnitVec3;

mod sphere;
pub use sphere::*;

pub enum Mesh {
    Sphere(Sphere),
}

impl Mesh {
    pub fn hit(&self, ray: &Ray3, hit_range: &Interval) -> Option<HitRec> {
        match self {
            Mesh::Sphere(mesh) => mesh.hit(ray, hit_range),
        }
    }
}

pub struct HitRec {
    pub point: Point3,
    pub distance: f64,
    pub is_front_face: bool,

    /** The normal against the ray direction */
    pub normal: UnitVec3,
}
