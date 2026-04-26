use crate::material::{Mat, ScatterRec};
use crate::mesh::HitRec;
use crate::ray::Ray3;
use crate::vec::{Color3, UnitVec3};

pub struct Lambertian {
    albedo: Color3,
}

impl Lambertian {
    pub const fn new(albedo: Color3) -> Mat {
        Mat::Lambertian(Lambertian { albedo })
    }
}

impl Lambertian {
    pub fn scatter(&self, _ray: &Ray3, hit_rec: &HitRec) -> Option<ScatterRec> {
        let scatter_vec = UnitVec3::new_rand().as_vec() + hit_rec.normal.as_vec();
        let scatter_ray = if scatter_vec.is_near_zero() {
            hit_rec.normal.clone().into_ray(hit_rec.point.clone())
        } else {
            scatter_vec.into_ray(hit_rec.point.clone())
        };

        Some(ScatterRec {
            ray: scatter_ray,
            attenuation: self.albedo.clone(),
        })
    }
}
