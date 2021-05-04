extern crate nalgebra as na;
mod color;
mod image;
use color::{Color, ColorData};
use image::Image;
use na::{Unit, Vector3};


struct Camera {
    width: u16,
    height: u16,
    fov: u16,        // field of view in degrees
    aspect_ratio: f32
}

impl Camera {
    fn new(width: u16, height: u16, fov: u16) -> Camera {
        Camera {
            width, 
            height,
            fov,
            aspect_ratio: width as f32 / height as f32,
        }
    }

    fn get_lens_upper_left(&self) -> Vector3<f32> {
        let lens_half_height: f32 = ((self.fov / 2) as f32).to_radians().tan();
        let lens_half_width: f32 = lens_half_height * self.aspect_ratio;
        let x: f32 = -lens_half_width;
        let y: f32 = lens_half_height;
        let z: f32 = -1.0;
        Vector3::<f32>::new(x, y, z)
    }
}



fn trace_image(file_path: &str, trace_fn: &dyn Fn(u32, u32, u32, u32) -> Color) {
    let width = 960;
    let height = 540;

    let mut color_data = ColorData::new(vec![]);

    for j in 0..height {
        for i in 0..width {
            let c: Color = trace_fn(i, j, width, height);
            color_data.push(c);
        }
    }

    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(file_path).unwrap();
}

fn test_rainbow(i: u32, j: u32, width: u32, height: u32) -> Color {
    Color::from_floats(i as f32 / width as f32, j as f32 / height as f32, 0.9)
}

struct Ray {
    origin: Vector3<f32>,
    dir: Unit<Vector3<f32>>,
}

impl Ray {
    fn default() -> Ray {
        let default_dir = Vector3::<f32>::new(0.0, 0.0, -1.0);
        Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            dir: Unit::try_new(default_dir, 1e-10).unwrap(),
        }
    }

    fn at_distance(&self, d: f32) -> Vector3<f32> {
        self.origin + d * self.dir.as_ref()
    }
}

fn ray_trace(_i: u32, _j: u32, _width: u32, _height: u32) -> Color {
    let r = 0.5;
    let g = 0.5;
    let b = 0.5;
    Color::from_floats(r, g, b)
}

fn main() {
    println!("Make a test png file");
    trace_image(r"output/rainbow.png", &test_rainbow);

    println!("Making test ray traced image");
    trace_image(r"output/traced.png", &ray_trace);

    let r = Ray::default();
    let k = r.at_distance(3.0);

    println! {"vector at distance : {:?}", k};


    let c = Camera::new(1920, 1080, 45);
    let loc_upper_left = c.get_lens_upper_left();
    println! {"upper left location : {:?}", loc_upper_left};

}
