use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use rust_ray_tracer::Vector3;
use rust_ray_tracer::image::push_color;
use rust_ray_tracer::image::save_image_to_png;

fn main() {
    println!("Hello world");

    let mut image: Vec<u8> = vec![];

    const height: u32 = 80;
    const width: u32 = 80;

    for y in 0..height {
        for x in 0..width {
            let color = Vector3::zeros();
            push_color(&mut image, &color)
        }
    }

    let file_path = "test_black.png";
    save_image_to_png(file_path, width, height, &image);

}