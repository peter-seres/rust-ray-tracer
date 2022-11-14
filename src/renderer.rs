use crate::camera::Camera;
use crate::color::{push_color, scalar_to_u8};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::{Scalar, UnitVector3, Vector3, NORM_EPS};
use time::Instant;

const RECURSION_DEPTH: u8 = 3;

struct HitPayload {
    hit_distance: Scalar,
    world_position: Vector3,
    world_normal: UnitVector3,
    object_index: u32,
}

impl HitPayload {
    pub fn default() -> HitPayload {
        let direction = Vector3::new(1.0, 0.0, 0.0);

        HitPayload {
            hit_distance: 1.0,
            world_position: Vector3::new(0.0, 0.0, 0.0),
            world_normal: UnitVector3::try_new(direction, NORM_EPS).unwrap(),
            object_index: 0,
        }
    }
}

pub struct Renderer<'a> {
    active_camera: &'a Camera,
    active_scene: &'a Scene,
}

impl<'a> Renderer<'a> {
    pub fn new(camera: &'a Camera, scene: &'a Scene) -> Renderer<'a> {
        Renderer {
            active_camera: camera,
            active_scene: scene,
        }
    }

    pub fn render(&mut self) -> Vec<u8> {
        // Empty buffer
        let mut final_image: Vec<u8> = vec![];

        // Generate rays:
        let rays = self.active_camera.generate_rays();

        // Iterate through the Camera, do ray tracing and gather the color data
        println!("Starting iterations.");
        let start = Instant::now();
        for ray in rays {
            let color: Vector3 = self.per_pixel(&ray);
            push_color(&mut final_image, &color);
        }
        println!(
            "Ray Tracing finished. Computation time {:?}",
            Instant::now() - start
        );

        final_image
    }

    fn per_pixel(&self, ray: &Ray) -> Vector3 {
        Vector3::zeros()
    }

    fn trace_ray(&self, ray: &Ray) -> HitPayload {
        HitPayload::default()
    }

    fn closest_hit(&self, ray: &Ray, hit_distance: Scalar, object_index: u32) -> HitPayload {
        HitPayload::default()
    }

    fn miss(&self, ray: &Ray) -> HitPayload {
        HitPayload::default()
    }
}
