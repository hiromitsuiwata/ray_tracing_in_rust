use crate::{
    hittable::Hittable,
    item::Sphere,
    material::Material,
    vec3::{color, random_color, random_color_range, random_f32, Vec3},
};

pub struct Scene {}

impl Scene {
    pub fn random_scene() -> Vec<Box<dyn Hittable>> {
        // 物体を配置
        let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

        // 地面
        scene.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::Lambertian,
            color(0.5, 0.5, 0.5),
            0.0,
            0.0,
            1.0,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_f32(0.0, 1.0);
                let center = Vec3::new(
                    a as f32 + 0.9 * random_f32(0.0, 1.0),
                    0.2,
                    b as f32 + 0.9 * random_f32(0.0, 1.0),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.6 {
                        // 拡散マテリアル(静止)
                        let albedo = random_color() * random_color();
                        scene.push(Box::new(Sphere::new(
                            center,
                            center,
                            0.2,
                            Material::Lambertian,
                            albedo,
                            0.0,
                            0.0,
                            1.0,
                        )));
                    } else if choose_mat < 0.8 {
                        // 拡散マテリアル(モーションあり)
                        let albedo = random_color() * random_color();
                        scene.push(Box::new(Sphere::new(
                            center,
                            center + Vec3::new(0.0, random_f32(0.0, 0.5), 0.0),
                            0.2,
                            Material::Lambertian,
                            albedo,
                            0.0,
                            0.0,
                            10.0,
                        )));
                    } else if choose_mat < 0.95 {
                        // 金属マテリアル
                        let albedo = random_color_range(0.5, 1.0);
                        scene.push(Box::new(Sphere::new(
                            center,
                            center,
                            0.2,
                            Material::Metal,
                            albedo,
                            random_f32(0.0, 0.5),
                            0.0,
                            1.0,
                        )));
                    } else {
                        // 誘電体マテリアル
                        scene.push(Box::new(Sphere::new(
                            center,
                            center,
                            0.2,
                            Material::Dielectric,
                            color(1.0, 1.0, 1.0),
                            0.0,
                            0.0,
                            1.0,
                        )));
                    }
                }
            }
        }

        scene.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Material::Dielectric,
            color(1.0, 1.0, 1.0),
            0.0,
            0.0,
            1.0,
        )));
        scene.push(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::Lambertian,
            color(0.4, 0.2, 0.1),
            0.0,
            0.0,
            1.0,
        )));
        scene.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Material::Metal,
            color(0.7, 0.6, 0.5),
            0.0,
            0.0,
            1.0,
        )));

        scene
    }
}
