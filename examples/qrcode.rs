use qrcode_png::{Grayscale, QrCode, QrCodeEcc, RGB, RGBA};

fn main() {
    let mut qrcode = QrCode::new(b"Hello Rust !", QrCodeEcc::Medium).unwrap();

    qrcode.margin(10);
    qrcode.zoom(10);

    let buf = qrcode.encode(Grayscale::default()).unwrap();
    std::fs::write("./qrcode.grayscale.png", buf).unwrap();

    let buf = qrcode
        .encode(RGB::new([3, 169, 244], [113, 140, 0]))
        .unwrap();
    std::fs::write("./qrcode.rgb.png", buf).unwrap();

    let buf = qrcode
        .encode(RGBA::new([137, 89, 168, 255], [255, 255, 255, 0]))
        .unwrap();
    std::fs::write("./qrcode.rgba.png", buf).unwrap();
}
