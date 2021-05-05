extern crate nalgebra as na;
mod color;
mod image;
mod camera;
mod ray;
use color::{Color, ColorData};
use image::Image;
use na::{Unit, Vector3};
use camera::Camera;
use ray::Ray;


fn unit_vector(v: Vector3<f32>) -> Unit<Vector3<f32>> {
    Unit::try_new(v, 1e-10).unwrap()
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


fn ray_trace(_i: u32, _j: u32, _width: u32, _height: u32) -> Color {
    let r = 0.5;
    let g = 0.5;
    let b = 0.5;
    Color::from_floats(r, g, b)
}

struct Foo {
    w: u32,
    h: u32,
    iter_idx: u32,
}

impl Foo {
    fn new(w: u32, h: u32) -> Self {
        Self {
            w, h, iter_idx: 0
        }
    }
}

impl Iterator for Foo {
    type Item = Vector3<f32>;

    fn next(&mut self) -> Option<Vector3<f32>> {
        let i = self.iter_idx / self.h;
        let j = self.iter_idx % self.h;
        if i == self.w {
            None
        } else {
            self.iter_idx += 1;
            // return a Vector depending on i and j
            Some(Vector3::<f32>::new(i as f32, j as f32, 0.))
        }
    }
}

fn main() {
    // println!("Make a test png file");
    // trace_image(r"output/rainbow.png", &test_rainbow);

    // println!("Making test ray traced image");
    // trace_image(r"output/traced.png", &ray_trace);

    // let r = Ray::default();
    // let k = r.at_distance(3.0);

    // println! {"vector at distance : {:?}", k};
    let c = Camera::new(2, 3, 45);
    for ray in c {
        println!("{:?}", ray)
    }
}
