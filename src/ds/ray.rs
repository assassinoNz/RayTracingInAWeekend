use crate::ds::{hittable::{HitRec, Hittable}, interval::Interval, vec::Color3};

use super::vec::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + (&self.dir * t)
    }

    pub fn calc_col(&self, geoms: &[impl Hittable]) -> Color3 {
        let mut closest_hit_rec: Option<HitRec> = None;
        let mut closest_t = f64::INFINITY;

        for geom in geoms {
            if let Some(hit_rec) = geom.hit(self, &Interval::new(0.001, closest_t)) {
                closest_t = hit_rec.t;
                closest_hit_rec = Some(hit_rec);
            }
        }

        if let Some(hit_rec) = closest_hit_rec {
            return (hit_rec.normal + Color3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let ref unit_direction = self.dir().unit_vec();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color3::new(1.0, 1.0, 1.0) * (1.0 - a) + Color3::new(0.5, 0.7, 1.0) * a
    }
}