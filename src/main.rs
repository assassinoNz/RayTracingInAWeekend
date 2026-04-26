mod bound;
mod cam;
mod material;
mod mesh;
mod model;
mod point;
mod ray;
mod util;
mod vec;

use crate::cam::Cam;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::mesh::Sphere;
use crate::model::Model;
use crate::point::Point3;
use crate::util::rand_f64;
use crate::vec::Color3;

const MAX_MODELS: usize = 488;

fn main() {
    // let ref models = [
    //     Model(
    //         Mesh::new_sphere(Point3::new(0.0, -100.5, -1.0), 100.0),
    //         Mat::new_lambertian(Color3::new(0.8, 0.8, 0.0)),
    //     ),
    //     Model(
    //         Mesh::new_sphere(Point3::new(0.0, 0.0, -1.2), 0.5),
    //         Mat::new_lambertian(Color3::new(0.1, 0.2, 0.5)),
    //     ),
    //     Model(
    //         Mesh::new_sphere(Point3::new(-1.0, 0.0, -1.0), 0.5),
    //         Mat::new_dielectric(1.5),
    //     ),
    //     Model(
    //         Mesh::new_sphere(Point3::new(-1.0, 0.0, -1.0), 0.4),
    //         Mat::new_dielectric(1.0 / 1.5),
    //     ),
    //     Model(
    //         Mesh::new_sphere(Point3::new(1.0, 0.0, -1.0), 0.5),
    //         Mat::new_metal(Color3::new(0.8, 0.6, 0.2), 1.0),
    //     ),
    // ];

    let cam = Cam::new();
    let ref models = rand_scene();
    cam.render(models);
}

fn rand_scene() -> [Model; MAX_MODELS] {
    // Initialize with a dummy or default Model (requires manual Default or similar)
    let mut models = [const {
        Model(
            Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.0),
            Lambertian::new(Color3::new(0.0, 0.0, 0.0)),
        )
    }; MAX_MODELS];

    let mut count = 0;

    // 1. Add Ground
    models[count] = Model(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0),
        Lambertian::new(Color3::new(0.5, 0.5, 0.5)),
    );
    count += 1;

    // 2. Add Random Small Spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let center = Point3::new(
                a as f64 + 0.9 * rand_f64(),
                0.2,
                b as f64 + 0.9 * rand_f64(),
            );

            // Avoid placing spheres too close to the three large ones
            if (&center - &Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let mat = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color3::new_rand() * Color3::new_rand();
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color3::new_rand_clamped(0.5, 1.0);
                    let fuzz = rand_f64() * 0.5;
                    Metal::new(albedo, fuzz)
                } else {
                    // Glass
                    Dielectric::new(1.5)
                };

                models[count] = Model(Sphere::new(center, 0.2), mat);
                count += 1;
            }
        }
    }

    // 3. Add Three Large Spheres
    models[count] = Model(
        Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0),
        Dielectric::new(1.5),
    );
    count += 1;

    models[count] = Model(
        Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0),
        Lambertian::new(Color3::new(0.4, 0.2, 0.1)),
    );
    count += 1;

    models[count] = Model(
        Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0),
        Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0),
    );

    models
}
