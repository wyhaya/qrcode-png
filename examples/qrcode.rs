use qrcode_png::QrCode;

fn main() {
    let qrcode = QrCode::new(b"Hello Rust !", 10, 6).unwrap();
    let buf = qrcode.encode().unwrap();
    std::fs::write("./qrcode.png", buf).unwrap();
}
