use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::mesh::HitRec;
use crate::ray::Ray3;
use crate::vec::Color3;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub enum Mat {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(dielectric::Dielectric),
}

impl Mat {
    pub const fn new_lambertian(albedo: Color3) -> Mat {
        Mat::Lambertian(Lambertian::new(albedo))
    }

    pub const fn new_metal(albedo: Color3, fuzz: f64) -> Mat {
        Mat::Metal(Metal::new(albedo, fuzz))
    }

    pub const fn new_dielectric(rel_ref_idx: f64) -> Mat {
        Mat::Dielectric(Dielectric::new(rel_ref_idx))
    }
}

impl Mat {
    pub fn scatter(&self, ray: &Ray3, hit_rec: &HitRec) -> Option<ScatterRec> {
        match self {
            Mat::Lambertian(mat) => mat.scatter(ray, hit_rec),
            Mat::Metal(mat) => mat.scatter(ray, hit_rec),
            Mat::Dielectric(mat) => mat.scatter(ray, hit_rec),
        }
    }
}

pub struct ScatterRec {
    pub ray: Ray3,
    pub attenuation: Color3,
}
