mod camera;
// mod color;
mod consts;
mod hittables;
mod image;
mod lights;
mod material;
mod ray;
mod types;
mod scene;

pub use camera::Camera;
// pub use color::{Color, ColorData};
pub use consts::*;
pub use hittables::{Hittable, InfPlane, Sphere};
pub use image::Image;
pub use lights::{Light, PointLight};
pub use ray::Ray;
pub use time::Instant;
pub use types::*;
pub use scene::{Scene, ObjectList, LightList};

// Shader settings:
const LAMBERT_INT: Scalar = 0.8;
const AMBIENT_INT: Scalar = 0.0;
const REFLECTION_INT: Scalar = 0.2;
const SHADOW_BIAS: Scalar = 2e-3;
const RECURSION_DEPTH: usize = 2;
const RAY_MAX_DIST: Scalar = 1e3;

struct Hit {
    point: Vector3,
    color: Vector3,
    normal: Unit<Vector3>,
    distance: Scalar,
}

fn raycast(ray: Ray, hittables: &ObjectList) -> Option<Hit> {
    let mut hit: (Scalar, Option<(Vector3, Normal)>) = (RAY_MAX_DIST, None);

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
        Some((color, normal)) => Some(Hit {
            point: ray.at_distance(hit.0),
            color,
            normal,
            distance: hit.0,
        }),
    };
}

fn trace(ray: Ray, hittables: &ObjectList, lights: &LightList, depth: usize) -> Color {

    // If we reached max depth -> return
    if depth <= 0 {
        return BLACK;
    }

    // Find the closest hit for a raycast.
    return match raycast(ray, hittables) {
        None => BLACK,
        Some(mut hit) => {
            let mut output_color = BLACK;

            output_color = output_color + hit.color * AMBIENT_INT;

            // Shifting along the bias against shadow acne
            hit.point = hit.point + SHADOW_BIAS * *hit.normal;

            // Find shadows
            for light in lights {
                let vector_to_light: Vector3 = light.get_origin() - hit.point;
                let distance_to_light: Scalar = vector_to_light.norm();
                let ray_to_light = Ray::new(hit.point, vector_to_light);

                let intensity: Scalar =
                    light.get_strength() / distance_to_light.powi(2);

                let mut lambert_intensity: Scalar =
                    intensity * LAMBERT_INT * vector_to_light.dot(&hit.normal);

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
                    }

                    None => {
                        // The ray to light has no obstructions -> calculate intensity
                        output_color = output_color + hit.color * lambert_intensity;
                    }
                }
            }

            // Do a reflection:
            let reflected_ray = ray.reflect(hit.point, hit.normal);
            let refl_color = trace(reflected_ray, hittables, lights, depth-1);
            output_color = output_color + refl_color * REFLECTION_INT;
            output_color
        }
    };
}

fn main() {
    // Set image resolution and ouput path:
    let width = 2 * 1920;
    let height = 2 * 1080;
    let file_path = r"output/traced.png";

    // Camera setup:
    let c = Camera::new(width, height, 60);

    // Data allocation into Vector:
    let mut color_data = ColorData::new(vec![]);

    // Default scene setup:
    let scene = Scene::default_scene(c);

    // Iterate through the Camera, do ray tracing and gather the color data
    println!("Starting iterations.");
    let start = Instant::now();
    for ray in scene.camera {
        let c: Color = trace(ray, &scene.objects, &scene.lights, RECURSION_DEPTH);
        color_data.push(c);
    }
    println!("Ray Tracing finished. Computation time {:?}", Instant::now() - start);

    // Save the color data to image
    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(file_path).unwrap();
}
