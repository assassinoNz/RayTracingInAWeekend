use crate::ds::hittable::{HitRecord, Hittable};
use crate::ds::interval::Interval;
use crate::ds::point::Point3;
use crate::ds::vec::{Color3, Vec3};
use crate::util::rand_f64;

pub struct Cam {
    aspect_ratio: f64,
    img_width: u32,
    img_height: u32,
    c: Point3,
    p00_center: Point3,
    du: Vec3,
    dv: Vec3,
    sample_count_per_pixel: u8,
    pixel_samples_scale: f64,
}

impl Cam {
    pub fn new(aspect_ratio: f64, img_width: u32, viewport_height: f64, focal_len: f64, sample_count_per_pixel: u8) -> Cam {
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
            sample_count_per_pixel,
            pixel_samples_scale: 1.0 / sample_count_per_pixel as f64,
        }
    }
}

impl Cam {
    pub fn render(&self, hittables: &[impl Hittable]) {
        println!("P3\n{} {}\n255", self.img_width, self.img_height);

        // let ref third_of_du = &self.du / 3.0;
        // let ref third_of_dv = &self.dv / 3.0;

        for j in 0..self.img_height {
            for i in 0..self.img_width {
                let i = i as f64;
                let j = j as f64;

                // let pij_center = &self.p00_center + (&self.du * i) + (&self.dv * j);
                // let ref pij_positions = [
                //     &pij_center + (third_of_du + third_of_dv),  //pij_bottom_right
                //     &pij_center + (third_of_du - third_of_dv),  //pij_top_right
                //     &pij_center + -(third_of_du - third_of_dv), //pij_top_left
                //     &pij_center + -(third_of_du + third_of_dv), //pij_bottom_left
                //     pij_center,                                 //pij_center
                // ];

                let get_pij_sample = || {
                    let rand_offset = (rand_f64() - 0.5, rand_f64() - 0.5);
                    &self.p00_center + (&self.du * (i + rand_offset.0)) + (&self.dv * (j + rand_offset.1))
                };
                let ref pij_positions: Vec<Point3> =
                    core::iter::repeat_with(get_pij_sample)
                        .take(self.sample_count_per_pixel as usize)
                        .collect();

                let mut pij_color: Color3 = Color3::new_black();
                for pij_position in pij_positions {
                    let ray_vec = pij_position - &self.c;
                    let ref ray = ray_vec.into_ray(self.c.clone());

                    let mut closest_hit_rec: Option<HitRecord> = None;
                    let mut closest_t = f64::INFINITY;

                    for hittable in hittables {
                        let ref interval = Interval::new(0.001, closest_t);
                        if let Some(hit_res) = hittable.hit(ray, interval) {
                            //CASE: A closer hit that the previous was found
                            closest_t = hit_res.ray_step;
                            closest_hit_rec = Some(hit_res);
                        }
                    }

                    if let Some(hit_res) = closest_hit_rec {
                        //CASE: A hit result was found
                        //Pixel must represent the surface color of te hittable object
                        let pij_position_color: Color3 =
                            hit_res.normal.as_vec() + Color3::new_white() * 0.5;
                        pij_color = pij_color + pij_position_color;
                    } else {
                        //CASE: No hit result was found
                        //Consider the pixel as representing the background color
                        let ray_dir = ray.vec().clone().into_unit();
                        let a = 0.5 * (ray_dir.y() + 1.0);
                        let pij_position_color: Color3 =
                            Color3::new_white() * (1.0 - a) + Color3::new(0.5, 0.7, 1.0) * a;
                        pij_color = pij_color + pij_position_color;
                    }
                }

                pij_color /= pij_positions.len() as f64;

                let ir = (256.0 * pij_color.r().clamp(0.0, 0.999)) as u8;
                let ig = (256.0 * pij_color.g().clamp(0.0, 0.999)) as u8;
                let ib = (256.0 * pij_color.b().clamp(0.0, 0.999)) as u8;

                println!("{ir} {ig} {ib}");
            }
        }
    }
}
