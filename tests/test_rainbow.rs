use crate::color::{Color, ColorData};
use crate::image::Image;

fn test_rainbow() {
    let width = 960;
    let height = 540;

    let mut color_data = ColorData::new(vec![]);

    for j in 0..height {
        for i in 0..width {
            let c: Color = Color::from_floats(i as f32 / width as f32, j as f32 / height as f32, 0.9);
            color_data.push(c);
        }
    }

    let data = color_data.into_vec();
    let im = Image::new(width, height, &data);
    im.save_as_png(r"output/test_rainbow.png").unwrap();
}
