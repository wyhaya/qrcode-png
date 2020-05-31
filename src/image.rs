use png::{ColorType as PngColorType, Encoder, EncodingError};
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

/// PNG image color type
#[derive(Debug)]
pub enum ColorType {
    Grayscale(Grayscale),
    RGB(RGB),
    RGBA(RGBA),
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
    color: ColorType,
}

impl PNG {
    // Create a png picture
    pub fn new(width: usize, height: usize, color: ColorType) -> Self {
        let data = match &color {
            ColorType::Grayscale(color) => {
                let gray = color.background;
                vec![gray; width * height]
            }
            ColorType::RGB(color) => {
                let [r, g, b] = color.background;
                vec![r, g, b].repeat(width * height)
            }
            ColorType::RGBA(color) => {
                let [r, g, b, a] = color.background;
                vec![r, g, b, a].repeat(width * height)
            }
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
            ColorType::Grayscale(color) => {
                let index = y * self.width + x;
                self.data[index] = color.foreground;
            }
            ColorType::RGB(color) => {
                let index = (y * self.width + x) * 3;
                self.data[index] = color.foreground[0];
                self.data[index + 1] = color.foreground[1];
                self.data[index + 2] = color.foreground[2];
            }
            ColorType::RGBA(color) => {
                let index = (y * self.width + x) * 4;
                self.data[index] = color.foreground[0];
                self.data[index + 1] = color.foreground[1];
                self.data[index + 2] = color.foreground[2];
                self.data[index + 3] = color.foreground[3];
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
        match &self.color {
            ColorType::Grayscale(_) => encoder.set_color(PngColorType::Grayscale),
            ColorType::RGB(_) => encoder.set_color(PngColorType::RGB),
            ColorType::RGBA(_) => encoder.set_color(PngColorType::RGBA),
        };

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;

        let buf = (*data).borrow().to_vec();

        Ok(buf)
    }
}
