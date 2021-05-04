use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

extern crate png;

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

fn float_to_u8(x: f32) -> u8 {
    (255.99 * x) as u8
}

fn test_rainbow() {
    let width = 1920;
    let height = 1080;

    let mut data = vec![];

    for j in 0..height {
        for i in 0..width {
            let r: u8 = float_to_u8(i as f32 / width as f32);
            let g: u8 = float_to_u8(j as f32 / height as f32);
            let b: u8 = 80;

            data.push(r);
            data.push(g);
            data.push(b);
        }
    }

    let im = Image::new(width, height, &data);
    im.save_as_png(r"output/test.png").unwrap();
}


fn main() {
    println!("Making a .png file");

    let data = vec![
        255, 0, 0,      // Red
        0, 255, 0,      // Green
        0, 0, 255,      // Blue
        0, 0, 0,        // Black
        125, 125, 125,  // Grey
        255, 255, 255,  // White
    ];

    let im = Image::new(3, 2, &data);

    match im.save_as_png(r"output/image.png") {
        Ok(_output_path) => println!("Successfully saved image."),
        Err(e) => panic!("Problem writing to the file: {}", e),
    };

    test_rainbow();
}
