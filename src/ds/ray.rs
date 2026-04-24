use crate::ds::hittable::{HitRecord, Hittable};
use crate::ds::interval::Interval;
use crate::ds::point::Point3;
use crate::ds::vec::{Color3, UnitVec3, Vec3};

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

    pub fn calc_color(&self, hittables: &[impl Hittable], depth: u8) -> Color3 {
        if depth == 0 {
            return Color3::new_black();
        }

        let mut closest_hit_rec: Option<HitRecord> = None;
        let mut closest_hit_distance = f64::INFINITY;

        for hittable in hittables {
            //NOTE: Hit range starts at 0.001 to prevent shadow acne
            let ref hit_range = Interval::new(0.001, closest_hit_distance);

            if let Some(hit_rec) = hittable.hit(self, hit_range) {
                //CASE: A closer hit that the previous was found
                closest_hit_distance = hit_rec.distance;
                closest_hit_rec = Some(hit_rec);
            }
        }

        let ray_color = if let Some(hit_rec) = closest_hit_rec {
            //CASE: A hit result was found
            //Pixel must represent the color of the ray

            let bounce_vec = {
                let rand_vec = UnitVec3::new_rand();

                //Adjust the rand unit vec to point outwards of the sphere
                if rand_vec.dot(&hit_rec.normal) > 0.0 {
                    rand_vec
                } else {
                    -rand_vec
                }
            };

            let ref bounce_ray = bounce_vec.into_ray(hit_rec.point);
            let ray_color = bounce_ray.calc_color(hittables, depth - 1);
            ray_color * 0.5
        } else {
            //CASE: No hit result was found
            //Consider the pixel as representing the background color
            let ray_dir = self.vec().clone().into_unit();
            let a = 0.5 * (ray_dir.y() + 1.0);
            Color3::new_white() * (1.0 - a) + Color3::new(0.5, 0.7, 1.0) * a
        };

        ray_color
    }
}

impl Ray3 {
    pub fn new(origin: Point3, vec: Vec3) -> Ray3 {
        Ray3 { origin, vec }
    }
}
