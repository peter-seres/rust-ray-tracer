extern crate nalgebra;
mod color;
mod image;

use color::{Color, ColorData};
use image::{Image};

fn trace_image(trace_fn: &dyn Fn(u32, u32, u32, u32) -> Color) {
    let width = 1920;
    let height = 1080;

    let mut color_data = ColorData::new(vec![]);

    for j in 0..height {
        for i in 0..width {
            let c: Color = trace_fn(i, j, width, height);
            color_data.push(c);
        }
    }

    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(r"output/test.png").unwrap();
}

fn test_rainbow(i: u32, j: u32, width: u32, height: u32) -> Color {
    Color::from_floats(i as f32 / width as f32, j as f32 / height as f32, 0.9)
}

fn main() {
    let v = nalgebra::Vector3::new(1, 2, 3);
    println!("I made a vector: {:?}", v);

    println!("Make a test png file");
    trace_image(&test_rainbow);
}
