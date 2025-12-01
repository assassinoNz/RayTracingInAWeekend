mod ds;
mod util;

use ds::ray::Ray;
use ds::vec::{Color3, Point3, Vec3};
use ds::geom::Sphere;

use crate::ds::hittable::{HitRec, Hittable};

const fn img_height(img_width: u32, aspect_ratio: f64) -> u32 {
    let img_height = img_width as f64 / aspect_ratio;
    if img_height < 1.0 {
        1
    } else {
        img_height as u32
    }
}

fn ray_color(ray: &Ray, geoms: &[impl Hittable]) -> Color3 {
    let mut closest_hit_rec: Option<HitRec> = None;
    let mut closest_t = f64::INFINITY;

    for geom in geoms {
        if let Some(hit_rec) = geom.hit(ray, 0.0, closest_t) {
            if hit_rec.t < closest_t {
                closest_t = hit_rec.t;
                closest_hit_rec = Some(hit_rec);
            }
        }
    }
    
    if let Some(hit_rec) = closest_hit_rec {
        return (hit_rec.normal + Color3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let ref unit_direction = ray.dir().unit_vec();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color3::new(1.0, 1.0, 1.0) * (1.0 - a) + Color3::new(0.5, 0.7, 1.0) * a
}

fn main() {
    //Define image dimensions
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = img_height(IMG_WIDTH, ASPECT_RATIO);

    //Define viewport dimensions to match image aspect ration
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMG_WIDTH as f64 / IMG_HEIGHT as f64);
    const FOCAL_LEN: f64 = 1.0;

    let ref vu = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let ref vv = -Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let ref f = Vec3::new(0.0, 0.0, FOCAL_LEN);
    let ref c00 = Point3::new(0.0, 0.0, 0.0);

    //Find the upper left corner of the viewport
    let ref v00 = c00 + (-f) + (-vu / 2.0) + (-vv / 2.0);

    //Divide the viewport into a grid of squares (virtual pixels) matching the image dimensions
    let ref du = vu / (IMG_WIDTH as f64);
    let ref dv = vv / (IMG_HEIGHT as f64);

    //Find the center of the upper left virtual pixel
    let ref p00 = v00 + (du * 0.5) + (dv * 0.5);

    let ref geoms = [
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)
    ];

    //Render
    println!("P3\n{IMG_WIDTH} {IMG_HEIGHT}\n255");
    for j in 0..IMG_HEIGHT {
        for i in 0..IMG_WIDTH {
            let i = i as f64;
            let j = j as f64;

            let pij = p00 + (du * i) + (dv * j);
            let ray_dir = pij - c00;
            let ref ray = Ray::new(c00.clone(), ray_dir);

            let color = ray_color(ray, geoms);
            util::print_col(&color);
        }
    }
}


//  bool hit(const ray& r, double ray_tmin, double ray_tmax, hit_record& rec) const override {
//         vec3 oc = center - r.origin();
//         auto a = r.direction().length_squared();
//         auto h = dot(r.direction(), oc);
//         auto c = oc.length_squared() - radius*radius;

//         auto discriminant = h*h - a*c;
//         if (discriminant < 0)
//             return false;

//         auto sqrtd = std::sqrt(discriminant);

//         // Find the nearest root that lies in the acceptable range.
//         auto root = (h - sqrtd) / a;
//         if (root <= ray_tmin || ray_tmax <= root) {
//             root = (h + sqrtd) / a;
//             if (root <= ray_tmin || ray_tmax <= root)
//                 return false;
//         }

//         rec.t = root;
//         rec.p = r.at(rec.t);
//         vec3 outward_normal = (rec.p - center) / radius;
//         front_face = dot(r.direction(), outward_normal) < 0;
//         rec.normal = front_face ? outward_normal : -outward_normal;

//         return true;
//     }

// bool hit(const ray& r, double ray_tmin, double ray_tmax, hit_record& rec) const override {
//         hit_record temp_rec;
//         bool hit_anything = false;
//         auto closest_so_far = ray_tmax;

//         for (const auto& object : objects) {
//             if (object->hit(r, ray_tmin, closest_so_far, temp_rec)) {
//                 hit_anything = true;
//                 closest_so_far = temp_rec.t;
//                 rec = temp_rec;
//             }
//         }

//         return hit_anything;
//     }