use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, front_face: bool) -> HitRecord {
        HitRecord {
            point,
            normal,
            front_face,
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal
        } else {
            self.normal = -outward_normal;
        }
    }
}
