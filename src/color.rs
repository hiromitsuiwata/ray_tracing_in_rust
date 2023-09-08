use image::Rgb;

use crate::vec3::Vec3;

pub fn write_color(pixel: &mut Rgb<u8>, color: Vec3, sample_per_pixel: u32) {
    let scale: f64 = 1.0 / sample_per_pixel as f64;

    let r = color.r() * scale;
    let g = color.g() * scale;
    let b = color.b() * scale;

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    *pixel = image::Rgb([ir, ig, ib]);
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if max < x {
        max
    } else {
        x
    }
}
