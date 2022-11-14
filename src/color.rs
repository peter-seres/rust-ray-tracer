use crate::{Scalar, Vector3};

pub fn clip(x: Scalar) -> Scalar {
    x.max(0.0).min(1.0)
}

pub fn u8_to_scalar(x: u8) -> Scalar {
    x as Scalar / u8::MAX as Scalar
}

pub fn scalar_to_u8(x: Scalar) -> u8 {
    let y = clip(x);
    (255.99 * y) as u8
}

pub fn hex_byte_to_scalar(h: &str) -> Scalar {
    u8::from_str_radix(h, 16).unwrap() as Scalar / u8::MAX as Scalar
}

pub fn hex_to_rgb(hexcode: &str) -> Result<Vector3, &str> {
    if hexcode.starts_with("#") && (hexcode.len() == 7) {
        let r = hex_byte_to_scalar(&hexcode[1..3]);
        let g = hex_byte_to_scalar(&hexcode[3..5]);
        let b = hex_byte_to_scalar(&hexcode[5..7]);

        let color = Vector3::new(r, g, b);
        Ok(color)
    } else {
        Err("Hexcode must start with # and have 6 follow-up characters. Example: '#FFFFFF'.")
    }
}

pub fn push_color(buffer: &mut Vec<u8>, color: &Vector3) {
    buffer.push(scalar_to_u8(color.x));
    buffer.push(scalar_to_u8(color.y));
    buffer.push(scalar_to_u8(color.z));
}
