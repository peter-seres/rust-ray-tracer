use rust_ray_tracer;
use rust_ray_tracer::camera::Camera;
use rust_ray_tracer::renderer::Renderer;
use rust_ray_tracer::scene::Scene;


fn main() {
    rust_ray_tracer::hello();

    let camera = Camera::new(1920, 1080, 45);

    let scene = Scene {};

    let renderer = Renderer::new();

}