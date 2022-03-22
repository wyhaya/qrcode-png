use qrcode_png::{Color, QrCode, QrCodeEcc};

fn main() {
    let mut qrcode = QrCode::new(b"Hello Rust !", QrCodeEcc::Medium).unwrap();

    qrcode.zoom(10).margin(10);

    // -------- Bitmap
    let buf = qrcode.generate(Color::Bitmap(false, true)).unwrap();
    std::fs::write("./qrcode.bitmap.png", buf).unwrap();

    // -------- Grayscale
    let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
    std::fs::write("./qrcode.grayscale.png", buf).unwrap();

    // -------- RGB
    let buf = qrcode
        .generate(Color::Rgb([3, 169, 244], [113, 140, 0]))
        .unwrap();
    std::fs::write("./qrcode.rgb.png", buf).unwrap();

    // -------- RGBA
    let buf = qrcode
        .generate(Color::Rgba([137, 89, 168, 255], [255, 255, 255, 0]))
        .unwrap();
    std::fs::write("./qrcode.rgba.png", buf).unwrap();
}
