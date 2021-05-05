extern crate nalgebra as na;
mod color;
mod image;
mod camera;
mod ray;
use color::{Color, ColorData};
use image::Image;
use camera::Camera;
use ray::Ray;


fn raycast(r: Ray) -> Color {
    Color::new(125, 125, 255)
}

fn main() {
    // Set image resolution
    let width = 192;
    let height = 108;
    let file_path = r"output/traced.png";

    // Camera setup:
    let c = Camera::new(width, height, 45);

    // Data allocation into Vector:
    let mut color_data = ColorData::new(vec![]);

    // Iterate through the Camera, do ray tracing and gather the color data
    for ray in c {
        let c = raycast(ray);
        color_data.push(c);
    }

    // Save the color data to image
    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(file_path).unwrap();
}
