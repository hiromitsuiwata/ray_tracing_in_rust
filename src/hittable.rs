use crate::hitrecord::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}
