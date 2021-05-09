use crate::{Normal, Point, Scalar, Vector3, LAMBERT_INT};

pub type LightList = [Box<dyn Light>];

pub trait Light {
    fn get_origin(&self) -> Point;
    fn get_intensity(&self, point: Point, normal: Normal) -> Scalar;
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

    fn get_intensity(&self, point: Point, normal: Normal) -> Scalar {
        let vector_to_light: Vector3 = self.origin - point;
        let distance: Scalar = vector_to_light.norm();
        let intensity: Scalar = self.strength / (distance * distance);
        let lambert_intensity: Scalar = intensity * LAMBERT_INT * vector_to_light.dot(&normal);
        lambert_intensity
    }
}
