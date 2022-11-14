use crate::{Scalar, Vector3, UnitVector3, NORM_EPS};

pub struct Ray {
    pub origin: Vector3,
    pub dir: UnitVector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            dir: UnitVector3::try_new(direction, NORM_EPS).unwrap(),
        }
    }

    pub fn at_distance(&self, d: Scalar) -> Vector3 {
        self.origin + d * self.dir.as_ref()
    }

    pub fn reflect(&self, point: Vector3, normal: UnitVector3) -> Ray {
        let reflected_dir = *self.dir - 2.0 * *normal * self.dir.dot(&normal);
        Ray::new(point, reflected_dir)
    }
}
