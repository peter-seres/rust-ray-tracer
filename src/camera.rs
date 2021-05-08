use crate::{Ray, Scalar, Vector3};

pub struct Camera {
    width: u32,                   // Pixel count - U
    height: u32,                  // Pixel count - V
    lens_size: (Scalar, Scalar),  // World space lens size
    pixel_size: (Scalar, Scalar), // World space pixel size
    iterator_state: u32,          // Iterator index state
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: u16) -> Self {
        let aspect_ratio = width as Scalar / height as Scalar;
        let lens_height = 2.0 * ((fov / 2) as Scalar).to_radians().tan();
        let lens_width = lens_height * aspect_ratio;

        let x_step_size = lens_width / (width - 1) as Scalar;
        let y_step_size = lens_height / (height - 1) as Scalar;

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
            let ray_origin = Vector3::zeros(); // todo: camera position

            let x = -self.lens_size.0 / 2.0 + (i as Scalar) * self.pixel_size.0;
            let y = self.lens_size.1 / 2.0 - (j as Scalar) * self.pixel_size.1;

            let ray_dir = Vector3::new(x, y, -1.0);
            Some(Ray::new(ray_origin, ray_dir))
        }
    }
}
