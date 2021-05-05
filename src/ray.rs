extern crate nalgebra as na;
use na::{Unit, Vector3};


#[derive(Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    dir: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { 
            origin, 
            dir: Unit::try_new(direction, 1e-10).unwrap()
        }
    }

    // pub fn default() -> Ray {
    //     let default_dir = Vector3::<f32>::new(0.0, 0.0, -1.0);
    //     Ray {
    //         origin: Vector3::new(0.0, 0.0, 0.0),
    //         dir: Unit::try_new(default_dir, 1e-10).unwrap(),
    //     }
    // }

    pub fn at_distance(&self, d: f32) -> Vector3<f32> {
        self.origin + d * self.dir.as_ref()
    }
}