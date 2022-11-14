use std::vec;

use rust_ray_tracer;
use rust_ray_tracer::camera::Camera;
use rust_ray_tracer::image::save_image_to_png;
use rust_ray_tracer::renderer::Renderer;
use rust_ray_tracer::scene::Scene;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 80;

fn main() {
    rust_ray_tracer::hello();

    // Setup camera
    let camera = Camera::new(WIDTH, HEIGHT, 45);

    // Setup scene
    let scene = Scene {
        spheres: vec![],
        lights: vec![],
        materials: vec![],
    };

    // Render
    let mut renderer = Renderer::new(&camera, &scene);
    let buffer: Vec<u8> = renderer.render();

    // Save the image
    let file_path = r"output/traced.png";
    // let im = Image::new(WIDTH, HEIGHT, &buffer);
    // im.save_as_png(file_path).unwrap();

    save_image_to_png(&file_path, WIDTH, HEIGHT, &buffer);
}
