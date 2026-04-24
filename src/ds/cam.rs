use crate::ds::hittable::{HitRecord, Hittable};
use crate::ds::interval::Interval;
use crate::ds::point::Point3;
use crate::ds::ray::Ray3;
use crate::ds::vec::{Color3, UnitVec3, Vec3};
use crate::util::rand_f64;

pub struct Cam {
    aspect_ratio: f64,
    img_width: u32,
    img_height: u32,
    c: Point3,
    p00_center: Point3,
    du: Vec3,
    dv: Vec3,
    pij_sample_count: u8,
    pixel_samples_scale: f64,
    max_depth: u8,
}

impl Cam {
    pub fn new(
        aspect_ratio: f64,
        img_width: u32,
        viewport_height: f64,
        focal_len: f64,
        pij_sample_count: u8,
        max_depth: u8,
    ) -> Cam {
        let img_height = img_width as f64 / aspect_ratio;
        let img_height = if img_height < 1.0 {
            1
        } else {
            img_height as u32
        };

        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);

        let ref vu = Vec3::new(viewport_width, 0.0, 0.0);
        let ref vv = -Vec3::new(0.0, viewport_height, 0.0);
        let ref f = Vec3::new(0.0, 0.0, focal_len);
        let c = Point3::new_origin();

        //Find the upper left corner of the viewport
        let ref v00 = &c + (-f) + (-vu / 2.0) + (-vv / 2.0);

        //Divide the viewport into a grid of squares (virtual pixels) matching the image dimensions
        let du = vu / (img_width as f64);
        let dv = vv / (img_height as f64);

        //Find the center of the upper left virtual pixel
        let p00_center = v00 + (&du * 0.5) + (&dv * 0.5);

        Cam {
            aspect_ratio,
            img_width,
            img_height,
            c,
            p00_center,
            du,
            dv,
            pij_sample_count,
            pixel_samples_scale: 1.0 / pij_sample_count as f64,
            max_depth,
        }
    }
}

impl Cam {
    pub fn render(&self, hittables: &[impl Hittable]) {
        println!("P3\n{} {}\n255", self.img_width, self.img_height);

        for j in 0..self.img_height {
            for i in 0..self.img_width {
                let i = i as f64;
                let j = j as f64;

                //Generate samples for pij
                let get_pij_sample = || {
                    let rand_offset = (rand_f64() - 0.5, rand_f64() - 0.5);
                    &self.p00_center
                        + (&self.du * (i + rand_offset.0))
                        + (&self.dv * (j + rand_offset.1))
                };
                let pij_samples: Vec<Point3> = core::iter::repeat_with(get_pij_sample)
                    .take(self.pij_sample_count as usize)
                    .collect();

                //Calculate the average color of pij
                let mut pij_color: Color3 = Color3::new_black();
                for pij_sample in &pij_samples {
                    let ref ray = (pij_sample - &self.c).into_ray(self.c.clone());
                    pij_color = pij_color + ray.calc_color(hittables, self.max_depth);
                }
                pij_color /= self.pij_sample_count as f64;

                //Print pij color
                let ir = (256.0 * pij_color.r().clamp(0.0, 0.999)) as u8;
                let ig = (256.0 * pij_color.g().clamp(0.0, 0.999)) as u8;
                let ib = (256.0 * pij_color.b().clamp(0.0, 0.999)) as u8;
                println!("{ir} {ig} {ib}");
            }
        }
    }
}
