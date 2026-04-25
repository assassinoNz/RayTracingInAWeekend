use crate::mesh::HitRec;
use crate::material::ScatterRec;
use crate::ray::Ray3;
use crate::vec::{Color3, UnitVec3};

pub struct Lambertian {
    albedo: Color3,
}

impl Lambertian {
    pub fn new(albedo: Color3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Lambertian {
    pub fn scatter(&self, _ray: &Ray3, hit_rec: &HitRec) -> ScatterRec {
        let scatter_vec = UnitVec3::new_rand().as_vec() + hit_rec.normal.as_vec();
        let scattered_ray = if scatter_vec.is_near_zero() {
            hit_rec.normal.clone().into_ray(hit_rec.point.clone())
        } else {
            scatter_vec.into_ray(hit_rec.point.clone())
        };

        ScatterRec {
            ray: scattered_ray,
            attenuation: self.albedo.clone(),
        }
    }
}
