use crate::{Point, Scalar};

pub trait Light {
    fn get_origin(&self) -> Point;
    fn get_strength(&self) -> Scalar;
}

pub struct PointLight {
    origin: Point,
    strength: Scalar,
}

impl PointLight {
    pub fn new(origin: Point, strength: Scalar) -> Self {
        Self { origin, strength }
    }
}

impl Light for PointLight {
    fn get_origin(&self) -> Point {
        self.origin
    }

    fn get_strength(&self) -> Scalar {
        self.strength
    }
}
