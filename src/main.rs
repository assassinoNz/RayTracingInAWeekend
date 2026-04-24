mod ds;
mod util;

use ds::cam::Cam;
use ds::geom::Sphere;
use ds::point::Point3;

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
    
    let ref geoms = [
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    cam.render(geoms);
}
