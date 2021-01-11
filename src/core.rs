#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum SkAlphaType {
        #[cxx_name = "kUnknown_SkAlphaType"]
        Unknown,
        #[cxx_name = "kOpaque_SkAlphaType"]
        Opaque,
        #[cxx_name = "kPremul_SkAlphaType"]
        Premul,
        #[cxx_name = "kUnpremul_SkAlphaType"]
        Unpremul,
    }

    #[repr(i32)]
    enum SkColorType {
        #[cxx_name = "kUnknown_SkColorType"]
        Unknown,
        #[cxx_name = "kAlpha_8_SkColorType"]
        Alpha8,
        #[cxx_name = "kRGB_565_SkColorType"]
        Rgb565,
        #[cxx_name = "kARGB_4444_SkColorType"]
        Argb4444,
        #[cxx_name = "kRGBA_8888_SkColorType"]
        Rgba8888,
        #[cxx_name = "kRGB_888x_SkColorType"]
        Rgb888x,
        #[cxx_name = "kBGRA_8888_SkColorType"]
        Bgra8888,
        #[cxx_name = "kRGBA_1010102_SkColorType"]
        Rgba1010102,
        #[cxx_name = "kBGRA_1010102_SkColorType"]
        Bgra1010102,
        #[cxx_name = "kRGB_101010x_SkColorType"]
        Rgb101010x,
        #[cxx_name = "kBGR_101010x_SkColorType"]
        Bgr101010x,
        #[cxx_name = "kGray_8_SkColorType"]
        Gray8,
        #[cxx_name = "kRGBA_F16Norm_SkColorType"]
        RgbaF16Norm,
        #[cxx_name = "kRGBA_F16_SkColorType"]
        RgbaF16,
        #[cxx_name = "kRGBA_F32_SkColorType"]
        RgbaF32,
        #[cxx_name = "kR8G8_unorm_SkColorType"]
        R8g8Unorm,
        #[cxx_name = "kA16_float_SkColorType"]
        A16Float,
        #[cxx_name = "kR16G16_float_SkColorType"]
        R16g16Float,
        #[cxx_name = "kA16_unorm_SkColorType"]
        A16Unorm,
        #[cxx_name = "kR16G16_unorm_SkColorType"]
        R16g16Unowm,
        #[cxx_name = "kR16G16B16A16_unorm_SkColorType"]
        R16g16b16a16Unorm,
    }

    #[repr(i32)]
    enum MatrixScaleToFit {
        #[cxx_name = "kFill_ScaleToFit"]
        Fill,
        #[cxx_name = "kStart_ScaleToFit"]
        Start,
        #[cxx_name = "kCenter_ScaleToFit"]
        Center,
        #[cxx_name = "kEnd_ScaleToFit"]
        End,
    }

    #[repr(i32)]
    enum SkBlendMode {
        #[cxx_name = "kClear"]
        Clear = 0,
        #[cxx_name = "kSrc"]
        Src = 1,
        #[cxx_name = "kDst"]
        Dst = 2,
        #[cxx_name = "kSrcOver"]
        SrcOver = 3,
        #[cxx_name = "kDstOver"]
        DstOver = 4,
        #[cxx_name = "kSrcIn"]
        SrcIn = 5,
        #[cxx_name = "kDstIn"]
        DstIn = 6,
        #[cxx_name = "kSrcOut"]
        SrcOut = 7,
        #[cxx_name = "kDstOut"]
        DstOut = 8,
        #[cxx_name = "kSrcATop"]
        SrcATop = 9,
        #[cxx_name = "kDstATop"]
        DstATop = 10,
        #[cxx_name = "kXor"]
        Xor = 11,
        #[cxx_name = "kPlus"]
        Plus = 12,
        #[cxx_name = "kModulate"]
        Modulate = 13,
        #[cxx_name = "kScreen"]
        Screen = 14,
        #[cxx_name = "kLastCoeffMode"]
        LastCoeffMode = 14,
        #[cxx_name = "kOverlay"]
        Overlay = 15,
        #[cxx_name = "kDarken"]
        Darken = 16,
        #[cxx_name = "kLighten"]
        Lighten = 17,
        #[cxx_name = "kColorDodge"]
        ColorDodge = 18,
        #[cxx_name = "kColorBurn"]
        ColorBurn = 19,
        #[cxx_name = "kHardLight"]
        HardLight = 20,
        #[cxx_name = "kSoftLight"]
        SoftLight = 21,
        #[cxx_name = "kDifference"]
        Difference = 22,
        #[cxx_name = "kExclusion"]
        Exclustion = 23,
        #[cxx_name = "kMultiply"]
        Multiply = 24,
        #[cxx_name = "kLastSeparableMode"]
        LastSeparableMode = 24,
        #[cxx_name = "kHue"]
        Hue = 25,
        #[cxx_name = "kSaturation"]
        Saturation = 26,
        #[cxx_name = "kColor"]
        Color = 27,
        #[cxx_name = "kLuminosity"]
        Luminosity = 28,
        #[cxx_name = "kLastMode"]
        LastMode = 28,
    }

    #[repr(u8)]
    enum PaintStyle {
        #[cxx_name = "kFill_Style"]
        Fill,
        #[cxx_name = "kStroke_Style"]
        Stroke,
        #[cxx_name = "kStrokeAndFill_Style"]
        StrokeAndFill,
    }

    #[repr(i32)]
    enum PaintStrokeCap {
        #[cxx_name = "kButt_Cap"]
        Butt,
        #[cxx_name = "kRound_Cap"]
        Round,
        #[cxx_name = "kSquare_Cap"]
        Square,
    }

    #[repr(u8)]
    enum PaintStrokeJoin {
        #[cxx_name = "kMiter_Join"]
        Miter,
        #[cxx_name = "kRound_Join"]
        Round,
        #[cxx_name = "kBevel_Join"]
        Bevel,
    }

    #[repr(i32)]
    enum PathFillType {
        #[cxx_name = "kWinding"]
        Winding,
        #[cxx_name = "kEvenOdd"]
        EvenOdd,
        #[cxx_name = "kInverseWinding"]
        InverseWinding,
        #[cxx_name = "kInverseEvenOdd"]
        InverseEvenOdd,
    }

    #[repr(i32)]
    enum PathDirection {
        #[cxx_name = "kCW"]
        Cw,
        #[cxx_name = "kCCW"]
        Ccw,
    }

    unsafe extern "C++" {
        include!("shade/cc/core.h");

        type SkAlphaType;
        type SkColorType;

        fn n32_color_type() -> SkColorType;

        fn n32_color_type_is_bgra() -> bool;

        #[cxx_name = "SkColorTypeBytesPerPixel"]
        fn color_type_bytes_per_pixel(ct: SkColorType) -> i32;

        type SkRect = super::SkRect;

        type SkMatrix = super::SkMatrix;
        type MatrixScaleToFit;

        fn new_identity_matrix() -> SkMatrix;

        #[cxx_name = "getScaleX"]
        fn get_scale_x(self: &SkMatrix) -> f32;

        #[cxx_name = "getScaleY"]
        fn get_scale_y(self: &SkMatrix) -> f32;

        #[cxx_name = "getSkewX"]
        fn get_skew_x(self: &SkMatrix) -> f32;

        #[cxx_name = "getSkewY"]
        fn get_skew_y(self: &SkMatrix) -> f32;

        #[cxx_name = "getTranslateX"]
        fn get_translate_x(self: &SkMatrix) -> f32;

        #[cxx_name = "getTranslateY"]
        fn get_translate_y(self: &SkMatrix) -> f32;

        #[cxx_name = "setTranslate"]
        fn set_translate(self: &mut SkMatrix, dx: f32, dy: f32) -> &mut SkMatrix;

        #[cxx_name = "setScale"]
        fn set_scale(self: &mut SkMatrix, sx: f32, sy: f32) -> &mut SkMatrix;

        #[cxx_name = "setRotate"]
        fn set_rotate(self: &mut SkMatrix, degrees: f32) -> &mut SkMatrix;

        #[cxx_name = "setSkew"]
        fn set_skew(self: &mut SkMatrix, sx: f32, sy: f32) -> &mut SkMatrix;

        #[cxx_name = "setSinCos"]
        fn set_sin_cos(self: &mut SkMatrix, sin: f32, cos: f32) -> &mut SkMatrix;

        #[cxx_name = "setRectToRect"]
        fn set_rect_to_rect(
            self: &mut SkMatrix,
            src: &SkRect,
            dst: &SkRect,
            stf: MatrixScaleToFit,
        ) -> bool;

        fn invert_matrix(m: &SkMatrix, ok: &mut bool) -> SkMatrix;

        type SkCanvas;

        fn new_canvas(
            width: u32,
            height: u32,
            color_type: SkColorType,
            alpha_type: SkAlphaType,
            pixels: &mut Vec<u8>,
            row_bytes: usize,
        ) -> UniquePtr<SkCanvas>;

        fn clear(self: Pin<&mut SkCanvas>, color: u32);

        fn flush(self: Pin<&mut SkCanvas>);

        fn save(self: Pin<&mut SkCanvas>) -> i32;

        fn restore(self: Pin<&mut SkCanvas>);

        fn scale(self: Pin<&mut SkCanvas>, sx: f32, sy: f32);

        fn translate(self: Pin<&mut SkCanvas>, dx: f32, dy: f32);

        fn rotate(self: Pin<&mut SkCanvas>, degrees: f32);

        fn skew(self: Pin<&mut SkCanvas>, sx: f32, sy: f32);

        fn concat(self: Pin<&mut SkCanvas>, matrix: &SkMatrix);

        #[cxx_name = "resetMatrix"]
        fn reset_matrix(self: Pin<&mut SkCanvas>);

        #[cxx_name = "drawRect"]
        fn draw_rect(self: Pin<&mut SkCanvas>, rect: &SkRect, paint: &SkPaint);

        #[cxx_name = "drawPath"]
        fn draw_path(self: Pin<&mut SkCanvas>, path: &SkPath, paint: &SkPaint);

        type SkBlendMode;

        type SkPaint;
        type PaintStyle;
        type PaintStrokeCap;
        type PaintStrokeJoin;

        fn new_paint() -> UniquePtr<SkPaint>;

        fn reset(self: Pin<&mut SkPaint>);

        #[cxx_name = "isAntiAlias"]
        fn is_anti_alias(self: &SkPaint) -> bool;

        #[cxx_name = "setAntiAlias"]
        fn set_anti_alias(self: Pin<&mut SkPaint>, aa: bool);

        #[cxx_name = "getStyle"]
        fn get_style(self: &SkPaint) -> PaintStyle;

        #[cxx_name = "setStyle"]
        fn set_style(self: Pin<&mut SkPaint>, style: PaintStyle);

        #[cxx_name = "setStroke"]
        fn set_stroke(self: Pin<&mut SkPaint>, b: bool);

        #[cxx_name = "getColor"]
        fn get_color(self: &SkPaint) -> u32;

        #[cxx_name = "setColor"]
        fn set_color(self: Pin<&mut SkPaint>, color: u32);

        #[cxx_name = "getAlpha"]
        fn get_alpha(self: &SkPaint) -> u8;

        #[cxx_name = "setAlpha"]
        fn set_alpha(self: Pin<&mut SkPaint>, alpha: u32);

        #[cxx_name = "setARGB"]
        fn set_argb(self: Pin<&mut SkPaint>, a: u32, r: u32, g: u32, b: u32);

        #[cxx_name = "getStrokeWidth"]
        fn get_stroke_width(self: &SkPaint) -> f32;

        #[cxx_name = "setStrokeWidth"]
        fn set_stroke_width(self: Pin<&mut SkPaint>, width: f32);

        #[cxx_name = "getStrokeMiter"]
        fn get_stroke_miter(self: &SkPaint) -> f32;

        #[cxx_name = "setStrokeMiter"]
        fn set_stroke_miter(self: Pin<&mut SkPaint>, miter: f32);

        #[cxx_name = "getStrokeCap"]
        fn get_stroke_cap(self: &SkPaint) -> PaintStrokeCap;

        #[cxx_name = "setStrokeCap"]
        fn set_stroke_cap(self: Pin<&mut SkPaint>, cap: PaintStrokeCap);

        #[cxx_name = "getStrokeJoin"]
        fn get_stroke_join(self: &SkPaint) -> PaintStrokeJoin;

        #[cxx_name = "setStrokeJoin"]
        fn set_stroke_join(self: Pin<&mut SkPaint>, join: PaintStrokeJoin);

        #[cxx_name = "getBlendMode"]
        fn get_blend_mode(self: &SkPaint) -> SkBlendMode;

        #[cxx_name = "isSrcOver"]
        fn is_src_over(self: &SkPaint) -> bool;

        #[cxx_name = "setBlendMode"]
        fn set_blend_mode(self: Pin<&mut SkPaint>, mode: SkBlendMode);

        type SkPath;
        type PathFillType;
        type PathDirection;

        fn new_path() -> UniquePtr<SkPath>;

        #[cxx_name = "getFillType"]
        fn get_fill_type(self: &SkPath) -> PathFillType;

        #[cxx_name = "setFillType"]
        fn set_fill_type(self: Pin<&mut SkPath>, ft: PathFillType);

        fn reset(self: Pin<&mut SkPath>) -> Pin<&mut SkPath>;

        fn rewind(self: Pin<&mut SkPath>) -> Pin<&mut SkPath>;

        #[cxx_name = "isEmpty"]
        fn is_empty(self: &SkPath) -> bool;

        #[cxx_name = "moveTo"]
        fn move_to(self: Pin<&mut SkPath>, x: f32, y: f32) -> Pin<&mut SkPath>;

        #[cxx_name = "lineTo"]
        fn line_to(self: Pin<&mut SkPath>, x: f32, y: f32) -> Pin<&mut SkPath>;

        #[cxx_name = "quadTo"]
        fn quad_to(self: Pin<&mut SkPath>, x1: f32, y1: f32, x2: f32, y2: f32) -> Pin<&mut SkPath>;

        #[cxx_name = "conicTo"]
        fn conic_to(
            self: Pin<&mut SkPath>,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            weight: f32,
        ) -> Pin<&mut SkPath>;

        #[cxx_name = "cubicTo"]
        fn cubic_to(
            self: Pin<&mut SkPath>,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            x3: f32,
            y3: f32,
        ) -> Pin<&mut SkPath>;

        fn close(self: Pin<&mut SkPath>) -> Pin<&mut SkPath>;

        #[cxx_name = "addRect"]
        fn add_rect<'a, 'b>(
            self: Pin<&'a mut SkPath>,
            rect: &'b SkRect,
            dir: PathDirection,
        ) -> Pin<&'a mut SkPath>;

        #[cxx_name = "computeTightBounds"]
        fn compute_tight_bounds(self: &SkPath) -> SkRect;

        fn dump_path(p: &SkPath);
    }
}

