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
mod ray;
mod vec3;

use camera::Camera;
use color::*;
use item::Sphere;
use rand::Rng;
use ray::Ray;
use vec3::{color, origin, unit_vector, Vec3};

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    let iter = world.iter();

    // 物体に衝突する場合
    let mut closest_record: HitRecord = HitRecord::not_hit();

    // カメラに最も近い物体のHitRecordを探す
    let mut tmax = INFINITY;
    for item in iter {
        match item.hit(ray, 0.0, tmax) {
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
        // 物体の色
        let n = closest_record.normal();
        return (n + color(1.0, 1.0, 1.0)) * 0.5;
    }
}

fn main() {
    // 定数設定
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 225;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const HEIGHT: f64 = (IMAGE_HEIGHT - 1) as f64;
    const WIDTH: f64 = (IMAGE_WIDTH - 1) as f64;
    const SAMPLE_PER_PIXEL: u32 = 20;
    const NUM_OF_PIXELS: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 2.0, 1.0, origin());

    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // 物体を配置
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(-0.2, 0.5, -1.6), 0.4)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    world.push(Box::new(Sphere::new(Vec3::new(0.2, 0.1, -0.6), 0.1)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.3)));

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
            let color = ray_color(&ray, &world);
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
