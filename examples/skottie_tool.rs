use std::str::FromStr;
use structopt::StructOpt;

fn parse_timeline_value(value: &str) -> Result<f64, String> {
    match f64::from_str(value) {
        Ok(v) => {
            if v >= 0.0 && v <= 1.0 {
                Ok(v)
            } else {
                Err(value.to_owned())
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "skottie_tool")]
struct Opt {
    /// Input JSON file.
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: std::path::PathBuf,
    /// Output directory.
    #[structopt(short = "o", long, parse(from_os_str))]
    output: std::path::PathBuf,
    /// Timeline start [0..1].
    #[structopt(default_value = "0.0", short = "f", long, parse(try_from_str = parse_timeline_value))]
    timeline_start: f64,
    /// Timeline stop [0..1].
    #[structopt(default_value = "1.0", short = "t", long, parse(try_from_str = parse_timeline_value))]
    timeline_stop: f64,
    /// FPS (default is animation native FPS).
    #[structopt(long)]
    fps: Option<f64>,
    /// Width
    #[structopt(default_value = "1280", short = "w", long)]
    width: u32,
    /// Height
    #[structopt(default_value = "720", short = "h", long)]
    height: u32,
}

fn main() {
    let opt = Opt::from_args();
    let json = std::fs::read_to_string(&opt.input).unwrap();

    let mut animation = shade::skottie::Animation::new(&json).unwrap();

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

    std::fs::create_dir_all(&opt.output).unwrap();

    let src_rect =
        shade::core::Rect::new(0.0, 0.0, animation.width() as _, animation.height() as _);
    let dst_rect = shade::core::Rect::new(0.0, 0.0, opt.width as _, opt.height as _);
    let scale_matrix =
        shade::core::Matrix::rect_to_rect(&src_rect, &dst_rect, shade::core::ScaleToFit::Center);
    surface.concat(&scale_matrix);

    let native_fps = animation.fps();
    let first_frame = animation.duration() * opt.timeline_start * native_fps;
    let duration = animation.duration() * (opt.timeline_stop - opt.timeline_start);
    let fps = opt.fps.unwrap_or(native_fps);
    if fps <= 0.0 {
        panic!("invalid FPS: {}", fps);
    }

    let fps_scale = native_fps / fps;
    let num_frames = (duration * fps) as usize;
    for i in 0..num_frames {
        let t = first_frame + (i as f64) * fps_scale;
        animation.seek_frame(t);
        surface.clear(0xffffffff.into());
        animation.render(&mut surface, None);
        let file =
            std::fs::File::create(std::path::Path::new(&opt.output).join(format!("{}.png", i)))
                .unwrap();
        let buffer = std::io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(buffer, opt.width, opt.height);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(surface.pixels()).unwrap();
    }
}