#[allow(non_camel_case_types)]
pub use self::ffi::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Color(pub u32);

impl Color {
    #[inline]
    pub fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | ((b as u32) << 0))
    }

    #[inline]
    pub fn alpha(&self) -> u8 {
        ((self.0 >> 24) & 0xff) as u8
    }

    #[inline]
    pub fn red(&self) -> u8 {
        ((self.0 >> 16) & 0xff) as u8
    }

    #[inline]
    pub fn green(&self) -> u8 {
        ((self.0 >> 8) & 0xff) as u8
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        ((self.0 >> 0) & 0xff) as u8
    }
}

impl From<u32> for Color {
    fn from(c: u32) -> Self {
        Self(c)
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SkPoint {
    pub x: f32,
    pub y: f32,
}

impl SkPoint {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

unsafe impl cxx::ExternType for SkPoint {
    type Id = cxx::type_id!("SkPoint");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SkRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl SkRect {
    #[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn from_wh(width: f32, height: f32) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: width,
            bottom: height,
        }
    }

    #[inline]
    pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            left: x,
            top: y,
            right: x + w,
            bottom: y + h,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        !(self.left < self.right && self.top < self.bottom)
    }
}

unsafe impl cxx::ExternType for SkRect {
    type Id = cxx::type_id!("SkRect");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SkSize {
    pub width: f32,
    pub height: f32,
}

impl SkSize {
    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

unsafe impl cxx::ExternType for SkSize {
    type Id = cxx::type_id!("SkSize");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkMatrix {
    pub mat: [f32; 9],
    type_mask: i32,
}

impl SkMatrix {
    pub fn invert(&self) -> Option<Self> {
        let mut ok = false;
        let inv = invert_matrix(self, &mut ok);
        if ok {
            Some(inv)
        } else {
            None
        }
    }
}

impl Default for SkMatrix {
    fn default() -> Self {
        new_identity_matrix()
    }
}

unsafe impl cxx::ExternType for SkMatrix {
    type Id = cxx::type_id!("SkMatrix");
    type Kind = cxx::kind::Trivial;
}

impl SkColorType {
    pub fn n32() -> Self {
        n32_color_type()
    }

    pub fn bytes_per_pixel(self) -> i32 {
        color_type_bytes_per_pixel(self)
    }
}

impl SkCanvas {
    #[inline]
    pub fn new(
        width: u32,
        height: u32,
        color_type: SkColorType,
        alpha_type: SkAlphaType,
        pixels: &mut Vec<u8>,
        row_bytes: usize,
    ) -> cxx::UniquePtr<Self> {
        new_canvas(width, height, color_type, alpha_type, pixels, row_bytes)
    }
}

impl SkPaint {
    #[inline]
    pub fn new() -> cxx::UniquePtr<Self> {
        new_paint()
    }
}

impl SkPath {
    #[inline]
    pub fn new() -> cxx::UniquePtr<Self> {
        new_path()
    }

    pub fn dump(&self) {
        dump_path(self);
    }
}
