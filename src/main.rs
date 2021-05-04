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

    fn save_as_png(&self, file_path: &str) {
        let path = Path::new(file_path);

        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(self.data).unwrap();
    }
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
    im.save_as_png(r"output/image.png");

    // match im.save_as_png(r"output/image2.png") {
    //     Ok(file_path) => println!("Successfully saved to {}", file_path),
    //     Err(error) => panic!("Error: {:?}", error),
    // };
}
