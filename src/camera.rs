use crate::ray::Ray;
use crate::{Scalar, Vector3};

pub struct Camera {
    position: Vector3,
    pub width: u32,               // Pixel count - U
    pub height: u32,              // Pixel count - V
    lens_size: (Scalar, Scalar),  // World space lens size
    pixel_size: (Scalar, Scalar), // World space pixel size
}

// todo: Projection, View, InverProjection, InverseView (matrix 4x4)
// todo: NearClip / FarClip, VerticalFOV

impl Camera {
    pub fn new(width: u32, height: u32, fov: u16) -> Self {
        let aspect_ratio = width as Scalar / height as Scalar;
        let lens_height = 2.0 * ((fov / 2) as Scalar).to_radians().tan();
        let lens_width = lens_height * aspect_ratio;

        let x_step_size = lens_width / (width - 1) as Scalar;
        let y_step_size = lens_height / (height - 1) as Scalar;

        Self {
            position: Vector3::zeros(),
            width,
            height,
            lens_size: (lens_width, lens_height),
            pixel_size: (x_step_size, y_step_size),
        }
    }

    fn ray_direction_per_pixel(&self, i: u32, j: u32) -> Vector3 {

        let x = -self.lens_size.0 / 2.0 + (i as Scalar) * self.pixel_size.0;
        let y = self.lens_size.1 / 2.0 - (j as Scalar) * self.pixel_size.1;

        Vector3::new(x, y, -1.0)
    }

    pub fn generate_ray(&self, i: u32, j: u32) -> Ray {
        let ray_dir = self.ray_direction_per_pixel(i, j);
        Ray::new(self.position, ray_dir)
    }
}
