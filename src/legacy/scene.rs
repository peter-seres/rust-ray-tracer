use crate::{Hittable, Light, Camera, Sphere, PointLight, InfPlane, Vector3};
use crate::consts::*;

pub type ObjectList = Vec<Box<dyn Hittable>>;
pub type LightList = Vec<Box<dyn Light>>;

pub struct Scene {
    pub camera: Camera,
    pub objects: ObjectList,
    pub lights: LightList,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn push_object(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn push_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn default_scene(camera: Camera) -> Self {

        let p = InfPlane::new(Vector3::new(0.0, -1.0, 0.0), UP, LIGHTGRAY);

        let s1 = Sphere::new(Vector3::new(-1.0, 0.0, -5.0), 1.0, RED);
        let s2 = Sphere::new(Vector3::new(1.5, 0.5, -5.0), 1.5, BLUE);
        let s3 = Sphere::new(Vector3::new(-1.5, -0.5, -3.0), 0.5, GREEN);
        let s4 = Sphere::new(Vector3::new(0.0, -0.82, -2.5), 0.2, TEAL);
        let s5 = Sphere::new(Vector3::new(0.8, -0.6, -3.0), 0.4, PINK);

        let l1 = PointLight::new(Vector3::new(-2.0, 3.3, -1.0), 2.0);
        let l2 = PointLight::new(Vector3::new(0.0, -0.7, -3.0), 1.0);

        Scene {
            camera,
            objects: vec![p, s1, s2, s3, s4, s5],
            lights: vec![l1, l2]
        }
    }

    pub fn single_sphere_two_lights(camera: Camera) -> Self {

        let p = InfPlane::new(Vector3::new(0.0, -1.0, 0.0), UP, LIGHTGRAY);
        let s = Sphere::new(Vector3::new(-0.0, 0.0, -3.0), 1.0, SKYBLUE);
        let l1 = PointLight::new(Vector3::new(-2.0, 3.0, -1.0), 3.0);
        let l2 = PointLight::new(Vector3::new(2.0, 3.0, -1.0), 3.0);

        Scene {
            camera,
            objects: vec![p, s],
            lights: vec![l1]//l2]
        }
    }

}