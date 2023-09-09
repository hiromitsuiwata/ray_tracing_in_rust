#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
    None,
}

impl Material {
    pub fn shlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r1 = r0 * r0;
        r1 + (1.0 - r1) * (1.0 - cosine).powi(5)
    }
}
