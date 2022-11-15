use crate::camera::Camera;
use crate::hit::Hittable;
use crate::image::push_color;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::{Scalar, UnitVector3, Vector3, NORM_EPS, hit};
use crate::colors::*;
use time::Instant;

const BOUNCES: u8 = 1;
const MULTIPLIER: Scalar = 1.0;
const RAY_MAX_DIST: Scalar = 1e3;
const SHADOW_BIAS: Scalar = 2e-3;
const REFLECTION_INT: Scalar = 0.8;
const LAMBERT_INT: Scalar = 0.8;
const AMBIENT_INT: Scalar = 0.0;


struct HitPayload {
    hit_distance: Scalar,
    world_position: Vector3,
    world_normal: UnitVector3,
    object_index: usize,
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
    
    // Constructor
    pub fn new(camera: &'a Camera, scene: &'a Scene) -> Renderer<'a> {
        Renderer {
            active_camera: camera,
            active_scene: scene,
        }
    }

    // Set different active camera
    pub fn set_active_camera(&mut self, camera: &'a Camera) {
        self.active_camera = camera
    }

    // Set different active camera
    pub fn set_active_scene(&mut self, scene: &'a Scene) {
        self.active_scene = scene
    }

    // Render the image from the active camera and active scene
    pub fn render(&mut self) -> Vec<u8> {

        // Create empty buffer
        let mut final_image: Vec<u8> = vec![];

        // Log & Start timer
        println!("Starting iterations.");
        let start = Instant::now();

        // Iterate through each pixel index (i, j)
        for j in 0..self.active_camera.height {
            for i in 0..self.active_camera.width {
                let color: Vector3 = self.per_pixel(i, j);
                push_color(&mut final_image, &color);
            }
        }

        // Log & End timer
        println!(
            "Ray Tracing finished. Computation time {}",
            Instant::now() - start
        );

        final_image
    }

    fn per_pixel(&self, i:u32, j: u32) -> Vector3 {
        // Rainbow test:

        // let r = i as Scalar / self.active_camera.width as Scalar;
        // let g = j as Scalar / self.active_camera.height as Scalar;
        // let b = 1.0;
        // Vector3::new(r, g, b)

        let ray: Ray = self.active_camera.generate_ray(i, j);
        
        let mut color = BLACK;

        for i in 0..BOUNCES {

            let hit_color = match self.trace_ray(&ray) {
                None => BLACK,
                Some(hit) => WHITE,
            };

            color += hit_color * REFLECTION_INT;
        }
        color
    }

    fn trace_ray(&self, ray: &Ray) -> Option<HitPayload> {
        
        // Initialize the distance far away
        let mut hit_distance: Scalar = Scalar::MAX;
        let mut closest_object_index: Option<usize> = None;

        for (idx, h) in self.active_scene.hittables.iter().enumerate() {
            
            match h.intersect(&ray) {
            Some(distance) => {
                if distance < hit_distance && distance > 0.0 {
                    hit_distance = distance;
                    closest_object_index = Some(idx);
                }
            },
            None => {
                continue;
            }
            }
        }


        match closest_object_index {
            // Miss
            None => None,
            
            // Closest hit
            Some(idx) => {

                // Fetch the object reference
                let object = &self.active_scene.hittables[idx];

                // World position of the hit location
                let p = ray.origin + hit_distance * *ray.dir;

                // Pack it into payload
                Some(HitPayload{
                    hit_distance,
                    world_position: p,
                    world_normal: object.get_normal(p),
                    object_index: idx
                })
            }
        }
    }
}
