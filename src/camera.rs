use std::f32::consts::PI;

use crate::ray::Ray;
use crate::vec3::{random_in_unit_disk, unit_vector, Vec3};

pub struct Camera {
    look_from: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32, // 垂直方向の視野角(弧度法)
        aspect_ratio: f32,
        aperture: f32,   // 絞り
        focus_dist: f32, // 焦点距離
    ) -> Camera {
        let theta = vfov / 180.0 * PI;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // カメラ座標系におけるz軸(カメラが見ている方向とは逆向きになっている)
        let w = unit_vector(look_from - look_at);
        // カメラ座標系におけるx軸(左から右向き)
        let u = unit_vector(vup.cross(w));
        // カメラ座標系におけるy軸(下から上向き)
        let v = w.cross(u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = look_from - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            look_from,
            horizontal,
            vertical,
            u,
            v,
            lower_left_corner,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.look_from + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.look_from
                - offset,
        )
    }
}
