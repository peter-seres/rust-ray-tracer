use crate::Scalar;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: Scalar,
    g: Scalar,
    b: Scalar,
}

fn u8_to_scalar(x: u8) -> Scalar {
    x as Scalar / u8::MAX as Scalar
}

impl Color {
    pub const fn new(r: Scalar, g: Scalar, b: Scalar) -> Self {
        Color { r, g, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: u8_to_scalar(r),
            g: u8_to_scalar(g),
            b: u8_to_scalar(b),
        }
    }

    pub fn from_hex(hexcode: &str) -> Self {
        hex_to_rgb(hexcode).unwrap()
    }

    fn float_to_u8(x: Scalar) -> u8 {
        let y = Color::clip(x);
        (255.99 * y) as u8
    }

    fn clip(x: Scalar) -> Scalar {
        x.max(0.0).min(1.0)
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        let r = self.r + _rhs.r;
        let g = self.g + _rhs.g;
        let b = self.b + _rhs.b;

        Color::new(r, g, b)
    }
}

impl std::ops::Add<Scalar> for Color {
    type Output = Color;

    fn add(self, _rhs: Scalar) -> Color {
        let r = self.r + _rhs;
        let g = self.g + _rhs;
        let b = self.b + _rhs;

        Color::new(r, g, b)
    }
}

impl std::ops::Mul<Scalar> for Color {
    type Output = Color;

    fn mul(self, _rhs: Scalar) -> Color {
        let r = self.r * _rhs;
        let g = self.g * _rhs;
        let b = self.b * _rhs;

        Color::new(r, g, b)
    }
}

impl std::ops::Mul<Color> for Scalar {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        let r = self * _rhs.r;
        let g = self * _rhs.g;
        let b = self * _rhs.b;

        Color::new(r, g, b)
    }
}

impl std::ops::Add<Color> for Scalar {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        let r = self + _rhs.r;
        let g = self + _rhs.g;
        let b = self + _rhs.b;

        Color::new(r, g, b)
    }
}

pub struct ColorData {
    data: Vec<u8>,
}

impl ColorData {
    pub fn new(v: Vec<u8>) -> ColorData {
        ColorData { data: v }
    }

    pub fn push(&mut self, c: Color) {
        self.data.push(Color::float_to_u8(c.r));
        self.data.push(Color::float_to_u8(c.g));
        self.data.push(Color::float_to_u8(c.b));
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }
}

fn hex_byte_to_scalar(h: &str) -> Scalar {
    u8::from_str_radix(h, 16).unwrap() as Scalar / u8::MAX as Scalar
}

fn hex_to_rgb(hexcode: &str) -> Result<Color, &str> {
    if hexcode.starts_with("#") && (hexcode.len() == 7) {
        let r = hex_byte_to_scalar(&hexcode[1..3]);
        let g = hex_byte_to_scalar(&hexcode[3..5]);
        let b = hex_byte_to_scalar(&hexcode[5..7]);

        let color = Color::new(r, g, b);
        Ok(color)
    } else {
        Err("Hexcode must start with # and have 6 follow-up characters. Example: #FFFFFF")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clipping() {
        let mut c = Color::new(0.1, 1.1, 0.9);
        c.clip_colors();
        assert_eq!(c.g, 1.0);

        let mut c = Color::new(-0.1, 1.1, 0.9);
        c.clip_colors();
        assert_eq!(c.r, 0.0);
    }
}
