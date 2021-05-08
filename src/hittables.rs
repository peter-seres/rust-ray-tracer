use crate::{Color, Normal, Point, Ray, Scalar, Vector3};

pub trait Hittable {
    fn get_normal(&self, p: Point) -> Normal;
    fn intersect(&self, ray: &Ray) -> Option<(Scalar, Color, Normal)>;
}

pub struct Sphere {
    origin: Point,
    radius: Scalar,
    color: Color,
}

impl Sphere {
    pub fn new(origin: Vector3, radius: Scalar, color: Color) -> Self {
        Self {
            origin,
            radius,
            color,
        }
    }
}

pub struct InfPlane {
    origin: Point,
    normal: Normal,
    color: Color,
}

impl InfPlane {
    pub const HIT_EPS: Scalar = 1e-4;

    pub fn new(origin: Vector3, normal: Normal, color: Color) -> Self {
        Self {
            origin,
            normal,
            color,
        }
    }
}

impl Hittable for Sphere {
    fn get_normal(&self, p: Point) -> Normal {
        p - self.origin
    }

    fn intersect(&self, ray: &Ray) -> Option<(Scalar, Color, Normal)> {
        // Vector pointing from Sphere origin to Ray origin
        let sphere_to_ray = ray.origin - self.origin;

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
                Some((distance, self.color, normal))
            } else {
                numerator = -b + discriminant.sqrt();

                if numerator > 0.0 {
                    let distance: Scalar = numerator / (2.0 * a);
                    let p = ray.at_distance(distance);
                    let normal = self.get_normal(p);
                    Some((distance, self.color, normal))
                } else {
                    None
                }
            }
        }
    }
}

impl Hittable for InfPlane {
    fn get_normal(&self, _p: Point) -> Normal {
        self.normal
    }

    fn intersect(&self, ray: &Ray) -> Option<(Scalar, Color, Normal)> {
        let denominator = ray.dir.dot(&self.normal);

        // Check if the ray is parallel with the plane with a threshold epsilon.
        if denominator.abs() < InfPlane::HIT_EPS {
            None
        } else {
            // Find the location of the intersection
            let lp = self.origin - ray.origin;
            let nominator = lp.dot(&self.normal);
            let distance: Scalar = nominator / denominator;

            if distance > 0.0 {
                Some((distance, self.color, self.normal))
            } else {
                None
            }
        }
    }
}
