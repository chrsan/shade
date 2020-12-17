fn main() {
    let mut pixels = vec![0u8; 1280 * 720 * 4];
    let mut canvas = shade::core::Canvas::new_rgba_premultiplied(1280, 720, &mut pixels, 0);
    println!("canvas is null: {}", canvas.is_null());
    let mut paint = shade::core::Paint::new();
    paint.pin_mut().set_anti_alias(true);
    paint.pin_mut().set_color(0xffff0000);
    canvas
        .pin_mut()
        .clear(shade::core::Color::from_argb(255, 255, 255, 255).0);
    canvas.pin_mut().translate(100.0, 100.0);
    canvas
        .pin_mut()
        .draw_rect(&shade::core::Rect::from_wh(100.0, 100.0), &paint);
    let file = std::fs::File::create("sandbox.png").unwrap();
    let buffer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(buffer, 1280, 720);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
}
