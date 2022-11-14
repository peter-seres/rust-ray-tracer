use crate::{Scalar, Vector3};

pub struct Material {
    albedo: Vector3,
    roughness: Scalar,
    metallic: Scalar,
}

pub struct Sphere {
    position: Vector3,
    radius: Scalar,
    material_index: u32,
}

pub struct Light {
    position: Vector3,
    strength: Scalar,
}

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub materials: Vec<Material>,
}
