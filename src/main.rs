mod cam;
mod interval;
mod material;
mod mesh;
mod model;
mod point;
mod ray;
mod util;
mod vec;

use crate::cam::Cam;
use crate::material::Mat;
use crate::mesh::Mesh;
use crate::model::Model;
use crate::point::Point3;
use crate::vec::Color3;

fn main() {
    let cam = Cam::new();

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
            Mat::new_metal(Color3::new(0.8, 0.8, 0.8), 0.3),
        ),
        Model(
            Mesh::new_sphere(Point3::new(1.0, 0.0, -1.0), 0.5),
            Mat::new_metal(Color3::new(0.8, 0.6, 0.2), 1.0),
        ),
    ];

    cam.render(models);
}
