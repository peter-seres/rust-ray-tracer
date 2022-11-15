use crate::{Scalar, Vector3};
use crate::hit::Hittable;

pub struct Material {
    pub albedo: Vector3,
    pub roughness: Scalar,
    pub metallic: Scalar,
}

pub struct Sphere {
    pub position: Vector3,
    pub radius: Scalar,
    pub material_index: u32,
}

pub struct Light {
    pub position: Vector3,
    pub strength: Scalar,
}

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
    pub materials: Vec<Material>,
}
