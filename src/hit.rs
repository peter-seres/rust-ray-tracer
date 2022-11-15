use crate::{UnitVector3, Vector3, Scalar, NORM_EPS};
use crate::scene::Sphere;
use crate::ray::Ray;

pub trait Hittable {
    fn get_normal(&self, p: Vector3) -> UnitVector3;
    fn intersect(&self, ray: &Ray) -> Option<Scalar>;
}

impl Hittable for Sphere {
    fn get_normal(&self, p: Vector3) -> UnitVector3 {
        UnitVector3::try_new(p - self.position, NORM_EPS).unwrap()
    }

    fn intersect(&self, ray: &Ray) -> Option<Scalar> {
        // Vector pointing from Sphere origin to Ray origin
        let sphere_to_ray = ray.origin - self.position;

        // Second order equation terms:
        let a: Scalar = ray.dir.dot(&ray.dir);
        let b: Scalar = 2.0 * sphere_to_ray.dot(&ray.dir);
        let c: Scalar = sphere_to_ray.dot(&sphere_to_ray) - self.radius * self.radius;

        // Look for solutions of the second order equation:
        let discriminant: Scalar = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {

            let mut numerator: Scalar = -b - discriminant.sqrt();

            if numerator > 0.0 {
                let distance: Scalar = numerator / (2.0 * a);
                let p = ray.at_distance(distance);
                let normal = self.get_normal(p);
                Some(distance)
            } else {
                numerator = -b + discriminant.sqrt();

                if numerator > 0.0 {
                    let distance: Scalar = numerator / (2.0 * a);
                    let p = ray.at_distance(distance);
                    let normal = self.get_normal(p);
                    Some(distance)
                } else {
                    None
                }
            }
        }
    }
}