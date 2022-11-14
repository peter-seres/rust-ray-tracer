use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Image<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> Image<'a> {
    pub fn new(width: u32, height: u32, data: &'a [u8]) -> Image {
        Image {
            width,
            height,
            data,
        }
    }

    pub fn save_as_png(&self, file_path: &str) -> Result<(), &str> {
        let path = Path::new(file_path);
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(self.data).unwrap();
        Ok(())
    }
}
