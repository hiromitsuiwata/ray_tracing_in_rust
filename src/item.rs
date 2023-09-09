use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
    attenuation: Vec3,
    metal_fuzz: f32,
}

impl Sphere {
    pub fn new(
        center: Vec3,
        radius: f32,
        material: Material,
        attenuation: Vec3,
        metal_fuzz: f32,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            attenuation,
            metal_fuzz,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let b = oc.dot(ray.direction()) * 2.0;
        let c = oc.length_squared() - self.radius * self.radius;
        // 判別式が正の場合、二次方程式に２つの解がある。レイが球を貫いている
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // 解がない場合はNoneを返す
            None
        } else {
            // 解がある場合はtが小さいほう(原点から近いほう)のHitRecordを返す
            let t: f32 = (-b - discriminant.sqrt()) / (2.0 * a);

            if tmin < t && t < tmax {
                let point = ray.at(t);
                let outward_normal = (point - self.center) / self.radius;

                let mut hit_record = HitRecord::new(
                    point,
                    outward_normal,
                    true,
                    t,
                    self.material,
                    self.attenuation,
                    self.metal_fuzz,
                );

                hit_record.set_face_normal(ray, outward_normal);
                Some(hit_record)
            } else {
                None
            }
        }
    }
}
