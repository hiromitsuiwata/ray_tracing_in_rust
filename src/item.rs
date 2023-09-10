use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center0: Vec3,
    center1: Vec3,
    radius: f32,
    material: Material,
    attenuation: Vec3,
    metal_fuzz: f32,
    time0: f32,
    time1: f32,
}

impl Sphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        radius: f32,
        material: Material,
        attenuation: Vec3,
        metal_fuzz: f32,
        time0: f32,
        time1: f32,
    ) -> Sphere {
        Sphere {
            center0,
            center1,
            radius,
            material,
            attenuation,
            metal_fuzz,
            time0,
            time1,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0)))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, time: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(time);
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
                let outward_normal = (point - self.center(time)) / self.radius;

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
