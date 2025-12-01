use crate::ds::vec::Color3;

pub fn deg2rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

pub fn print_col(color: &Color3) {
    let ir = (255.999 * color.r()) as u8;
    let ig = (255.999 * color.g()) as u8;
    let ib = (255.999 * color.b()) as u8;
    
    println!("{ir} {ig} {ib}");
}