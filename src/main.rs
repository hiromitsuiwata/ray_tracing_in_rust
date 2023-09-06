fn main() {
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    let height = (IMAGE_HEIGHT - 1) as f64;
    let width = (IMAGE_WIDTH - 1) as f64;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / width;
            let g = (j as f64) / height;
            let b = 0.25;

            let ir = (255.999 * r) as u16;
            let ig = (255.999 * g) as u16;
            let ib = (255.999 * b) as u16;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
