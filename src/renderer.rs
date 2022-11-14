use crate::{Scalar, Vector3, UnitVector3, NORM_EPS};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::color::scalar_to_u8;
use crate::image::Image;
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
    active_camera: &'a mut Camera,
    active_scene: &'a Scene,
    final_image: Vec<u8>
}


impl<'a> Renderer<'a> {
    pub fn new(camera: &'a mut Camera, scene: &'a Scene) -> Renderer<'a> {
        Renderer { 
            active_camera: camera, 
            active_scene: scene, 
            final_image: vec![],
        }
    }

    pub fn push_color(&mut self, color: &Vector3) {
        self.final_image.push(scalar_to_u8(color.x));
        self.final_image.push(scalar_to_u8(color.y));
        self.final_image.push(scalar_to_u8(color.z));
    }

    pub fn render(&mut self, file_path: &str) {
    
        // Generate rays:
        let rays = self.active_camera.generate_rays();

        // Iterate through the Camera, do ray tracing and gather the color data
        println!("Starting iterations.");
        let start = Instant::now();
        for ray in rays {
            let color: Vector3 = self.per_pixel(&ray);
            self.push_color(&color);
        }
        println!("Ray Tracing finished. Computation time {:?}", Instant::now() - start);

        // save as png
        let im = Image::new(self.active_camera.width, self.active_camera.height, &self.final_image);
        im.save_as_png(file_path).unwrap();

        println!("Saved image to: {:?}", file_path);
    }

    fn per_pixel(&self, ray: &Ray) -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
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

