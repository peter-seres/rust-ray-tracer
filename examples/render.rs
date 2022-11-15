use rust_ray_tracer;
use rust_ray_tracer::camera::Camera;
use rust_ray_tracer::colors::BLUE;
use rust_ray_tracer::image::save_image_to_png;
use rust_ray_tracer::renderer::Renderer;
use rust_ray_tracer::scene::{Scene, Sphere, Material};
use rust_ray_tracer::directions::*;
use rust_ray_tracer::Vector3;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() {
    let camera = Camera::new(WIDTH, HEIGHT, 45);

    let sphere_0 = Box::new(Sphere {
        position: 3.0 * FORWARD,
        radius: 1.0,
        material_index: 0
    });

    let material_0 = Material {
        albedo: BLUE,
        metallic: 1.0,
        roughness: 1.0
    };

    let scene = Scene {
        hittables: vec![sphere_0],
        lights: vec![],
        materials: vec![material_0]
    };

    let mut renderer = Renderer::new(&camera, &scene);
    let image: Vec<u8> = renderer.render();

    let file_path = "output/test_render.png";
    save_image_to_png(file_path, WIDTH, HEIGHT, &image);

}