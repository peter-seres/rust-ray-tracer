use crate::Scalar;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: Scalar,
    g: Scalar,
    b: Scalar,
}

impl Color {
    pub const fn new(r: Scalar, g: Scalar, b: Scalar) -> Self {
        Color { r, g, b }
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
