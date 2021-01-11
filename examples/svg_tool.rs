use structopt::StructOpt;

fn parse_color_value(value: &str) -> Result<u32, String> {
    let result = if value.starts_with("0x") {
        u32::from_str_radix(&value[2..], 16)
    } else {
        value.parse()
    };
    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}", e)),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "svg_tool")]
struct Opt {
    /// Input SVG file.
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: std::path::PathBuf,

    /// Output file.
    #[structopt(short = "o", long, parse(from_os_str))]
    output: std::path::PathBuf,

    /// Width
    #[structopt(default_value = "1280", short = "w", long)]
    width: u32,

    /// Height
    #[structopt(default_value = "720", short = "h", long)]
    height: u32,

    /// Background color.
    #[structopt(default_value = "0", short = "c", long, parse(try_from_str = parse_color_value))]
    color: u32,
}

fn main() {
    let opt = Opt::from_args();
    let svg = std::fs::read_to_string(&opt.input).unwrap();

    let mut dom = shade::svg::SvgDom::new(&svg);
    assert!(!dom.is_null());
    dom.pin_mut()
        .set_container_size(&shade::core::SkSize::new(opt.width as _, opt.height as _));

    let ct = shade::core::SkColorType::n32();
    let mut pixels =
        vec![0u8; opt.height as usize * opt.width as usize * ct.bytes_per_pixel() as usize];
    let mut canvas = shade::core::SkCanvas::new(
        opt.width,
        opt.height,
        ct,
        shade::core::SkAlphaType::Premul,
        &mut pixels,
        0,
    );
    assert!(!canvas.is_null());
    canvas.pin_mut().clear(opt.color);

    dom.render(canvas.pin_mut());

    let file = std::fs::File::create(&opt.output).unwrap();
    let buffer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(buffer, opt.width, opt.height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
}
