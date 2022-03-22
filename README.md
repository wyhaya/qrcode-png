
# qrcode-png
[![Crates.io](https://img.shields.io/crates/v/qrcode-png.svg?style=flat-square)](https://crates.io/crates/qrcode-png)
[![LICENSE](https://img.shields.io/crates/l/qrcode-png.svg?style=flat-square)](https://crates.io/crates/qrcode-png)
 
Generate QR code in png format
 
## Install

Add this in your `Cargo.toml`:

```toml
[dependencies]
qrcode-png = "*"
```

## Example
 
```rust
use qrcode_png::*;

fn main() {
    let mut qrcode = QrCode::new(b"Hello Rust !", QrCodeEcc::Medium).unwrap();

    qrcode.margin(10);
    qrcode.zoom(10);

    let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
    std::fs::write("./qrcode.png", buf).unwrap();
}
```

![qrcode](https://user-images.githubusercontent.com/23690145/83348739-c4f88d00-a361-11ea-932e-e722e0bd1b65.png)


