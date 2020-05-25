
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
use qrcode_png::QrCode;

fn main() {
    let qrcode = QrCode::new(b"Hello Rust !", 10, 6).unwrap();
    let buf = qrcode.encode().unwrap();
    std::fs::write("./qrcode.png", buf).unwrap();
}
```

