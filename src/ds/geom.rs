use crate::ds::hittable::{HitBouncePointRecord, HitRecord, Hittable};
use crate::ds::interval::Interval;
use crate::ds::point::Point3;
use crate::ds::ray::Ray3;
use crate::ds::vec::UnitVec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray3, hit_range: &Interval) -> Option<HitRecord> {
        let ref oc = &self.center - ray.origin();
        let a = ray.vec().len_sq();
        let h = ray.vec().dot(oc);
        let c = oc.len_sq() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            //CASE: Ray misses sphere
            return None;
        }

        //CASE: Ray hits the sphere
        let discriminant_sqrt = discriminant.sqrt();
        let mut hit_distance = (h - discriminant_sqrt) / a;
        if !hit_range.surrounds(hit_distance) {
            //CASE: Ray doesn't hit the sphere within the given step range
            //Check the other answer for the step
            hit_distance = (h + discriminant_sqrt) / a;
            if !hit_range.surrounds(hit_distance) {
                //CASE: Ray doesn't hit the sphere within the given range
                return None;
            }
        }

        let hit_point = ray.cast(hit_distance);
        let outward_normal =
            unsafe { UnitVec3::new_unchecked((&hit_point - &self.center) / self.radius) };
        let is_front_face = ray.vec().dot(&outward_normal) < 0.0;

        let hit_rec = HitRecord {
            distance: hit_distance,
            point: hit_point,
            is_front_face,
            normal: if is_front_face {
                outward_normal
            } else {
                -outward_normal
            },
        };

        Some(hit_rec)
    }
}
