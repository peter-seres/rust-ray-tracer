extern crate nalgebra as na;
use na::{Unit, Vector3};
use crate::ray::Ray;

pub struct Camera {
    width: u32,
    height: u32,            
    fov: u16,      // field of view in degrees
    aspect_ratio: f32,
    lens_width: f32,
    lens_height: f32,
    iterator_state: u32
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: u16) -> Self {
        let lens_half_height = ((fov / 2) as f32).to_radians().tan();
        let aspect_ratio = width as f32 / height as f32;
        Self {
            width, 
            height,
            fov,
            aspect_ratio: aspect_ratio,
            lens_height: 2.0 * lens_half_height,
            lens_width: 2.0 * lens_half_height * aspect_ratio,
            iterator_state: 0
        }
    }
}

impl Iterator for Camera {
    type Item = Ray;

    fn next(&mut self) -> Option<Ray> {
        let i = self.iterator_state / self.height;
        let j = self.iterator_state % self.height;

        if i == self.width {
            None
        } else {
            self.iterator_state += 1;
            Some(Ray::default())
        }
    }
}