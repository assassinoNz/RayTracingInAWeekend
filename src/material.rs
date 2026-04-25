use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::mesh::HitRec as HitRec;
use crate::ray::Ray3;
use crate::vec::Color3;

pub mod lambertian;
pub mod metal;

pub enum Mat {
    Lambertian(Lambertian),
    Metal(Metal)
}

impl Mat {
    pub fn new_lambertian(albedo: Color3) -> Mat {
        Mat::Lambertian(Lambertian::new(albedo))
    }

    pub fn new_metal(albedo: Color3) -> Mat {
        Mat::Metal(Metal::new(albedo))
    }
}

impl Mat {
    pub fn scatter(&self, ray: &Ray3, hit_rec: &HitRec) -> ScatterRec {
        match self {
            Mat::Lambertian(mat) => mat.scatter(ray, hit_rec),
            Mat::Metal(mat) => mat.scatter(ray, hit_rec),
        }
    }
}

pub struct ScatterRec {
    pub ray: Ray3,
    pub attenuation: Color3
}