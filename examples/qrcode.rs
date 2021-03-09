use qrcode_png::{Grayscale, QrCode, QrCodeEcc, Rgb, Rgba};

fn main() {
    let mut qrcode = QrCode::new(b"Hello Rust !", QrCodeEcc::Medium).unwrap();

    qrcode.zoom(10).margin(10);

    let buf = qrcode.generate(Grayscale::default()).unwrap();
    std::fs::write("./qrcode.grayscale.png", buf).unwrap();

    let buf = qrcode
        .generate(Rgb::new([3, 169, 244], [113, 140, 0]))
        .unwrap();
    std::fs::write("./qrcode.rgb.png", buf).unwrap();

    let buf = qrcode
        .generate(Rgba::new([137, 89, 168, 255], [255, 255, 255, 0]))
        .unwrap();
    std::fs::write("./qrcode.rgba.png", buf).unwrap();
}
