extern crate nalgebra;
use nalgebra::Vector3;
use crate::ray::Ray;

pub struct Camera {
   
    width: u32,                 // Pixel count - U
    height: u32,                // Pixel count - V
    lens_size: (f32, f32),      // World space lens size
    pixel_size: (f32, f32),     // World space pixel size
    iterator_state: u32         // Iterator index state
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: u16) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let lens_height = 2.0 * ((fov / 2) as f32).to_radians().tan();
        let lens_width = lens_height * aspect_ratio;

        let x_step_size = lens_width / (width - 1) as f32;
        let y_step_size = lens_height / (height - 1) as f32;

        Self {
            width, 
            height,
            lens_size: (lens_width, lens_height),
            pixel_size: (x_step_size, y_step_size),
            iterator_state: 0,
        }
    }
}

impl Iterator for Camera {
    type Item = Ray;

    fn next(&mut self) -> Option<Ray> {
        let i = self.iterator_state % self.width;
        let j = self.iterator_state / self.width;

        if j == self.height {
            None
        } else {
            self.iterator_state += 1;
            let ray_origin = Vector3::<f32>::zeros();   // todo: camera position

            let x: f32 = - self.lens_size.0 / 2.0 + (i as f32) * self.pixel_size.0;
            let y: f32 = self.lens_size.1 / 2.0 - (j as f32) * self.pixel_size.1;

            let ray_dir = Vector3::<f32>::new(x, y, -1.0);
            Some(Ray::new(ray_origin, ray_dir))
        }
    }
}