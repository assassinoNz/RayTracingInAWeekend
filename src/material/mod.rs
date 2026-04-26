use crate::mesh::HitRec;
use crate::ray::Ray3;
use crate::vec::Color3;

mod dielectric;
pub use dielectric::*;
mod lambertian;
pub use lambertian::*;
mod metal;
pub use metal::*;

pub enum Mat {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(dielectric::Dielectric),
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
