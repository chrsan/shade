use structopt::StructOpt;

fn parse_color_value(value: &str) -> Result<shade::core::Color, String> {
    let result = if value.starts_with("0x") {
        u32::from_str_radix(&value[2..], 16)
    } else {
        value.parse()
    };
    match result {
        Ok(v) => Ok(v.into()),
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
    color: shade::core::Color,
}

fn main() {
    let opt = Opt::from_args();
    let svg = std::fs::read_to_string(&opt.input).unwrap();

    let mut dom = shade::svg::Dom::new(&svg).unwrap();
    dom.set_container_size(opt.width as _, opt.height as _);

    let mut pixels =
        vec![0u8; opt.height as usize * shade::core::Surface::row_bytes(opt.width as _)];
    let mut surface = shade::core::Surface::new(
        opt.width,
        opt.height,
        &mut pixels,
        None,
        shade::core::AlphaType::Premultiplied,
    )
    .unwrap();
    surface.clear(opt.color);

    dom.render(&mut surface);

    let file = std::fs::File::create(&opt.output).unwrap();
    let buffer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(buffer, opt.width, opt.height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(surface.pixels()).unwrap();
}
