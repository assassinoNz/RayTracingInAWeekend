use crate::mesh::HitRec;
use crate::material::ScatterRec;
use crate::vec::Color3;
use crate::ray::Ray3;

pub struct Metal {
    albedo: Color3,
}

impl Metal {
    pub fn new(albedo: Color3) -> Metal {
        Metal { albedo }
    }
}

impl Metal {
    pub fn scatter(&self, ray: &Ray3, hit_rec: &HitRec) -> ScatterRec {
        let scatter_vec = hit_rec.normal.reflect(ray.vec());
        let scattered_ray = scatter_vec.into_ray(hit_rec.point.clone());

        ScatterRec {
            ray: scattered_ray,
            attenuation: self.albedo.clone(),
        }
    }
}