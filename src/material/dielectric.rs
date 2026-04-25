use crate::material::ScatterRec;
use crate::mesh::HitRec;
use crate::ray::Ray3;
use crate::vec::{Color3, UnitVec3};

pub struct Dielectric {
    refract_idx: f64,
}

impl Dielectric {
    pub fn new(refract_idx: f64) -> Dielectric {
        Dielectric { refract_idx }
    }
}

impl Dielectric {
    pub fn scatter(&self, ray: &Ray3, hit_rec: &HitRec) -> Option<ScatterRec> {
        let ni_div_nt = if hit_rec.is_front_face {
            1.0 / self.refract_idx
        } else {
            self.refract_idx
        };

        let refract_vec = ray
            .vec()
            .clone()
            .into_unit()
            .refract(&hit_rec.normal, ni_div_nt);
        let scatter_ray = refract_vec.into_ray(hit_rec.point.clone());

        Some(ScatterRec {
            ray: scatter_ray,
            attenuation: Color3::new_white(),
        })
    }
}
