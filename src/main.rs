mod cam;
mod interval;
mod material;
mod mesh;
mod point;
mod ray;
mod util;
mod vec;
mod model;

use crate::cam::Cam;
use crate::material::Mat;
use crate::mesh::Mesh;
use crate::model::Model;
use crate::point::Point3;
use crate::vec::Color3;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 400;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const FOCAL_LEN: f64 = 1.0;
    const PIJ_SAMPLE_COUNT: u8 = 10;
    const MAX_DEPTH: u8 = 50;

    let cam = Cam::new(
        ASPECT_RATIO,
        IMG_WIDTH,
        VIEWPORT_HEIGHT,
        FOCAL_LEN,
        PIJ_SAMPLE_COUNT,
        MAX_DEPTH,
    );

    let ref models = [
        Model(
            Mesh::new_sphere(Point3::new(0.0, -100.5, -1.0), 100.0),
            Mat::new_lambertian(Color3::new(0.8, 0.8, 0.0)),
        ),
        Model(
            Mesh::new_sphere(Point3::new(0.0, 0.0, -1.2), 0.5),
            Mat::new_lambertian(Color3::new(0.1, 0.2, 0.5)),
        ),
        Model(
            Mesh::new_sphere(Point3::new(-1.0, 0.0, -1.0), 0.5),
            Mat::new_metal(Color3::new(0.8, 0.8, 0.8)),
        ),
        Model(
            Mesh::new_sphere(Point3::new(1.0, 0.0, -1.0), 0.5),
            Mat::new_metal(Color3::new(0.8, 0.6, 0.2)),
        )
    ];

    cam.render(models);
}
