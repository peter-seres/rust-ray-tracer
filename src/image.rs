use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Image<'a> {
    width: u32,
    height: u32,
    data: &'a Vec<u8>,
}

impl<'a> Image<'a> {
    pub fn new(width: u32, height: u32, data: &'a Vec<u8>) -> Image {
        Image {
            width,
            height,
            data,
        }
    }

    pub fn save_as_png(&self, file_path: &str) -> Result<(), &str> {
        let path = Path::new(file_path);
        let file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                panic!(
                    "Error while opening file at path: {:?}, error: {:?}",
                    path, e
                );
            }
        };

        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder
            .write_header()
            .expect("Header wasn't written correctly.");
        writer
            .write_image_data(self.data)
            .expect("Image data incorrect");
        Ok(())
    }
}

pub fn save_image_to_png(file_path: &str, width: u32, height: u32, data: &Vec<u8>) {
    // Make a path from the string literal
    let path = Path::new(file_path);

    // Open file and writer
    let file = File::create(path).unwrap();
    let writer = &mut BufWriter::new(file);

    // PNG encoder
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    // Write the file
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}
