use png::Encoder;

#[test]
fn create_bitmap() -> Result<(), Box<dyn std::error::Error>> {
    let width = 15;
    let height = 11;
    let mut buf = Vec::new();
    {
        let mut encoder = Encoder::new(&mut buf, width, height);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::One);
        let mut w = encoder.write_header()?;
        #[rustfmt::skip]
        let data = [
            // 1234567    89abcdef
            0b00000000, 0b11111100,
            0b00000000, 0b11111000,
            0b11111111, 0b11110000,
            0b11110001, 0b11111110,
            0b11110001, 0b11111110,
            0b11111111, 0b11111110,
            0b11000000, 0b11111110,
            0b11000000, 0b11111110,
            0b11000000, 0b11111110,
            0b11000000, 0b11111110,
            0b11000000, 0b11111110,
        ];
        w.write_image_data(&data)?;
    }
    std::fs::write("target/bitmap.png", &buf)?;
    Ok(())
}
