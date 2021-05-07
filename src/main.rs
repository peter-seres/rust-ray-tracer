mod camera;
mod color;
mod consts;
mod image;
mod ray;
mod types;

pub use camera::Camera;
pub use color::{Color, ColorData};
pub use consts::*;
pub use image::Image;
pub use ray::Ray;
pub use types::*;

// Pre-define a few colors.
const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const BLACK: Color = Color::new(0.0, 0.0, 0.0);
const GREY: Color = Color::new(0.5, 0.5, 0.5);
const SKYBLUE: Color = Color::new(0.5, 0.7, 1.0);
const RED: Color = Color::new(1.0, 0.1, 0.1);
const BLUE: Color = Color::new(0.1, 0.1, 1.0);


trait Hittable {
    fn get_normal(&self, p: Point) -> Normal;
    fn intersect(&self, ray: &Ray) -> Option<(Scalar, Color, Normal)>;
}

struct Sphere {
    origin: Vector3,
    radius: Scalar,
    color: Color
}

impl Sphere {
    fn new(origin: Vector3, radius: Scalar, color: Color) -> Self {
        Self{origin, radius, color}
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
                }
                else {
                    None
                }
            }
        }
    }
}


fn get_closest_intersection<const N: usize>(ray: Ray, hittables: &[&dyn Hittable; N]) -> Option<(Point, Color, Normal)> {
    let mut hit: (Scalar, Option<(Color, Normal)>) = (1e3, None);

    for h in hittables{
        match h.intersect(&ray) {
            Some((distance, color, normal)) => {
                if distance < hit.0 && distance > 0.0 {
                    hit.0 = distance;
                    hit.1 = Some((color, normal));
                }
            },
            None => {},
        }
    }

    return match hit.1 {
        None => None,
        Some((color, normal)) => Some((ray.at_distance(hit.0), color, normal)),
    }
}


fn raycast<const N: usize>(ray: Ray, hittables: &[&dyn Hittable; N]) -> Color {
    // let sky_scaler = ray.dir[1];
    // let background: Color = (1.0 - sky_scaler) * WHITE + sky_scaler * SKYBLUE;

    match get_closest_intersection(ray, hittables) {
        None => BLACK,
        Some((_point, color, _normal)) => color
    }
}


fn main() {
    // Set image resolution and ouput path:
    let width = 860;
    let height = 640;
    let file_path = r"output/traced.png";

    // Camera setup:
    let c = Camera::new(width, height, 45);

    // Data allocation into Vector:
    let mut color_data = ColorData::new(vec![]);

    // Make objects in the scene:
    const N: usize = 2;                  // number of objects
    let s1 = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 0.5, RED);
    let s2 = Sphere::new(Vector3::new(1.0, 0.5, -7.0), 0.5, BLUE);
    let objects: [&dyn Hittable; N] = [&s1, &s2];

    // Iterate through the Camera, do ray tracing and gather the color data
    for ray in c {
        let c: Color = raycast(ray, &objects);
        color_data.push(c);
    }

    // Save the color data to image
    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(file_path).unwrap();
}
