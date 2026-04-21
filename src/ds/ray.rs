use crate::ds::point::Point3;
use crate::ds::vec::Vec3;

pub struct Ray3 {
    origin: Point3,
    vec: Vec3,
}

impl Ray3 {
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn vec(&self) -> &Vec3 {
        &self.vec
    }

    /**
     * Returns the point after traveling "step" steps of vec length toward the vec direction
     */
    pub fn cast(&self, step: f64) -> Point3 {
        &self.origin + (&self.vec * step)
    }
}

impl Ray3 {
    pub fn new(origin: Point3, vec: Vec3) -> Ray3 {
        Ray3 { origin, vec }
    }
}
