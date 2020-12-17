#[cxx::bridge]
mod ffi {
    // See https://github.com/dtolnay/cxx/issues/379 for using cxx_name on enum variants.

    #[repr(i32)]
    enum SkAlphaType {
        kUnknown_SkAlphaType,
        kOpaque_SkAlphaType,
        kPremul_SkAlphaType,
        kUnpremul_SkAlphaType,
    }

    #[repr(i32)]
    enum SkColorType {
        kUnknown_SkColorType,
        kAlpha_8_SkColorType,
        kRGB_565_SkColorType,
        kARGB_4444_SkColorType,
        kRGBA_8888_SkColorType,
        kRGB_888x_SkColorType,
        kBGRA_8888_SkColorType,
        kRGBA_1010102_SkColorType,
        kBGRA_1010102_SkColorType,
        kRGB_101010x_SkColorType,
        kBGR_101010x_SkColorType,
        kGray_8_SkColorType,
        kRGBA_F16Norm_SkColorType,
        kRGBA_F16_SkColorType,
        kRGBA_F32_SkColorType,
        kR8G8_unorm_SkColorType,
        kA16_float_SkColorType,
        kR16G16_float_SkColorType,
        kA16_unorm_SkColorType,
        kR16G16_unorm_SkColorType,
        kR16G16B16A16_unorm_SkColorType,
    }

    #[repr(i32)]
    enum ScaleToFit {
        kFill_ScaleToFit,
        kStart_ScaleToFit,
        kCenter_ScaleToFit,
        kEnd_ScaleToFit,
    }

    #[repr(i32)]
    enum SkBlendMode {
        kClear = 0,
        kSrc = 1,
        kDst = 2,
        kSrcOver = 3,
        kDstOver = 4,
        kSrcIn = 5,
        kDstIn = 6,
        kSrcOut = 7,
        kDstOut = 8,
        kSrcATop = 9,
        kDstATop = 10,
        kXor = 11,
        kPlus = 12,
        kModulate = 13,
        kScreen = 14,
        kLastCoeffMode = 14,
        kOverlay = 15,
        kDarken = 16,
        kLighten = 17,
        kColorDodge = 18,
        kColorBurn = 19,
        kHardLight = 20,
        kSoftLight = 21,
        kDifference = 22,
        kExclusion = 23,
        kMultiply = 24,
        kLastSeparableMode = 24,
        kHue = 25,
        kSaturation = 26,
        kColor = 27,
        kLuminosity = 28,
        kLastMode = 28,
    }

    #[repr(u8)]
    enum PaintStyle {
        kFill_Style,
        kStroke_Style,
        kStrokeAndFill_Style,
    }

    #[repr(i32)]
    enum PaintStrokeCap {
        kButt_Cap,
        kRound_Cap,
        kSquare_Cap,
    }

    #[repr(u8)]
    enum PaintStrokeJoin {
        kMiter_Join,
        kRound_Join,
        kBevel_Join,
    }

    #[repr(i32)]
    enum PathFillType {
        kWinding,
        kEvenOdd,
        kInverseWinding,
        kInverseEvenOdd,
    }

    #[repr(i32)]
    enum PathDirection {
        kCW,
        kCCW,
    }

    unsafe extern "C++" {
        include!("shade/cc/core.h");

        type SkAlphaType;
        type SkColorType;

        fn n32_color_type() -> SkColorType;

        fn n32_color_type_is_bgra() -> bool;

        #[cxx_name = "SkColorTypeBytesPerPixel"]
        fn color_type_bytes_per_pixel(ct: SkColorType) -> i32;

        type SkPoint = super::SkPoint;
        type SkRect = super::SkRect;

        type SkMatrix = super::SkMatrix;
        type ScaleToFit;

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
            stf: ScaleToFit,
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
