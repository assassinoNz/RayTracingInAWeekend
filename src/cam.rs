use crate::model::Model;
use crate::point::Point3;
use crate::util::{deg_2_rad, rand_f64};
use crate::vec::{Color3, UnitVec3, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 1200;
const V_FOV: f64 = 20.0;
const LOOK_FROM: Point3 = Point3::new(13.0, 2.0, 3.0);
const LOOK_AT: Point3 = Point3::new(0.0, 0.0, 0.0);
const V_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const PIJ_SAMPLE_COUNT: u16 = 10;
const MAX_DEPTH: u8 = 5;
const DEFOCUS_ANGLE: f64 = 0.6;
const FOCUS_DIST: f64 = 10.0;

pub struct Cam {
    aspect_ratio: f64,
    img_width: u32,
    img_height: u32,
    c: Point3,
    p00_center: Point3,
    du: Vec3,
    dv: Vec3,
    w: UnitVec3,
    u: UnitVec3,
    v: Vec3,
    pij_sample_count: u16,
    pixel_samples_scale: f64,
    max_depth: u8,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Cam {
    pub fn new() -> Cam {
        let img_height = IMG_WIDTH as f64 / ASPECT_RATIO;
        let img_height = if img_height < 1.0 {
            1
        } else {
            img_height as u32
        };

        let viewport_height = 2.0 * (deg_2_rad(V_FOV) / 2.0).tan() * FOCUS_DIST;
        let viewport_width = viewport_height * (IMG_WIDTH as f64 / img_height as f64);

        let w = (&LOOK_FROM - &LOOK_AT).into_unit();
        let u = V_UP.cross(&w).into_unit();
        let v = w.cross(&u);

        let ref vu = u.as_vec() * viewport_width;
        let ref vv = -&v * viewport_height;

        //Divide the viewport into a grid of squares (virtual pixels) matching the image dimensions
        let du = vu / (IMG_WIDTH as f64);
        let dv = vv / (img_height as f64);

        //Find the upper left corner of the viewport
        let ref v00 = (LOOK_FROM - Point3::new_origin())
            - (w.as_vec() * FOCUS_DIST)
            - (vu * 0.5)
            - (vv * 0.5);

        //Find the center of the upper left virtual pixel
        let p00_center = Point3::from(v00 + (&du + &dv) * 0.5);

        //Find the defocus params
        let defocus_r = FOCUS_DIST * deg_2_rad(DEFOCUS_ANGLE / 2.0).tan();
        let defocus_disk_u = u.as_vec() * defocus_r;
        let defocus_disk_v = &v * defocus_r;

        Cam {
            aspect_ratio: ASPECT_RATIO,
            img_width: IMG_WIDTH,
            img_height,
            c: LOOK_FROM,
            u,
            v,
            w,
            p00_center,
            du,
            dv,
            pij_sample_count: PIJ_SAMPLE_COUNT,
            pixel_samples_scale: 1.0 / PIJ_SAMPLE_COUNT as f64,
            max_depth: MAX_DEPTH,
            defocus_angle: DEFOCUS_ANGLE,
            defocus_disk_u,
            defocus_disk_v,
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

                //Calculate the average color of pij
                let mut pij_color: Color3 = Color3::new_black();
                for _ in 0..self.pij_sample_count {
                    let px = i + rand_f64() - 0.5;
                    let py = j + rand_f64() - 0.5;
                    let pij_sample = &self.p00_center + (&self.du * px) + (&self.dv * py);

                    let ray_origin = if self.defocus_angle <= 0.0 {
                        self.c.clone()
                    } else {
                        let rand_point = Point3::new_rand_in_unit_circ();
                        &self.c
                            + (&self.defocus_disk_u * rand_point.x())
                            + (&self.defocus_disk_v * rand_point.y())
                    };
                    let ref ray = (pij_sample - &ray_origin).into_ray(ray_origin);
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
