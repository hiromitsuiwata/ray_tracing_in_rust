use image::Rgb;

use crate::vec3::Vec3;

pub fn write_color(pixel: &mut Rgb<u8>, color: Vec3) {
    let ir = (255.999 * color.r()) as u8;
    let ig = (255.999 * color.g()) as u8;
    let ib = (255.999 * color.b()) as u8;
    *pixel = image::Rgb([ir, ig, ib]);
}
