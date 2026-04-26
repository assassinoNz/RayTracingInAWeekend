use crate::bound::Interval;
use crate::material::{Mat, ScatterRec};
use crate::mesh::{HitRec, Mesh};
use crate::ray::Ray3;

pub struct Model(pub Mesh, pub Mat);

impl Model {
    /**
     * Hits the mesh first. If mesh hit is successful, scatters the material.
     */
    pub fn hit(&self, ray: &Ray3, hit_range: &Interval) -> Option<(HitRec, ScatterRec)> {
        let hit_rec = self.0.hit(ray, hit_range)?;
        let scatter_rec = self.1.scatter(ray, &hit_rec)?;

        Some((hit_rec, scatter_rec))
    }
}
