#[cxx::bridge]
mod ffi {
    // See https://github.com/dtolnay/cxx/issues/379 for using cxx_name on enum variants.

    #[repr(i32)]
    enum MatrixScaleToFit {
        kFill_ScaleToFit,
        kStart_ScaleToFit,
        kCenter_ScaleToFit,
        kEnd_ScaleToFit,
    }

    #[repr(i32)]
    enum BlendMode {
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

        fn n32_color_type_is_bgra() -> bool;

        type Point = super::Point;
        type Rect = super::Rect;

        type Matrix = super::Matrix;
        type MatrixScaleToFit;

        fn new_identity_matrix() -> Matrix;

        #[cxx_name = "getScaleX"]
        fn get_scale_x(self: &Matrix) -> f32;

        #[cxx_name = "getScaleY"]
        fn get_scale_y(self: &Matrix) -> f32;

        #[cxx_name = "getSkewX"]
        fn get_skew_x(self: &Matrix) -> f32;

        #[cxx_name = "getSkewY"]
        fn get_skew_y(self: &Matrix) -> f32;

        #[cxx_name = "getTranslateX"]
        fn get_translate_x(self: &Matrix) -> f32;

        #[cxx_name = "getTranslateY"]
        fn get_translate_y(self: &Matrix) -> f32;

        #[cxx_name = "setTranslate"]
        fn set_translate(self: &mut Matrix, dx: f32, dy: f32) -> &mut Matrix;

        #[cxx_name = "setScale"]
        fn set_scale(self: &mut Matrix, sx: f32, sy: f32) -> &mut Matrix;

        #[cxx_name = "setRotate"]
        fn set_rotate(self: &mut Matrix, degrees: f32) -> &mut Matrix;

        #[cxx_name = "setSkew"]
        fn set_skew(self: &mut Matrix, sx: f32, sy: f32) -> &mut Matrix;

        #[cxx_name = "setSinCos"]
        fn set_sin_cos(self: &mut Matrix, sin: f32, cos: f32) -> &mut Matrix;

        #[cxx_name = "setRectToRect"]
        fn set_rect_to_rect(
            self: &mut Matrix,
            src: &Rect,
            dst: &Rect,
            stf: MatrixScaleToFit,
        ) -> bool;

        fn invert_matrix(m: &Matrix, ok: &mut bool) -> Matrix;

        type Canvas;

        fn new_rgba_canvas(
            width: u32,
            height: u32,
            pixels: &mut Vec<u8>,
            row_bytes: usize,
            premultiplied: bool,
        ) -> UniquePtr<Canvas>;

        fn clear(self: Pin<&mut Canvas>, color: u32);

        fn flush(self: Pin<&mut Canvas>);

        fn save(self: Pin<&mut Canvas>) -> i32;

        fn restore(self: Pin<&mut Canvas>);

        fn scale(self: Pin<&mut Canvas>, sx: f32, sy: f32);

        fn translate(self: Pin<&mut Canvas>, dx: f32, dy: f32);

        fn rotate(self: Pin<&mut Canvas>, degrees: f32);

        fn skew(self: Pin<&mut Canvas>, sx: f32, sy: f32);

        fn concat(self: Pin<&mut Canvas>, matrix: &Matrix);

        #[cxx_name = "resetMatrix"]
        fn reset_matrix(self: Pin<&mut Canvas>);

        #[cxx_name = "drawRect"]
        fn draw_rect(self: Pin<&mut Canvas>, rect: &Rect, paint: &Paint);

        #[cxx_name = "drawPath"]
        fn draw_path(self: Pin<&mut Canvas>, path: &Path, paint: &Paint);

        type BlendMode;

        type Paint;
        type PaintStyle;
        type PaintStrokeCap;
        type PaintStrokeJoin;

        fn new_paint() -> UniquePtr<Paint>;

        fn reset(self: Pin<&mut Paint>);

        #[cxx_name = "isAntiAlias"]
        fn is_anti_alias(self: &Paint) -> bool;

        #[cxx_name = "setAntiAlias"]
        fn set_anti_alias(self: Pin<&mut Paint>, aa: bool);

        #[cxx_name = "getStyle"]
        fn get_style(self: &Paint) -> PaintStyle;

        #[cxx_name = "setStyle"]
        fn set_style(self: Pin<&mut Paint>, style: PaintStyle);

        #[cxx_name = "setStroke"]
        fn set_stroke(self: Pin<&mut Paint>, b: bool);

        #[cxx_name = "getColor"]
        fn get_color(self: &Paint) -> u32;

        #[cxx_name = "setColor"]
        fn set_color(self: Pin<&mut Paint>, color: u32);

        #[cxx_name = "getAlpha"]
        fn get_alpha(self: &Paint) -> u8;

        #[cxx_name = "setAlpha"]
        fn set_alpha(self: Pin<&mut Paint>, alpha: u32);

