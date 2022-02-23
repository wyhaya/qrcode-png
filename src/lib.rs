//! Create a QR code
//! ```
//! use qrcode_png::{Bitmap, Grayscale, QrCode, QrCodeEcc, Rgb, Rgba};
//!
//! let mut qrcode = QrCode::new(b"Hello Rust !", QrCodeEcc::Medium).unwrap();
//!
//! qrcode.zoom(10).margin(10);
//!
//! // -------- Grayscale
//! let buf = qrcode.generate(Grayscale::default()).unwrap();
//! std::fs::write("./qrcode.grayscale.png", buf).unwrap();
//!
//! // -------- RGB
//! let buf = qrcode
//!     .generate(Rgb::new([3, 169, 244], [113, 140, 0]))
//!     .unwrap();
//! std::fs::write("./qrcode.rgb.png", buf).unwrap();
//!
//! // -------- RGBA
//! let buf = qrcode
//!     .generate(Rgba::new([137, 89, 168, 255], [255, 255, 255, 0]))
//!     .unwrap();
//! std::fs::write("./qrcode.rgba.png", buf).unwrap();
//! ```

mod image;

use image::PNG;
pub use image::{Bitmap, Color, Grayscale, Rgb, Rgba};
use png::EncodingError;
pub use qrcodegen::QrCodeEcc;
use qrcodegen::{DataTooLong, QrCode as QrCode_};

/// Define QR code
#[derive(Clone)]
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
    pub fn new<T: AsRef<[u8]>>(content: T, ecl: QrCodeEcc) -> Result<Self, DataTooLong> {
        let qr = QrCode_::encode_binary(content.as_ref(), ecl)?;

        Ok(Self {
            qr,
            zoom: 1,
            margin: 0,
        })
    }

    /// Enlarge the QR code according to the original scale,
    /// Default value: 1
    pub fn zoom(&mut self, zoom: u32) -> &mut Self {
        assert_ne!(zoom, 0, "The minimum value is 1");
        self.zoom = zoom;
        self
    }

    /// Set the distance between the QR code and the edge of the picture
    pub fn margin(&mut self, margin: u32) -> &mut Self {
        self.margin = margin;
        self
    }

    /// Get png data of QR code
    pub fn generate<C: Color>(&self, color: C) -> Result<Vec<u8>, EncodingError> {
        let size = self.qr.size() as u32 * self.zoom + self.margin * 2;

        let mut image = PNG::new(size as usize, size as usize, color);

        for x in 0..self.qr.size() {
            for y in 0..self.qr.size() {
                if self.qr.get_module(x, y) {
                    let x_start = (x as u32 * self.zoom) + self.margin;
                    let y_start = (y as u32 * self.zoom) + self.margin;

                    for x in x_start..x_start + self.zoom {
                        for y in y_start..y_start + self.zoom {
                            image.set_color(x as usize, y as usize);
                        }
                    }
                }
            }
        }

        image.encode()
    }
}
