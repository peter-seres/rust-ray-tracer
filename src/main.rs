mod camera;
mod color;
mod consts;
mod image;
mod ray;
mod types;
mod hittables;

pub use camera::Camera;
pub use color::{Color, ColorData};
pub use consts::*;
pub use image::Image;
pub use ray::Ray;
pub use types::*;
pub use hittables::{Hittable, Sphere, InfPlane};


// Pre-define a few colors.
const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const BLACK: Color = Color::new(0.0, 0.0, 0.0);
const GREY: Color = Color::new(0.5, 0.5, 0.5);
const SKYBLUE: Color = Color::new(0.5, 0.7, 1.0);
const RED: Color = Color::new(1.0, 0.1, 0.1);
const BLUE: Color = Color::new(0.1, 0.1, 1.0);
const GREEN: Color = Color::new(0.1, 1.0, 0.1);


// Shader settings:
const LAMBERT_INT: Scalar = 0.99;
const AMBIENT_INT: Scalar = 0.2;


#[derive(Clone, Copy)]
struct Light {
    origin: Point,
    strength: Scalar,
}

impl Light {
    fn new(origin: Point, strength: Scalar) -> Self {
        Self{origin, strength}
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



fn raycast<const N: usize, const M: usize>(ray: Ray, hittables: &[&dyn Hittable; N], lights: &[&Light; M]) -> Color {

    // Find the closest hit for a raycast.
    return match get_closest_intersection(ray, hittables) {
        None => return BLACK,
        Some((mut point, base_color, normal)) => {

            // Shift point P along the normal vector to avoid shadow acne:
            const BIAS: Scalar = 2e-4;
            point = point + BIAS * normal;
            let mut color: Color = base_color * AMBIENT_INT;

            for light in lights {
                let vector_to_light: Vector3 = light.origin - point;

                // Calculate light intensity fall-off
                let distance: Scalar = vector_to_light.norm();
                let intensity: Scalar = light.strength / (distance * distance);

                let lambert_factor: Scalar = intensity * LAMBERT_INT * vector_to_light.dot(&normal);

                let ray_to_light = Ray::new(point, vector_to_light);
                match get_closest_intersection(ray_to_light, hittables) {
                    None => {
                        if lambert_factor > 0.0 {
                            color = color * (1.0 + lambert_factor);
                        }
                    },
                    Some(_) => continue,
                }
            }
            color
        }
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
    const N: usize = 3;                  // number of objects
    let s1 = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 0.5, RED);
    let s2 = Sphere::new(Vector3::new(1.3, 0.5, -7.0), 1.5, BLUE);
    let p = InfPlane::new(Vector3::new(0.0, -1.0, -5.0), Vector3::new(0.0, 1.0, 0.0), GREY);
    let objects: [&dyn Hittable; N] = [&s1, &s2, &p];

    // Make lights in the scene:
    const M: usize = 2;
    let l1 = Light::new(Vector3::new(-3.0, 2.0, -2.0), 10.0);
    let l2 = Light::new(Vector3::new(3.0, 2.0, -2.0), 2.0);
    let lights: [&Light; M] = [&l1, &l2];

    // Iterate through the Camera, do ray tracing and gather the color data
    for ray in c {
        let c: Color = raycast(ray, &objects, &lights);
        color_data.push(c);
    }

    // Save the color data to image
    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(file_path).unwrap();
}
