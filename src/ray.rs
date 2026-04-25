use crate::interval::Interval;
use crate::material::ScatterRec;
use crate::mesh::HitRec;
use crate::model::Model;
use crate::point::Point3;
use crate::vec::{Color3, Vec3};

pub struct Ray3 {
    origin: Point3,
    vec: Vec3,
}

impl Ray3 {
    pub fn new(origin: Point3, vec: Vec3) -> Ray3 {
        Ray3 { origin, vec }
    }
}

impl Ray3 {
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn vec(&self) -> &Vec3 {
        &self.vec
    }

    /**
     * Returns the point after traveling "step" steps of vec length toward the vec direction
     */
    pub fn cast(&self, step: f64) -> Point3 {
        &self.origin + (&self.vec * step)
    }

    /**
     * Finds and hits the camera-closest model in the scene and uses the result's scatter record to calculate the color
     */
    pub fn calc_color(&self, models: &[Model], depth: u8) -> Color3 {
        if depth == 0 {
            return Color3::new_black();
        }

        let mut closest_hit_res: Option<(HitRec, ScatterRec)> = None;
        let mut closest_hit_distance = f64::INFINITY;

        for model in models {
            //NOTE: Hit range starts at 0.001 to prevent shadow acne
            let ref hit_range = Interval::new(0.001, closest_hit_distance);

            if let Some(hit_res) = model.hit(self, hit_range) {
                //CASE: A closer hit that the previous was found
                closest_hit_distance = hit_res.0.distance;
                closest_hit_res = Some(hit_res);
            }
        }

        let ray_color = if let Some((_, scatter_rec)) = closest_hit_res {
            //CASE: A hit result was found
            //Pixel must represent the color of the scattered ray
            scatter_rec.ray.calc_color(models, depth - 1) * scatter_rec.attenuation
        } else {
            //CASE: No hit result was found
            //Consider the pixel as representing the background color
            let ray_dir = self.vec().clone().into_unit();
            let a = 0.5 * (ray_dir.y() + 1.0);
            Color3::new_white() * (1.0 - a) + Color3::new(0.5, 0.7, 1.0) * a
        };

        ray_color
    }
}
