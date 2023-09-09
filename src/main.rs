use std::f32::INFINITY;
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
mod scene;
mod vec3;

use camera::Camera;
use color::*;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use vec3::{color, origin, random_f32, random_unit_vector, reflect, refract, unit_vector, Vec3};

use crate::{material::Material, scene::Scene};

fn ray_color(ray: &Ray, scene: &Vec<Box<dyn Hittable>>, depth: u32) -> Vec3 {
    // 反射回数が一定よりも多くなったら、その時点で追跡をやめる
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    let iter = scene.iter();

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
        let two = 2.0_f32;
        let sqrt2: f32 = two.sqrt();
        let t = (unit_direction.y() + 1.0 / sqrt2) / sqrt2;
        return color(1.0, 1.0, 1.0) * (1.0 - t) + color(0.5, 0.7, 1.0) * t;
    } else {
        // 物体に当たった場合
        match closest_record.material() {
            // 拡散マテリアル
            Material::Lambertian => {
                // 新しい向き先(拡散)
                let target = closest_record.normal() + random_unit_vector();
                // 跳ね返ったレイ
                let new_ray = Ray::new(closest_record.point(), target);
                return ray_color(&new_ray, scene, depth - 1) * closest_record.attenuation();
            }
            // 金属マテリアル
            Material::Metal => {
                // 新しい向き先(反射)
                let target = reflect(unit_vector(ray.direction()), closest_record.normal())
                    + random_unit_vector() * closest_record.metal_fuzz();
                // 跳ね返ったレイ
                let new_ray = Ray::new(closest_record.point(), target);
                return ray_color(&new_ray, scene, depth - 1) * closest_record.attenuation();
            }
            // 誘電体マテリアル
            Material::Dielectric => {
                // 新しい向き先(屈折)
                let ref_idx = 1.5;
                let etai_over_etat: f32;
                if closest_record.front_face() {
                    etai_over_etat = 1.0 / ref_idx;
                } else {
                    etai_over_etat = ref_idx;
                }
                let unit_direction = unit_vector(ray.direction());

                let a = -unit_direction.dot(closest_record.normal());
                let cos_theta: f32;
                if a < 1.0 {
                    cos_theta = a;
                } else {
                    cos_theta = 1.0;
                }

                let reflect_prob = Material::shlick(cos_theta, etai_over_etat);

                if random_f32(0.0, 1.0) < reflect_prob {
                    // 反射
                    let target = reflect(unit_vector(ray.direction()), closest_record.normal());
                    // 跳ね返ったレイ
                    let new_ray = Ray::new(closest_record.point(), target);
                    return ray_color(&new_ray, scene, depth - 1);
                }

                // 屈折
                let target = refract(unit_direction, closest_record.normal(), etai_over_etat);
                let new_ray = Ray::new(closest_record.point(), target);
                return ray_color(&new_ray, scene, depth - 1);
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
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    // 横幅
    // const IMAGE_WIDTH: u32 = 225;
    const IMAGE_WIDTH: u32 = 1024;
    // const IMAGE_WIDTH: u32 = 512;

    // アンチエイリアシングのためのサンプル数
    // const SAMPLE_PER_PIXEL: u32 = 20;
    const SAMPLE_PER_PIXEL: u32 = 100;

    // 反射回数の上限値。これ以上の反射が起きたら黒色とする
    // const MAX_DEPTH: u32 = 10;
    const MAX_DEPTH: u32 = 50;

    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const HEIGHT: f32 = (IMAGE_HEIGHT - 1) as f32;
    const WIDTH: f32 = (IMAGE_WIDTH - 1) as f32;
    const NUM_OF_PIXELS: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;

    // カメラ
    let look_from = Vec3::new(12.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        25.0,
        ASPECT_RATIO,
        0.1,
        dist_to_focus,
    );

    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let scene = Scene::random_scene();

    // 進捗
    let mut progress: u32 = 0;

    // カメラから見える画角の1ピクセルごとに色を決めていく
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // アンチエイリアシングのために乱数を使って少しずらした方向にレイをたくさん飛ばして色を平均化する
        // 平均を計算するために足しこむための変数
        let mut sum_of_colors = origin();
        // 乱数

        let colors: Vec<Vec3> = (0..SAMPLE_PER_PIXEL)
            .into_par_iter()
            .map(|_| {
                // 乱数を生成
                let mut rng = rand::thread_rng();
                let rand1 = rng.gen::<f32>();
                let rand2 = rng.gen::<f32>();

                // 画角の横座標
                let u = ((x as f32) + rand1) / (WIDTH - 1.0);
                // 画角の縦座標
                // 画角の座標系では左上が(0, 0)なためy軸の向きが逆になっている
                let v = ((HEIGHT - 1.0) - ((y as f32) + rand2)) / (HEIGHT - 1.0);
                let ray = camera.get_ray(u, v);

                // レイを飛ばして色を決める
                let color = ray_color(&ray, &scene, MAX_DEPTH);
                color
            })
            .collect();

        for c in colors {
            // 足しこむ
            sum_of_colors = sum_of_colors + c;
        }

        // ピクセルに色を塗る。サンプル数で割る
        write_color(pixel, sum_of_colors, SAMPLE_PER_PIXEL);

        // 進捗を表示
        if progress % (NUM_OF_PIXELS / 100) == 0 {
            println!("{:.0}%", 100.0 * (progress as f32) / NUM_OF_PIXELS as f32);
            io::stdout().flush().unwrap();
        }
        progress += 1;
    }
    img.save("result.png").unwrap();
}
