use image;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let height = (IMAGE_HEIGHT - 1) as f64;
    let width = (IMAGE_WIDTH - 1) as f64;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x as f64) / width;
        let g = ((IMAGE_HEIGHT - y) as f64) / height;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }
    img.save("result.png").unwrap();
}
