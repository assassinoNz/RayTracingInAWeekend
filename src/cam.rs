use crate::model::Model;
use crate::point::Point3;
use crate::util::rand_f64;
use crate::vec::{Color3, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 400;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LEN: f64 = 1.0;
const PIJ_SAMPLE_COUNT: u8 = 10;
const MAX_DEPTH: u8 = 50;

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
    pub fn new() -> Cam {
        let img_height = IMG_WIDTH as f64 / ASPECT_RATIO;
        let img_height = if img_height < 1.0 {
            1
        } else {
            img_height as u32
        };

        let viewport_width = VIEWPORT_HEIGHT * (IMG_WIDTH as f64 / img_height as f64);

        let ref vu = Vec3::new(viewport_width, 0.0, 0.0);
        let ref vv = -Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let ref f = Vec3::new(0.0, 0.0, FOCAL_LEN);
        let c = Point3::new_origin();

        //Find the upper left corner of the viewport
        let ref v00 = &c + (-f) + (-vu / 2.0) + (-vv / 2.0);

        //Divide the viewport into a grid of squares (virtual pixels) matching the image dimensions
        let du = vu / (IMG_WIDTH as f64);
        let dv = vv / (img_height as f64);

        //Find the center of the upper left virtual pixel
        let p00_center = v00 + (&du * 0.5) + (&dv * 0.5);

        Cam {
            aspect_ratio: ASPECT_RATIO,
            img_width: IMG_WIDTH,
            img_height,
            c,
            p00_center,
            du,
            dv,
            pij_sample_count: PIJ_SAMPLE_COUNT,
            pixel_samples_scale: 1.0 / PIJ_SAMPLE_COUNT as f64,
            max_depth: MAX_DEPTH,
        }
    }
}

impl Cam {
    pub fn render(&self, models: &[Model]) {
        println!("P3\n{} {}\n255", self.img_width, self.img_height);

        for j in 0..self.img_height {
            for i in 0..self.img_width {
                let i = i as f64;
                let j = j as f64;

                let pij_samples: [Point3; PIJ_SAMPLE_COUNT as usize] = std::array::from_fn(|_| {
                    let px = i + rand_f64() - 0.5;
                    let py = j + rand_f64() - 0.5;
                    &self.p00_center + (&self.du * px) + (&self.dv * py)
                });

                //Calculate the average color of pij
                let mut pij_color: Color3 = Color3::new_black();
                for pij_sample in &pij_samples {
                    let ref ray = (pij_sample - &self.c).into_ray(self.c.clone());
                    pij_color = pij_color + ray.calc_color(models, self.max_depth);
                }
                pij_color /= self.pij_sample_count as f64;

                //Gamma correct
                let pij_color: Color3 = pij_color.into_gamma_corrected();

                //Print pij color
                let r_byte = (256.0 * pij_color.r().clamp(0.0, 0.999)) as u8;
                let g_byte = (256.0 * pij_color.g().clamp(0.0, 0.999)) as u8;
                let b_byte = (256.0 * pij_color.b().clamp(0.0, 0.999)) as u8;
                println!("{r_byte} {g_byte} {b_byte}");
            }
        }
    }
}
