use image::{ImageBuffer, RgbImage};

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        viewport_height: f64,
        focal_length: f64,
        origin: Vec3,
    ) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Camera {
            aspect_ratio,
            image_width,
            image_height: ((image_width as f64) / aspect_ratio) as u32,
            viewport_height,
            viewport_width: aspect_ratio * viewport_height,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
