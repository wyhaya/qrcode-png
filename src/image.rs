use png::{ColorType, Encoder, EncodingError};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

// Container when encoding as PNG
struct RcWriter {
    data: Rc<RefCell<Vec<u8>>>,
}

impl RcWriter {
    fn new(data: Rc<RefCell<Vec<u8>>>) -> Self {
        Self { data }
    }
}

impl Write for RcWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// Define the color of the QR code
pub trait Color {
    fn get_foreground(&self) -> ColorValue;
    fn get_background(&self) -> ColorValue;
}

/// PNG image color type
#[derive(Debug)]
pub enum ColorValue {
    Grayscale(u8),
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}

/// Grayscale 0-255
#[derive(Debug)]
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
    fn get_foreground(&self) -> ColorValue {
        ColorValue::Grayscale(self.foreground)
    }

    fn get_background(&self) -> ColorValue {
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
#[derive(Debug)]
pub struct RGB {
    pub foreground: [u8; 3],
    pub background: [u8; 3],
}

impl Default for RGB {
    fn default() -> Self {
        Self {
            foreground: [0, 0, 0],
            background: [255, 255, 255],
        }
    }
}

impl Color for RGB {
    fn get_foreground(&self) -> ColorValue {
        let [r, g, b] = self.foreground;
        ColorValue::RGB(r, g, b)
    }

    fn get_background(&self) -> ColorValue {
        let [r, g, b] = self.background;
        ColorValue::RGB(r, g, b)
    }
}

impl RGB {
    pub fn new(foreground: [u8; 3], background: [u8; 3]) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

/// RGB color [0-255, 0-255, 0-255, 0-255]
#[derive(Debug)]
pub struct RGBA {
    pub foreground: [u8; 4],
    pub background: [u8; 4],
}

impl Default for RGBA {
    fn default() -> Self {
        Self {
            foreground: [0, 0, 0, 255],
            background: [255, 255, 255, 255],
        }
    }
}

impl Color for RGBA {
    fn get_foreground(&self) -> ColorValue {
        let [r, g, b, a] = self.foreground;
        ColorValue::RGBA(r, g, b, a)
    }

    fn get_background(&self) -> ColorValue {
        let [r, g, b, a] = self.background;
        ColorValue::RGBA(r, g, b, a)
    }
}

impl RGBA {
    pub fn new(foreground: [u8; 4], background: [u8; 4]) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

#[derive(Debug)]
pub struct PNG {
    width: usize,
    height: usize,
    data: Vec<u8>,
    foreground: ColorValue,
}

impl PNG {
    // Create a png picture
    pub fn new<C: Color>(width: usize, height: usize, color: C) -> Self {
        let data = match color.get_background() {
            ColorValue::Grayscale(c) => vec![c; width * height],
            ColorValue::RGB(r, g, b) => vec![r, g, b].repeat(width * height),
            ColorValue::RGBA(r, g, b, a) => vec![r, g, b, a].repeat(width * height),
        };

        Self {
            width,
            height,
            data,
            foreground: color.get_foreground(),
        }
    }

    // Set QR code foreground color
    pub fn set_color(&mut self, x: usize, y: usize) {
        match &self.foreground {
            ColorValue::Grayscale(c) => {
                let index = y * self.width + x;
                self.data[index] = *c;
            }
            ColorValue::RGB(r, g, b) => {
                let index = (y * self.width + x) * 3;
                self.data[index] = *r;
                self.data[index + 1] = *g;
                self.data[index + 2] = *b;
            }
            ColorValue::RGBA(r, g, b, a) => {
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
        let data = Rc::new(RefCell::new(Vec::new()));

        let mut encoder = Encoder::new(
            RcWriter::new(data.clone()),
            self.width as u32,
            self.height as u32,
        );

        // ..
        match &self.foreground {
            ColorValue::Grayscale(..) => encoder.set_color(ColorType::Grayscale),
            ColorValue::RGB(..) => encoder.set_color(ColorType::RGB),
            ColorValue::RGBA(..) => encoder.set_color(ColorType::RGBA),
        };

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;

        let buf = (*data).borrow().to_vec();

        Ok(buf)
    }
}
