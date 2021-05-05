pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color{r, g, b}
    }

    pub fn from_floats(r: f32, g: f32, b: f32) -> Color {
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

pub struct ColorData {
    data: Vec<u8>,
}

impl ColorData {
    pub fn new(v: Vec<u8>) -> ColorData {
        ColorData { data: v }
    }

    pub fn push(&mut self, c: Color) {
        self.data.push(c.r);
        self.data.push(c.g);
        self.data.push(c.b);
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }
}
