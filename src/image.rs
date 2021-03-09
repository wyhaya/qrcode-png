use png::{ColorType, Encoder, EncodingError};

// Define the color of the QR code
pub trait Color {
    fn foreground(&self) -> ColorValue;
    fn background(&self) -> ColorValue;
}

/// PNG image color type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorValue {
    Grayscale(u8),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
}

/// Grayscale color 0-255
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grayscale {
    pub foreground: u8,
    pub background: u8,
}

impl Default for Grayscale {
    fn default() -> Self {
        Self {
            foreground: 0,
            background: 255,
        }
    }
}

impl Color for Grayscale {
    fn foreground(&self) -> ColorValue {
        ColorValue::Grayscale(self.foreground)
    }

    fn background(&self) -> ColorValue {
        ColorValue::Grayscale(self.background)
    }
}

impl Grayscale {
    pub fn new(foreground: u8, background: u8) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

/// RGB color [0-255, 0-255, 0-255]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub foreground: [u8; 3],
    pub background: [u8; 3],
}

impl Default for Rgb {
    fn default() -> Self {
        Self {
            foreground: [0, 0, 0],
            background: [255, 255, 255],
        }
    }
}

impl Color for Rgb {
    fn foreground(&self) -> ColorValue {
        let [r, g, b] = self.foreground;
        ColorValue::Rgb(r, g, b)
    }

    fn background(&self) -> ColorValue {
        let [r, g, b] = self.background;
        ColorValue::Rgb(r, g, b)
    }
}

impl Rgb {
    pub fn new(foreground: [u8; 3], background: [u8; 3]) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

/// RGBA color [0-255, 0-255, 0-255, 0-255]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub foreground: [u8; 4],
    pub background: [u8; 4],
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            foreground: [0, 0, 0, 255],
            background: [255, 255, 255, 255],
        }
    }
}

impl Color for Rgba {
    fn foreground(&self) -> ColorValue {
        let [r, g, b, a] = self.foreground;
        ColorValue::Rgba(r, g, b, a)
    }

    fn background(&self) -> ColorValue {
        let [r, g, b, a] = self.background;
        ColorValue::Rgba(r, g, b, a)
    }
}

impl Rgba {
    pub fn new(foreground: [u8; 4], background: [u8; 4]) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PNG {
    width: usize,
    height: usize,
    data: Vec<u8>,
    foreground: ColorValue,
}

impl PNG {
    // Create a png picture
    pub fn new<C: Color>(width: usize, height: usize, color: C) -> Self {
        let data = match color.background() {
            ColorValue::Grayscale(c) => vec![c; width * height],
            ColorValue::Rgb(r, g, b) => vec![r, g, b].repeat(width * height),
            ColorValue::Rgba(r, g, b, a) => vec![r, g, b, a].repeat(width * height),
        };

        Self {
            width,
            height,
            data,
            foreground: color.foreground(),
        }
    }

    // Set QR code foreground color
    pub fn set_color(&mut self, x: usize, y: usize) {
        match &self.foreground {
            ColorValue::Grayscale(c) => {
                let index = y * self.width + x;
                self.data[index] = *c;
            }
            ColorValue::Rgb(r, g, b) => {
                let index = (y * self.width + x) * 3;
                self.data[index] = *r;
                self.data[index + 1] = *g;
                self.data[index + 2] = *b;
            }
            ColorValue::Rgba(r, g, b, a) => {
                let index = (y * self.width + x) * 4;
                self.data[index] = *r;
                self.data[index + 1] = *g;
                self.data[index + 2] = *b;
                self.data[index + 3] = *a;
            }
        }
    }

    // Encode pixel information as png
    pub fn encode(&self) -> Result<Vec<u8>, EncodingError> {
        let mut data = Vec::new();

        {
            let mut encoder = Encoder::new(&mut data, self.width as u32, self.height as u32);

            match &self.foreground {
                ColorValue::Grayscale(..) => encoder.set_color(ColorType::Grayscale),
                ColorValue::Rgb(..) => encoder.set_color(ColorType::RGB),
                ColorValue::Rgba(..) => encoder.set_color(ColorType::RGBA),
            };

            let mut writer = encoder.write_header()?;
            writer.write_image_data(&self.data)?;
        }

        Ok(data)
    }
}
