mod camera;
mod color;
mod consts;
mod image;
mod ray;
mod types;

pub use camera::Camera;
pub use color::{Color, ColorData};
pub use consts::*;
pub use image::Image;
pub use ray::Ray;
pub use types::*;

// Pre-define a few colors.
const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const BLACK: Color = Color::new(0.0, 0.0, 0.0);
const GREY: Color = Color::new(0.5, 0.5, 0.5);
const SKYBLUE: Color = Color::new(0.5, 0.7, 1.0);
const RED: Color = Color::new(1.0, 0.1, 0.1);


fn raycast(ray: Ray) -> Color {
    let sky_scaler = ray.dir[1];
    (1.0 - sky_scaler) * WHITE + sky_scaler * SKYBLUE
}


fn main() {
    // Set image resolution and ouput path:
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
