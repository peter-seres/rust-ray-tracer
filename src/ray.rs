use crate::{Unit, Scalar, Point, Vector3, NORM_EPS};


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

    pub fn at_distance(&self, d: Scalar) -> Point {
        self.origin + d * self.dir.as_ref()
    }
}
