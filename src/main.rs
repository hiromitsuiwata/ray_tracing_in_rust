use std::f64::INFINITY;
use std::io;
use std::io::Write;

use hitrecord::HitRecord;
use hittable::Hittable;

mod camera;
mod color;
mod hitrecord;
mod hittable;
mod item;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use color::*;
use item::Sphere;
use rand::Rng;
use ray::Ray;
use vec3::{color, origin, random_f64, random_unit_vector, reflect, refract, unit_vector, Vec3};

use crate::material::Material;

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>, depth: u32) -> Vec3 {
    // 反射回数が一定よりも多くなったら、その時点で追跡をやめる
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    let iter = world.iter();

    // 物体に衝突する場合
    let mut closest_record: HitRecord = HitRecord::not_hit();

    // カメラに最も近い物体のHitRecordを探す
    let mut tmax = INFINITY;
    for item in iter {
        match item.hit(ray, 0.001, tmax) {
            None => {}
            Some(hit_record) => {
                if closest_record.t() > hit_record.t() {
                    closest_record = hit_record;
                    tmax = closest_record.t();
                }
            }
        }
    }

    if closest_record.t() == INFINITY {
        // tが無限大ということは何にも衝突しなかったという意味なので背景の色
        let unit_direction = unit_vector(ray.direction());
        let two = 2.0_f64;
        let sqrt2: f64 = two.sqrt();
        let t = (unit_direction.y() + 1.0 / sqrt2) / sqrt2;
        return color(1.0, 1.0, 1.0) * (1.0 - t) + color(0.5, 0.7, 1.0) * t;
    } else {
        // 物体に当たった場合
        match closest_record.material() {
            // 拡散マテリアル
            Material::Lambertian => {
                // 新しい向き先(拡散)
                let target =
                    closest_record.point() + closest_record.normal() + random_unit_vector();
                // 跳ね返ったレイ
                let new_ray = Ray::new(closest_record.point(), target - closest_record.point());
                return ray_color(&new_ray, world, depth - 1) * closest_record.attenuation();
            }
            // 金属マテリアル
            Material::Metal => {
                // 新しい向き先(反射)
                let target = reflect(unit_vector(ray.direction()), closest_record.normal())
                    + random_unit_vector() * closest_record.metal_fuzz();
                // 跳ね返ったレイ
                let new_ray = Ray::new(closest_record.point(), target - closest_record.point());
                return ray_color(&new_ray, world, depth - 1) * closest_record.attenuation();
            }
            // 誘電体マテリアル
            Material::Dielectric => {
                // 新しい向き先(屈折)
                let ref_idx = 1.7;
                let etai_over_etat: f64;
                if closest_record.front_face() {
                    etai_over_etat = 1.0 / ref_idx;
                } else {
                    etai_over_etat = ref_idx;
                }
                let unit_direction = unit_vector(ray.direction());

                let a = -unit_direction.dot(closest_record.normal());
                let cos_theta: f64;
                if a < 1.0 {
                    cos_theta = a;
                } else {
                    cos_theta = 1.0;
                }

                let reflect_prob = Material::shlick(cos_theta, etai_over_etat);

                if random_f64(0.0, 1.0) < reflect_prob {
                    // 反射
                    let target = reflect(unit_vector(ray.direction()), closest_record.normal());
                    // 跳ね返ったレイ
                    let new_ray = Ray::new(closest_record.point(), target - closest_record.point());
                    return ray_color(&new_ray, world, depth - 1);
                }

                // 屈折
                let target = refract(unit_direction, closest_record.normal(), etai_over_etat);
                let new_ray = Ray::new(closest_record.point(), target - closest_record.point());
                return ray_color(&new_ray, world, depth - 1);
            }
            // 未指定
            Material::None => {
                return color(1.0, 0.0, 0.0);
            }
        }
    }
}

fn main() {
    // 定数設定

    // アスペクト比
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // 横幅
    const IMAGE_WIDTH: u32 = 225;
    // const IMAGE_WIDTH: u32 = 1024;
    // const IMAGE_WIDTH: u32 = 512;

    // アンチエイリアシングのためのサンプル数
    const SAMPLE_PER_PIXEL: u32 = 20;
    // const SAMPLE_PER_PIXEL: u32 = 100;

    // 反射回数の上限値。これ以上の反射が起きたら黒色とする
    // const MAX_DEPTH: u32 = 10;
    const MAX_DEPTH: u32 = 50;

    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const HEIGHT: f64 = (IMAGE_HEIGHT - 1) as f64;
    const WIDTH: f64 = (IMAGE_WIDTH - 1) as f64;
    const NUM_OF_PIXELS: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;

    // カメラ
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        IMAGE_WIDTH,
        2.0,
        dist_to_focus,
    );

    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // 物体を配置
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian,
        color(0.3, 0.3, 0.7),
        0.0,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian,
        color(0.8, 0.8, 0.0),
        0.0,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal,
        color(0.8, 0.6, 0.2),
        0.0,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-0.5, 0.0, -1.0),
        0.5,
        Material::Dielectric,
        color(1.0, 1.0, 1.0),
        0.0,
    )));

    // 進捗
    let mut progress: u32 = 0;

    // カメラから見える画角の1ピクセルごとに色を決めていく
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // アンチエイリアシングのために乱数を使って少しずらした方向にレイをたくさん飛ばして色を平均化する
        // 平均を計算するために足しこむための変数
        let mut sum_of_colors = origin();
        // 乱数
        let mut rng = rand::thread_rng();
        for _ in 0..SAMPLE_PER_PIXEL {
            // 乱数を生成
            let rand1 = rng.gen::<f64>();
            let rand2 = rng.gen::<f64>();

            // 画角の横座標
            let u = ((x as f64) + rand1) / (WIDTH - 1.0);
            // 画角の縦座標
            // 画角の座標系では左上が(0, 0)なためy軸の向きが逆になっている
            let v = ((HEIGHT - 1.0) - ((y as f64) + rand2)) / (HEIGHT - 1.0);
            let ray = camera.get_ray(u, v);

            // レイを飛ばして色を決める
            let color = ray_color(&ray, &world, MAX_DEPTH);
            // 足しこむ
            sum_of_colors = sum_of_colors + color;
        }

        // ピクセルに色を塗る。サンプル数で割る
        write_color(pixel, sum_of_colors, SAMPLE_PER_PIXEL);

        // 進捗を表示
        if progress % (NUM_OF_PIXELS / 20) == 0 {
            println!("{:.0}%", 100.0 * (progress as f64) / NUM_OF_PIXELS as f64);
            io::stdout().flush().unwrap();
        }
        progress += 1;
    }
    img.save("result.png").unwrap();
}
