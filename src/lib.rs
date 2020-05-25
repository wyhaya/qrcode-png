//! # Create a QR code
//!
//!     use qrcode_png::QrCode;
//!
//!     let qrcode = QrCode::new(b"Hello Rust !", 10, 6).unwrap();
//!     let buf = qrcode.encode().unwrap();
//!     std::fs::write("./qrcode.png", buf);
//!

mod image;

use image::PNG;
use png::EncodingError;
use qrcodegen::{DataTooLong, QrCode as QrCode_, QrCodeEcc};

pub struct QrCode {
    // QR Code
    qr: QrCode_,
    // Zoom factor
    zoom: u32,
    // Margin of the QR code from the picture
    margin: u32,
}

impl QrCode {
    /// Create a QR code
    pub fn new<T: AsRef<[u8]>>(content: T, zoom: u32, margin: u32) -> Result<Self, DataTooLong> {
        let qr = QrCode_::encode_binary(content.as_ref(), QrCodeEcc::Medium)?;

        Ok(Self { qr, zoom, margin })
    }

    /// Get png data of QR code
    pub fn encode(self) -> Result<Vec<u8>, EncodingError> {
        let size = self.qr.size() as u32 * self.zoom + self.margin * 2;
        let mut image = PNG::new(size as usize, size as usize);

        for x in 0..self.qr.size() {
            for y in 0..self.qr.size() {
                if self.qr.get_module(x, y) {
                    let x_start = (x as u32 * self.zoom) + self.margin;
                    let y_start = (y as u32 * self.zoom) + self.margin;

                    for x in x_start..x_start + self.zoom {
                        for y in y_start..y_start + self.zoom {
                            image.set_black(x as usize, y as usize);
                        }
                    }
                }
            }
        }

        image.encode()
    }
}
