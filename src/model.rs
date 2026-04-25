use crate::interval::Interval;
use crate::mesh::{HitRec, Mesh};
use crate::material::{Mat, ScatterRec};
use crate::ray::Ray3;

pub struct Model(pub Mesh, pub Mat);

impl Model {
    /**
     * Hits to the mesh first. If mesh hit is successful, calculates the scatter record.
     */
    pub fn hit(&self, ray: &Ray3, hit_range: &Interval) -> Option<(HitRec, ScatterRec)> {
        let mesh_hit_res = self.0.hit(ray, hit_range);

        if let Some(hit_rec) = mesh_hit_res {
            //CASE: Mesh hit is successful
            let scatter_rec = self.1.scatter(ray, &hit_rec);
            Some((hit_rec, scatter_rec))
        } else {
            //CASE: Mesh hit is unsuccessful
            None
        }
    }
}