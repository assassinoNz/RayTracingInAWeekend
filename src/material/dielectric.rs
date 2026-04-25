use crate::material::ScatterRec;
use crate::mesh::HitRec;
use crate::ray::Ray3;
use crate::util::{rand_f64, schlick_reflectrance};
use crate::vec::Color3;

pub struct Dielectric {
    /**
     * Ratio of the material's refractive index to the enclosing medium's refractive index
     */
    rel_ref_idx: f64,
}

impl Dielectric {
    pub const fn new(rel_ref_idx: f64) -> Dielectric {
        Dielectric { rel_ref_idx }
    }
}

impl Dielectric {
    pub fn scatter(&self, ray: &Ray3, hit_rec: &HitRec) -> Option<ScatterRec> {
        let rel_ref_idx = if hit_rec.is_front_face {
            1.0 / self.rel_ref_idx
        } else {
            self.rel_ref_idx
        };

        let ray_dir = ray.vec().clone().into_unit();
        let cos = (-ray_dir.dot(&hit_rec.normal)).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();
        let can_refract = rel_ref_idx * sin <= 1.0;

        let scatter_vec = if can_refract && schlick_reflectrance(cos, rel_ref_idx) <= rand_f64() {
            ray_dir.refract(hit_rec.normal.as_vec(), rel_ref_idx)
        } else {
            ray_dir.reflect(hit_rec.normal.as_vec())
        };

        let scatter_ray = scatter_vec.into_ray(hit_rec.point.clone());

        Some(ScatterRec {
            ray: scatter_ray,
            attenuation: Color3::new_white(),
        })
    }
}
