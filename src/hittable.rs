use std::fmt;

use crate::hitrecord::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

impl fmt::Debug for dyn Hittable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hittable");
        Ok(())
    }
}
