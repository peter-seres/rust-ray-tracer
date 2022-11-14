use crate::{Vector3, Scalar};

pub struct Material {
    albedo: Vector3,
    roughness: Scalar,
    metallic: Scalar,
}

pub struct Sphere {
    position: Vector3,
    radius: Scalar,
    material_index: u32
}

pub struct Light {
    position: Vector3,
    strength: Scalar
}

pub struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
    materials: Vec<Material>,
}
