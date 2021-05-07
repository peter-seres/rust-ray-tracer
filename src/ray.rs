use crate::{Unit, Vector3, NORM_EPS};


#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub dir: Unit<Vector3>,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            dir: Unit::try_new(direction, NORM_EPS).unwrap(),
        }
    }

    // pub fn default() -> Ray {
    //     let default_dir = Vector3::<f32>::new(0.0, 0.0, -1.0);
    //     Ray {
    //         origin: Vector3::new(0.0, 0.0, 0.0),
    //         dir: Unit::try_new(default_dir, 1e-10).unwrap(),
    //     }
    // }

    // pub fn at_distance(&self, d: f32) -> Vector3<f32> {
    //     self.origin + d * self.dir.as_ref()
    // }
}
