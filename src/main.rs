mod ds;
mod util;

use ds::cam::Cam;
use ds::geom::Sphere;
use ds::vec::Point3;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 400;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const FOCAL_LEN: f64 = 1.0;

    let cam = Cam::new(
        ASPECT_RATIO,
        IMG_WIDTH,
        VIEWPORT_HEIGHT,
        FOCAL_LEN
    );
    
    let ref geoms = [
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
    ];

    cam.render(geoms);
}
