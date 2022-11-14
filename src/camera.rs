use crate::{Scalar, Vector3};
use crate::ray::Ray;

pub struct Camera {
    position: Vector3,
    pub width: u32,                   // Pixel count - U
    pub height: u32,                  // Pixel count - V
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

    pub fn generate_rays(&mut self) -> Vec<Ray> {

        let mut rays: Vec<Ray> = vec![];
        let mut iterator_state = 0;

        let i = iterator_state % self.width;
        let j = iterator_state / self.width;

        if j != self.height {
            iterator_state += 1;
            let ray_origin = self.position; // todo: camera position

            let x = -self.lens_size.0 / 2.0 + (i as Scalar) * self.pixel_size.0;
            let y = self.lens_size.1 / 2.0 - (j as Scalar) * self.pixel_size.1;

            let ray_dir = Vector3::new(x, y, -1.0);
            rays.push(Ray::new(ray_origin, ray_dir));
        }

        rays
    }
}
