use crate::ds::hittable::Hittable;
use crate::ds::ray::Ray;
use crate::ds::vec::{Point3, Vec3};
use crate::util;

pub struct Cam {
    aspect_ratio: f64,
    img_width: u32,
    img_height: u32,
    c00: Point3,
    p00: Point3,
    du: Vec3,
    dv: Vec3,
    samples_per_pixel: u32,
}

impl Cam {
    pub fn new(
        aspect_ratio: f64,
        img_width: u32,
        viewport_height: f64,
        focal_len: f64,
        samples_per_pixel: u32,
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
        let c00 = Point3::new(0.0, 0.0, 0.0);

        //Find the upper left corner of the viewport
        let ref v00 = &c00 + (-f) + (-vu / 2.0) + (-vv / 2.0);

        //Divide the viewport into a grid of squares (virtual pixels) matching the image dimensions
        let du = vu / (img_width as f64);
        let dv = vv / (img_height as f64);

        //Find the center of the upper left virtual pixel
        let p00 = v00 + (&du * 0.5) + (&dv * 0.5);

        Cam {
            aspect_ratio,
            img_width,
            img_height,
            c00,
            p00,
            du,
            dv,
            samples_per_pixel,
        }
    }

    pub fn render(&self, geoms: &[impl Hittable]) {
        println!("P3\n{} {}\n255", self.img_width, self.img_height);
        for j in 0..self.img_height {
            for i in 0..self.img_width {
                let i = i as f64;
                let j = j as f64;

                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let offset = Vec3::new(util::rand_f64() - 0.5, util::rand_f64() - 0.5, 0.0);
                    let pij_sample =
                        &self.p00 + (&self.du * (i + offset.x())) + (&self.dv * (j + offset.y()));

                    let ray_dir = pij_sample - &self.c00;
                    let ray = Ray::new(self.c00.clone(), ray_dir);

                    color += &ray.calc_col(geoms);
                }

                color /= self.samples_per_pixel as f64;

                let ir = (255.999 * color.r()) as u8;
                let ig = (255.999 * color.g()) as u8;
                let ib = (255.999 * color.b()) as u8;

                println!("{ir} {ig} {ib}");
            }
        }
    }
}