        #[cxx_name = "setARGB"]
        fn set_argb(self: Pin<&mut Paint>, a: u32, r: u32, g: u32, b: u32);

        #[cxx_name = "getStrokeWidth"]
        fn get_stroke_width(self: &Paint) -> f32;

        #[cxx_name = "setStrokeWidth"]
        fn set_stroke_width(self: Pin<&mut Paint>, width: f32);

        #[cxx_name = "getStrokeMiter"]
        fn get_stroke_miter(self: &Paint) -> f32;

        #[cxx_name = "setStrokeMiter"]
        fn set_stroke_miter(self: Pin<&mut Paint>, miter: f32);

        #[cxx_name = "getStrokeCap"]
        fn get_stroke_cap(self: &Paint) -> PaintStrokeCap;

        #[cxx_name = "setStrokeCap"]
        fn set_stroke_cap(self: Pin<&mut Paint>, cap: PaintStrokeCap);

        #[cxx_name = "getStrokeJoin"]
        fn get_stroke_join(self: &Paint) -> PaintStrokeJoin;

        #[cxx_name = "setStrokeJoin"]
        fn set_stroke_join(self: Pin<&mut Paint>, join: PaintStrokeJoin);

        #[cxx_name = "getBlendMode"]
        fn get_blend_mode(self: &Paint) -> BlendMode;

        #[cxx_name = "isSrcOver"]
        fn is_src_over(self: &Paint) -> bool;

        #[cxx_name = "setBlendMode"]
        fn set_blend_mode(self: Pin<&mut Paint>, mode: BlendMode);

        type Path;
        type PathFillType;
        type PathDirection;

        fn new_path() -> UniquePtr<Path>;

        #[cxx_name = "getFillType"]
        fn get_fill_type(self: &Path) -> PathFillType;

        #[cxx_name = "setFillType"]
        fn set_fill_type(self: Pin<&mut Path>, ft: PathFillType);

        fn reset(self: Pin<&mut Path>) -> Pin<&mut Path>;

        fn rewind(self: Pin<&mut Path>) -> Pin<&mut Path>;

        #[cxx_name = "isEmpty"]
        fn is_empty(self: &Path) -> bool;

        #[cxx_name = "moveTo"]
        fn move_to(self: Pin<&mut Path>, x: f32, y: f32) -> Pin<&mut Path>;

        #[cxx_name = "lineTo"]
        fn line_to(self: Pin<&mut Path>, x: f32, y: f32) -> Pin<&mut Path>;

        #[cxx_name = "quadTo"]
        fn quad_to(self: Pin<&mut Path>, x1: f32, y1: f32, x2: f32, y2: f32) -> Pin<&mut Path>;

        #[cxx_name = "conicTo"]
        fn conic_to(
            self: Pin<&mut Path>,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            weight: f32,
        ) -> Pin<&mut Path>;

        #[cxx_name = "cubicTo"]
        fn cubic_to(
            self: Pin<&mut Path>,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            x3: f32,
            y3: f32,
        ) -> Pin<&mut Path>;

        fn close(self: Pin<&mut Path>) -> Pin<&mut Path>;

        #[cxx_name = "addRect"]
        fn add_rect<'a, 'b>(
            self: Pin<&'a mut Path>,
            rect: &'b Rect,
            dir: PathDirection,
        ) -> Pin<&'a mut Path>;

        #[cxx_name = "computeTightBounds"]
        fn compute_tight_bounds(self: &Path) -> Rect;

        fn dump_path(p: &Path);
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
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

unsafe impl cxx::ExternType for Point {
    type Id = cxx::type_id!("Point");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
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

unsafe impl cxx::ExternType for Rect {
    type Id = cxx::type_id!("Rect");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub mat: [f32; 9],
    type_mask: i32,
}

impl Matrix {
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

impl Default for Matrix {
    fn default() -> Self {
        new_identity_matrix()
    }
}

unsafe impl cxx::ExternType for Matrix {
    type Id = cxx::type_id!("Matrix");
    type Kind = cxx::kind::Trivial;
}

impl Canvas {
    #[inline]
    pub fn new_rgba(
        width: u32,
        height: u32,
        pixels: &mut Vec<u8>,
        row_bytes: usize,
    ) -> cxx::UniquePtr<Self> {
        new_rgba_canvas(width, height, pixels, row_bytes, false)
    }

    #[inline]
    pub fn new_rgba_premultiplied(
        width: u32,
        height: u32,
        pixels: &mut Vec<u8>,
        row_bytes: usize,
    ) -> cxx::UniquePtr<Self> {
        new_rgba_canvas(width, height, pixels, row_bytes, true)
    }
}

impl Paint {
    #[inline]
    pub fn new() -> cxx::UniquePtr<Self> {
        new_paint()
    }
}

impl Path {
    #[inline]
    pub fn new() -> cxx::UniquePtr<Self> {
        new_path()
    }

    pub fn dump(&self) {
        dump_path(self);
    }
}
