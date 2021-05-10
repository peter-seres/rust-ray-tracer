mod camera;
mod color;
mod consts;
mod hittables;
mod image;
mod lights;
mod ray;
mod types;
mod material;

pub use camera::Camera;
pub use color::{Color, ColorData};
pub use consts::*;
pub use hittables::{ObjectList, Hittable, InfPlane, Sphere};
pub use image::Image;
pub use lights::{LightList, Light, PointLight};
pub use ray::Ray;
pub use types::*;
pub use logger::{Logger, LogLevel};
pub use time::{Instant};

const ORIGIN: Point = Point::new(0.0, 0.0, 0.0);
const UP: Vector3 = Vector3::new(0.0, 1.0, 0.0);
const DOWN: Vector3 = Vector3::new(0.0, -1.0, 0.0);
const RIGHT: Vector3 = Vector3::new(1.0, 0.0, 0.0);
const LEFT: Vector3 = Vector3::new(-1.0, 0.0, 0.0);
const FORWARD: Vector3 = Vector3::new(0.0, 0.0, -1.0);
const BACKWARD: Vector3 = Vector3::new(0.0, 0.0, 1.0);

// Shader settings:
const LAMBERT_INT: Scalar = 0.8;
const AMBIENT_INT: Scalar = 0.0;
const REFLECTION_INT: Scalar = 0.2;
const SHADOW_BIAS: Scalar = 1e-3;

struct Hit {
    point: Point,
    color: Color,
    normal: Normal,
    distance: Scalar
}

/// Return the location, color and normal vector of the first object that a ray hits.
fn raycast(ray: Ray, hittables: &ObjectList) -> Option<Hit> {
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
        Some((color, normal)) => Some( Hit {
            point: ray.at_distance(hit.0),
            color,
            normal,
            distance: hit.0
        }),
    };
}


fn trace(
    ray: Ray,
    hittables: &ObjectList,
    lights: &LightList,
) -> (Color, Option<Ray>) {

    // Find the closest hit for a raycast.
    return match raycast(ray, hittables) {
        None => return (BLACK, None),
        Some(mut hit) => {
            let mut output_color = BLACK;

            output_color = output_color + hit.color * AMBIENT_INT;

            // Shifting along the bias against shadow acne
            // hit.point = hit.point + SHADOW_BIAS * *hit.normal;

            // Find shadows
            for light in lights {

                let vector_to_light: Vector3 = light.get_origin() - hit.point;

                let unit_to_light = Unit::try_new(vector_to_light, NORM_EPS).unwrap();

                // Shifting along the bias against shadow acne
                hit.point = hit.point + SHADOW_BIAS * *unit_to_light;

                let vector_to_light: Vector3 = light.get_origin() - hit.point;
                let distance_to_light: Scalar = vector_to_light.norm();
                let ray_to_light = Ray::new(hit.point, vector_to_light);

                let intensity: Scalar = light.get_strength() / (distance_to_light * distance_to_light);
                let mut lambert_intensity: Scalar = intensity * LAMBERT_INT * vector_to_light.dot(&hit.normal);

                if lambert_intensity < 0.0 {
                    lambert_intensity = 0.0;
                }

                match raycast(ray_to_light, hittables) {

                    // The ray to light hits something -> check distance to skip or not
                    Some(hit_towards_light) => {
                        // The ray to light has no obstructions -> calculate intensity
                        if hit_towards_light.distance > distance_to_light {
                            output_color = output_color + hit.color * lambert_intensity;
                        }
                    },

                    None => {
                        // The ray to light has no obstructions -> calculate intensity
                        output_color = output_color + hit.color * lambert_intensity;
                    },
                }
            }

            // Return the resulting color and the reflected ray.
            let reflected_ray = ray.reflect(hit.point, hit.normal);
            (output_color, Some(reflected_ray))
        }
    };
}

fn sample(
    ray: Ray,
    hittables: &ObjectList,
    lights: &LightList,
) -> Color {

    match trace(ray, hittables, lights) {
        (color, None) => return color,
        (mut color, Some(reflected_ray)) => {

            let refl_color = match trace(reflected_ray, hittables, lights) {
                (color, _) => color
            };
            color = color + refl_color * REFLECTION_INT;
            color
        }
    }
}


fn main() {

    // Logger:
    let logger = Logger::new(LogLevel::Info);
    logger.info("Setting up scene...");

    // Set image resolution and ouput path:
    let width = 4 * 1920;
    let height = 4 * 1080;
    let file_path = r"output/traced.png";

    // Camera setup:
    let c = Camera::new(width, height, 60);

    // Data allocation into Vector:
    let mut color_data = ColorData::new(vec![]);

    // Make objects in the scene:
    let p = InfPlane::new(Vector3::new(0.0, -1.0, 0.0), UP,LIGHTGRAY);
    let s1 = Sphere::new(Vector3::new(-1.0, 0.0, -5.0), 1.0, RED);
    let s2 = Sphere::new(Vector3::new(1.5, 0.5, -5.0), 1.5, BLUE);
    let s3 = Sphere::new(Vector3::new(-1.5, -0.5, -3.0), 0.5, GREEN);
    let s4 = Sphere::new(Vector3::new(0.0, -0.82, -2.5), 0.2, TEAL);
    let s5 = Sphere::new(Vector3::new(0.8, -0.6, -3.0), 0.4, PINK);

    // Object list of heap pointers:
    let mut objects: Vec<Box<dyn Hittable>> = vec![];
    objects.push(Box::new(p));
    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));
    objects.push(Box::new(s4));
    objects.push(Box::new(s5));

    // Make lights in the scene:
    let mut lights: Vec<Box<dyn Light>> = vec![];
    let l1 = PointLight::new(Vector3::new(-2.3, 2.3, -3.0), 10.0);
    let l2 = PointLight::new(Vector3::new(3.0, 6.0, -2.0), 10.0);
    let l3 = PointLight::new(Vector3::new(0.0, -0.7, -3.0), 1.0);

    // lights.push(Box::new(l1));
    // lights.push(Box::new(l2));
    lights.push(Box::new(l3));

    // Iterate through the Camera, do ray tracing and gather the color data
    logger.info("Starting iterations.");

    let start = Instant::now();
    for ray in c {
        let c: Color = sample(ray, &objects, &lights);
        color_data.push(c);
    }
    let end = Instant::now();

    let duration = end - start;

    logger.info("Ray Tracing finished.");
    println!("Computation time {:?}", duration);

    // Save the color data to image
    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(file_path).unwrap();
}
