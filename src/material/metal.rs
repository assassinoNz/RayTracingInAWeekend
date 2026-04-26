use crate::material::{Mat, ScatterRec};
use crate::mesh::HitRec;
use crate::ray::Ray3;
use crate::vec::{Color3, UnitVec3};

pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    pub const fn new(albedo: Color3, fuzz: f64) -> Mat {
        Mat::Metal(Metal { albedo, fuzz })
    }
}

impl Metal {
    pub fn scatter(&self, ray: &Ray3, hit_rec: &HitRec) -> Option<ScatterRec> {
        let reflect_vec = hit_rec.normal.reflect(ray.vec());
        let scatter_vec =
            reflect_vec.into_unit().as_vec() + (UnitVec3::new_rand().as_vec() * self.fuzz);
        let scatter_ray = scatter_vec.into_ray(hit_rec.point.clone());

        if scatter_ray.vec().dot(hit_rec.normal.as_vec()) > 0.0 {
            Some(ScatterRec {
                ray: scatter_ray,
                attenuation: self.albedo.clone(),
            })
        } else {
            None
        }
    }
}
