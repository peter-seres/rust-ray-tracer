use crate::{Point, Scalar, Unit, Vector3, NORM_EPS, Normal};

#[derive(Debug, Copy, Clone)]
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

    pub fn at_distance(&self, d: Scalar) -> Point {
        self.origin + d * self.dir.as_ref()
    }

    pub fn reflect(&self, point: Point, normal: Normal) -> Ray {
        let reflected_dir = *self.dir - 2.0 * normal * self.dir.dot(&normal);
        Ray::new(point, reflected_dir)
    }
}
