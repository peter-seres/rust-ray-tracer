use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

extern crate png;
extern crate nalgebra;

struct Image<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> Image<'a> {
    fn new(width: u32, height: u32, data: &'a [u8]) -> Image {
        Image {
            width,
            height,
            data,
        }
    }

    fn save_as_png(&self, file_path: &str) -> Result<(), &str> {

        let path = Path::new(file_path);
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(self.data).unwrap();
        Ok(())
    }
}


struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color {r, g, b}
    }

    fn from_floats(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: Color::float_to_u8(r), 
            g: Color::float_to_u8(g),
            b: Color::float_to_u8(b),
        }
    }

    fn float_to_u8(x: f32) -> u8 {
        (255.99 * x) as u8
    }
}

struct ColorData {
    data: Vec<u8>
}

impl ColorData {
    fn new(v: Vec<u8>) -> ColorData {
        ColorData {data: v}
    }

    fn push(&mut self, c: Color) {
        self.data.push(c.r);
        self.data.push(c.g);
        self.data.push(c.b);
    }

    fn into_vec(self) -> Vec<u8> {
        self.data
    }
}

fn test_rainbow() {
    let width = 1920;
    let height = 1080;

    let mut color_data = ColorData::new(vec![]);

    for j in 0..height {
        for i in 0..width {

            let c = Color::from_floats(i as f32 / width as f32, j as f32 / height as f32, 0.3);
            color_data.push(c);
        }
    }

    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(r"output/test.png").unwrap();
}


fn main() {
    println!("Making a .png file");

    let v = nalgebra::Vector3::new(1, 2, 3);

    println!("I made a vector: {:?}", v);

    test_rainbow();
}
