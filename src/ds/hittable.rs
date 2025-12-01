use crate::ds::{ray::Ray, vec::{Point3, Vec3}};

pub struct HitRec {
    pub t: f64,
    pub ray_at_t: Point3,
    pub is_front_face: bool,
    pub normal: Vec3
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec>;
}