use crate::ds::point::Point3;
use crate::ds::vec::Vec3;

pub struct Ray3 {
    origin: Point3,
    vec: Vec3,
}

impl Ray3 {
    pub fn new(origin: Point3, vec: Vec3) -> Ray3 {
        Ray3 { origin, vec }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn vec(&self) -> &Vec3 {
        &self.vec
    }

    /**
     * Returns the point after traveling t times the underlying vector length
     */
    pub fn cast(&self, t: f64) -> Point3 {
        &self.origin + (&self.vec * t)
    }
}
