use crate::ds::hittable::{HitRec, Hittable};
use crate::ds::interval::Interval;
use crate::ds::ray::Ray;
use crate::ds::vec::Point3;

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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRec> {
        let ref oc = &self.center - ray.origin();
        let a = ray.dir().len_sq();
        let h = ray.dir().dot(oc);
        let c = oc.len_sq() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            //CASE: Ray misses sphere
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut t = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(t) {
            t = (h + discriminant_sqrt) / a;
            if !ray_t.surrounds(t) {
                //CASE: Ray doesn't hit the sphere within the given range
                return None;
            }
        }

        let ray_at_t = ray.at(t);
        let outward_normal = (&ray_at_t - &self.center) / self.radius;
        let is_front_face = ray.dir().dot(&outward_normal) < 0.0;

        let hit_rec = HitRec {
            t,
            ray_at_t,
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
