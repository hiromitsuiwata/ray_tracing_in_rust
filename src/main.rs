use std::f64::INFINITY;

use hittable::Hittable;
use image;

mod hitrecord;
mod hittable;
mod item;
mod ray;
mod vec3;

use item::Sphere;
use ray::Ray;
use vec3::{color, unit_vector, Vec3};

fn ray_color(ray: &Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    match sphere.hit(ray, 0.0, INFINITY) {
        None => {
            let unit_direction = unit_vector(ray.direction());
            let two = 2.0_f64;
            let sqrt2: f64 = two.sqrt();
            let t = (unit_direction.y() + 1.0 / sqrt2) / sqrt2;
            return color(1.0, 1.0, 1.0) * (1.0 - t) + color(0.5, 0.7, 1.0) * t;
        }
        Some(hit_record) => {
            let n = hit_record.normal();
            return color(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
        }
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    eprintln!("bottom_left_corner: {:?}", lower_left_corner);
    eprintln!(
        "top_right_corner: {:?}",
        lower_left_corner + horizontal + vertical
    );

    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let height = (IMAGE_HEIGHT - 1) as f64;
    let width = (IMAGE_WIDTH - 1) as f64;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = (x as f64) / width;
        // カメラの座標系では左上が(0, 0)なためy軸の向きが逆になっている
        let v = (height - (y as f64)) / height;
        let ray = Ray::new(
            origin,
            lower_left_corner + horizontal * u + vertical * v - origin,
        );

        let color = ray_color(&ray);

        let ir = (255.999 * color.r()) as u8;
        let ig = (255.999 * color.g()) as u8;
        let ib = (255.999 * color.b()) as u8;
        *pixel = image::Rgb([ir, ig, ib]);

        if y == IMAGE_HEIGHT - 1 {
            eprintln!("{:.1}%", 100.0 * (x as f64) / ((IMAGE_WIDTH - 1) as f64));
        }
    }
    img.save("result.png").unwrap();
}
