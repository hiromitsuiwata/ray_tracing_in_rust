use std::f64::INFINITY;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    front_face: bool,
    t: f64,
    material: Material,
    attenuation: Vec3,
    metal_fuzz: f64,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        front_face: bool,
        t: f64,
        material: Material,
        attenuation: Vec3,
        metal_fuzz: f64,
    ) -> HitRecord {
        HitRecord {
            point,
            normal,
            front_face,
            t,
            material,
            attenuation,
            metal_fuzz,
        }
    }

    pub fn not_hit() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            t: INFINITY,
            material: Material::None,
            attenuation: Vec3::new(0.0, 0.0, 0.0),
            metal_fuzz: 0.0,
        }
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn attenuation(&self) -> Vec3 {
        self.attenuation
    }

    pub fn metal_fuzz(&self) -> f64 {
        self.metal_fuzz
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
