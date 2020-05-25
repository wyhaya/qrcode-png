use png::{ColorType, Encoder, EncodingError};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

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

pub struct PNG {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl PNG {
    // Create a png picture
    pub fn new(width: usize, height: usize) -> Self {
        // The default is white
        let data = vec![255; width * height];

        Self {
            width,
            height,
            data,
        }
    }

    // Set the specified position to black
    pub fn set_black(&mut self, x: usize, y: usize) {
        let index = y * self.width + x;
        self.data[index] = 0;
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
        encoder.set_color(ColorType::Grayscale);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;

        let buf = (*data).borrow().to_vec();

        Ok(buf)
    }
}
