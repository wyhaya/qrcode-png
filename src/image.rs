use png::{BitDepth, ColorType, Encoder, EncodingError};

/// Define the color of the `QR code`
///
/// `PNG` image color type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Bitmap color false-true: one bit per pixel.
    Bitmap(bool, bool),
    /// Grayscale color 0-255
    Grayscale(u8, u8),
    /// RGB color [0-255, 0-255, 0-255]
    Rgb([u8; 3], [u8; 3]),
    /// RGBA color [0-255, 0-255, 0-255, 0-255]
    Rgba([u8; 4], [u8; 4]),
}

impl Default for Color {
    fn default() -> Self {
        Self::Bitmap(false, true)
    }
}

#[derive(Debug, Clone)]
pub struct Png {
    width: usize,
    height: usize,
    data: Vec<u8>,
    color: Color,
}

impl Png {
    // Create a png picture
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        // fill image background
        let data = match color {
            Color::Bitmap(_, c) => {
                let bytes_per_row = width / 8 + (width % 8 != 0) as usize;
                vec![if c { 0xff } else { 0x00 }; height * bytes_per_row]
            }
            Color::Grayscale(_, c) => vec![c; width * height],
            Color::Rgb(_, [r, g, b]) => vec![r, g, b].repeat(width * height),
            Color::Rgba(_, [r, g, b, a]) => vec![r, g, b, a].repeat(width * height),
        };

        Self {
            width,
            height,
            data,
            color,
        }
    }

    // Set QR code foreground color
    pub fn set_color(&mut self, x: usize, y: usize) {
        match &self.color {
            Color::Bitmap(c, _) => {
                let (x_byte, x_bit) = (x / 8, x % 8);
                let stride = self.width / 8 + (self.width % 8 != 0) as usize;
                let mask: u8 = 1 << (7 - x_bit);
                let byte = &mut self.data[y * stride + x_byte];
                if *c {
                    *byte |= mask;
                } else {
                    *byte &= !mask;
                }
            }
            Color::Grayscale(c, _) => {
                let index = y * self.width + x;
                self.data[index] = *c;
            }
            Color::Rgb([r, g, b], _) => {
                let index = (y * self.width + x) * 3;
                self.data[index] = *r;
                self.data[index + 1] = *g;
                self.data[index + 2] = *b;
            }
            Color::Rgba([r, g, b, a], _) => {
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

            match &self.color {
                Color::Bitmap(..) => {
                    encoder.set_color(ColorType::Grayscale);
                    encoder.set_depth(BitDepth::One)
                }
                Color::Grayscale(..) => encoder.set_color(ColorType::Grayscale),
                Color::Rgb(..) => encoder.set_color(ColorType::Rgb),
                Color::Rgba(..) => encoder.set_color(ColorType::Rgba),
            };

            let mut writer = encoder.write_header()?;
            writer.write_image_data(&self.data)?;
        }

        Ok(data)
    }
}
