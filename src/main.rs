mod camera;
mod color;
mod consts;
mod hittables;
mod image;
mod lights;
mod ray;
mod types;

pub use camera::Camera;
pub use color::{Color, ColorData};
pub use consts::*;
pub use hittables::{Hittable, InfPlane, Sphere};
pub use image::Image;
pub use lights::{Light, PointLight};
pub use ray::Ray;
pub use types::*;

// Shader settings:
const LAMBERT_INT: Scalar = 0.99;
const AMBIENT_INT: Scalar = 0.2;
const REFLECTION_INT: Scalar = 0.5;

fn get_closest_intersection<const N: usize>(
    ray: Ray,
    hittables: &[&dyn Hittable; N],
) -> Option<(Point, Color, Normal)> {
    let mut hit: (Scalar, Option<(Color, Normal)>) = (1e3, None);

    for h in hittables {
        match h.intersect(&ray) {
            Some((distance, color, normal)) => {
                if distance < hit.0 && distance > 0.0 {
                    hit.0 = distance;
                    hit.1 = Some((color, normal));
                }
            }
            None => {}
        }
    }

    return match hit.1 {
        None => None,
        Some((color, normal)) => Some((ray.at_distance(hit.0), color, normal)),
    };
}


fn raycast<const N: usize, const M: usize>(
    ray: Ray,
    hittables: &[&dyn Hittable; N],
    lights: &[&dyn Light; M],
) -> (Color, Option<Ray>) {

    // Find the closest hit for a raycast.
    return match get_closest_intersection(ray, hittables) {
        None => return (BLACK, None),
        Some((mut point, base_color, normal)) => {

            // Shift point P along the normal vector to avoid shadow acne:
            const BIAS: Scalar = 2e-4;
            point = point + BIAS * normal;
            let mut color: Color = base_color * AMBIENT_INT;

            for light in lights {
                let vector_to_light = light.get_origin() - point;
                let ray = Ray::new(point, vector_to_light);
                match get_closest_intersection(ray, hittables) {
                    Some(_) => continue,
                    None => {
                        let intensity: Scalar = light.get_intensity(point, normal);
                        color = color * (1.0 + intensity);
                    }
                }
            }
            (color, Some(ray.reflect(point, normal)))
        }
    };
}

fn sample<const N: usize, const M: usize>(
    ray: Ray,
    hittables: &[&dyn Hittable; N],
    lights: &[&dyn Light; M],
) -> Color {

    match raycast(ray, hittables, lights) {
        (color, None) => return color,
        (mut color, Some(ray_reflected)) => {
            let refl_color = match raycast(ray_reflected, hittables, lights) {
                (c, None) => c,
                (c, Some(_)) => c,
            };
            color = color + refl_color * REFLECTION_INT;
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
    let c = Camera::new(width, height, 60);

    // Data allocation into Vector:
    let mut color_data = ColorData::new(vec![]);

    // Make objects in the scene:
    const N: usize = 3; // number of objects
    let s1 = Sphere::new(Vector3::new(-0.6, 0.0, -7.0), 0.5, RED);
    let s2 = Sphere::new(Vector3::new(1.3, 0.5, -5.0), 1.3, BLUE);
    let p = InfPlane::new(
        Vector3::new(0.0, -1.0, -5.0),
        Vector3::new(0.0, 1.0, 0.0),
        LIGHTGRAY,
    );
    let objects: [&dyn Hittable; N] = [&s1, &s2, &p];

    // Make lights in the scene:
    const M: usize = 2;
    let l1 = PointLight::new(Vector3::new(-3.0, 2.0, -2.0), 10.0);
    let l2 = PointLight::new(Vector3::new(3.0, 2.0, -2.0), 2.0);
    let lights: [&dyn Light; M] = [&l1, &l2];

    // Iterate through the Camera, do ray tracing and gather the color data
    for ray in c {
        let c: Color = sample(ray, &objects, &lights);
        color_data.push(c);
    }

    // Save the color data to image
    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(file_path).unwrap();
}
