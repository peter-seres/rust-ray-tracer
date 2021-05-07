mod camera;
mod color;
mod consts;
mod image;
mod ray;
mod types;

use camera::Camera;
use color::{Color, ColorData};
pub use consts::*;
use image::Image;
use ray::Ray;
pub use types::*;

fn raycast(ray: Ray) -> Color {
    let sky_blue: Color = Color::new(0.5, 0.7, 1.0);
    let white: Color = Color::new(1.0, 1.0, 1.0);

    let sky_scaler = ray.dir[1];

    let c = ((1.0 - sky_scaler) * sky_blue) + (white * sky_scaler);
    c

    // Color::new(125, 125, 255)
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
