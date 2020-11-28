use crate::core_bindings::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    #[inline]
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            min: Point::new(left, top),
            max: Point::new(right, bottom),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        !(self.min.x < self.max.x && self.min.y < self.max.y)
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}

pub type MatrixArray = [f32; 9];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix(pub MatrixArray);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScaleToFit {
    Fill,
    Start,
    Center,
    End,
}

impl Matrix {
    pub const SCALE_X_INDEX: usize = 0;
    pub const SKEW_X_INDEX: usize = 1;
    pub const TRANS_X_INDEX: usize = 2;
    pub const SKEW_Y_INDEX: usize = 3;
    pub const SCALE_Y_INDEX: usize = 4;
    pub const TRANS_Y_INDEX: usize = 5;
    pub const PERSP_X_INDEX: usize = 6;
    pub const PERSP_Y_INDEX: usize = 7;
    pub const PERSP_BIAS_INDEX: usize = 8;

    #[inline]
    pub fn new() -> Self {
        Self::new_with_array(Self::identity_matrix_array())
    }

    pub fn rect_to_rect(src: &Rect, dst: &Rect, align: ScaleToFit) -> Self {
        let mut matrix = Self::new();
        if src.is_empty() {
            return matrix;
        }
        if dst.is_empty() {
            matrix.set_scale_x(0.0);
            matrix.set_scale_y(0.0);
            return matrix;
        }
        let mut sx = dst.width() / src.width();
        let mut sy = dst.height() / src.height();
        let mut x_larger = false;
        if align != ScaleToFit::Fill {
            if sx > sy {
                x_larger = true;
                sx = sy;
            } else {
                sy = sx;
            }
        }
        let mut tx = dst.min.x - src.min.x * sx;
        let mut ty = dst.min.y - src.min.x * sy;
        if align == ScaleToFit::Center || align == ScaleToFit::End {
            let mut diff = if x_larger {
                dst.width() - src.width() * sy
            } else {
                dst.height() - src.height() * sy
            };
            if align == ScaleToFit::Center {
                diff *= 0.5;
            }
            if x_larger {
                tx += diff;
            } else {
                ty += diff;
            }
        }
        matrix.set_scale_x(sx);
        matrix.set_scale_y(sy);
        matrix.set_translate_x(tx);
        matrix.set_translate_y(ty);
        matrix
    }

    #[inline]
    pub fn scale_x(&self) -> f32 {
        self.0[Self::SCALE_X_INDEX]
    }

    #[inline]
    pub fn set_scale_x(&mut self, value: f32) {
        self.0[Self::SCALE_X_INDEX] = value;
    }

    #[inline]
    pub fn scale_y(&self) -> f32 {
        self.0[Self::SCALE_Y_INDEX]
    }

    #[inline]
    pub fn set_scale_y(&mut self, value: f32) {
        self.0[Self::SCALE_Y_INDEX] = value;
    }

    #[inline]
    pub fn skew_x(&self) -> f32 {
        self.0[Self::SKEW_X_INDEX]
    }

    #[inline]
    pub fn set_skew_x(&mut self, value: f32) {
        self.0[Self::SKEW_X_INDEX] = value;
    }

    #[inline]
    pub fn skew_y(&self) -> f32 {
        self.0[Self::SKEW_Y_INDEX]
    }

    #[inline]
    pub fn set_skew_y(&mut self, value: f32) {
        self.0[Self::SKEW_Y_INDEX] = value;
    }

    #[inline]
    pub fn translate_x(&self) -> f32 {
        self.0[Self::TRANS_X_INDEX]
    }

    #[inline]
    pub fn set_translate_x(&mut self, value: f32) {
        self.0[Self::TRANS_X_INDEX] = value;
    }

    #[inline]
    pub fn translate_y(&self) -> f32 {
        self.0[Self::TRANS_Y_INDEX]
    }

    #[inline]
    pub fn set_translate_y(&mut self, value: f32) {
        self.0[Self::TRANS_Y_INDEX] = value;
    }

    #[inline]
    pub fn persp_x(&self) -> f32 {
        self.0[Self::PERSP_X_INDEX]
    }

    #[inline]
    pub fn set_persp_x(&mut self, value: f32) {
        self.0[Self::PERSP_X_INDEX] = value;
    }

    #[inline]
    pub fn persp_y(&self) -> f32 {
        self.0[Self::PERSP_Y_INDEX]
    }

    #[inline]
    pub fn set_persp_y(&mut self, value: f32) {
        self.0[Self::PERSP_Y_INDEX] = value;
    }

    #[inline]
    pub fn persp_bias(&self) -> f32 {
        self.0[Self::PERSP_BIAS_INDEX]
    }

    #[inline]
    pub fn set_persp_bias(&mut self, value: f32) {
        self.0[Self::PERSP_BIAS_INDEX] = value;
    }

    #[inline]
    pub fn reset(&mut self) {
        (*self).0 = Self::identity_matrix_array();
    }

    #[inline]
    pub fn invert(&self) -> Option<Self> {
        let mut mat = Self::identity_matrix_array();
        if unsafe { shade_matrix_create_inverse(self.0.as_ptr(), mat.as_mut_ptr()) } {
            Some(Self::new_with_array(mat))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) const fn identity_matrix_array() -> MatrixArray {
        [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
    }

    #[inline]
    pub(crate) fn new_with_array(mat: MatrixArray) -> Self {
        Self(mat)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AlphaType {
    Premultiplied,
    Unpremultiplied,
}

#[derive(Debug)]
pub struct Surface<'a> {
    pub(crate) surface: *mut ShadeSurface,
    pub(crate) canvas: Canvas,
    width: u32,
    height: u32,
    pixels: &'a [u8],
    row_bytes: Option<usize>,
    alpha_type: AlphaType,
}

impl<'a> Surface<'a> {
    #[inline]
    pub fn is_bgra() -> bool {
        unsafe { shade_is_surface_bgra() }
    }

    #[inline]
    pub const fn row_bytes(width: u32) -> usize {
        width as usize * 4
    }

    #[inline]
    pub fn new(
        width: u32,
        height: u32,
        pixels: &'a mut [u8],
        row_bytes: Option<usize>,
        alpha_type: AlphaType,
    ) -> Option<Self> {
        let stride = if let Some(ref stride) = row_bytes {
            stride as *const usize
        } else {
            std::ptr::null()
        };
        let surface = unsafe {
            match alpha_type {
                AlphaType::Premultiplied => shade_surface_create_rgba_premultiplied(
                    width,
                    height,
                    pixels.as_mut_ptr(),
                    stride,
                ),
                AlphaType::Unpremultiplied => {
                    shade_surface_create_rgba(width, height, pixels.as_mut_ptr(), stride)
                }
            }
        };
        if surface.is_null() {
            None
        } else {
            let canvas = Canvas(unsafe { shade_surface_get_canvas(surface) });
            assert!(!canvas.0.is_null());
            Some(Self {
                surface,
                canvas,
                width,
                height,
                pixels,
                row_bytes,
                alpha_type,
            })
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn pixels(&self) -> &[u8] {
        self.pixels
    }

    #[inline]
    pub fn alpha_type(&self) -> AlphaType {
        self.alpha_type
    }
}

impl<'a> Drop for Surface<'a> {
    fn drop(&mut self) {
        unsafe {
            shade_surface_destroy(self.surface);
        }
    }
}

impl<'a> std::ops::Deref for Surface<'a> {
    type Target = Canvas;

    fn deref(&self) -> &Self::Target {
        &self.canvas
    }
}

impl<'a> std::ops::DerefMut for Surface<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.canvas
    }
}

#[derive(Debug)]
pub struct Canvas(pub(crate) *mut ShadeCanvas);

impl Canvas {
    #[inline]
    pub fn clear(&mut self, color: Color) {
        unsafe {
            shade_canvas_clear(self.0, color.0);
        }
    }

    #[inline]
    pub fn flush(&mut self) {
        unsafe {
            shade_canvas_flush(self.0);
        }
    }

    #[inline]
    pub fn save(&mut self) {
        unsafe {
            shade_canvas_save(self.0);
        }
    }

    #[inline]
    pub fn restore(&mut self) {
        unsafe {
            shade_canvas_restore(self.0);
        }
    }

    #[inline]
    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe {
            shade_canvas_scale(self.0, sx, sy);
        }
    }

    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) {
        unsafe {
            shade_canvas_translate(self.0, dx, dy);
        }
    }

    #[inline]
    pub fn set_matrix(&mut self, matrix: &Matrix) {
        unsafe {
            shade_canvas_set_matrix(self.0, matrix.0.as_ptr());
        }
    }

    #[inline]
    pub fn concat(&mut self, matrix: &Matrix) {
        unsafe {
            shade_canvas_concat(self.0, matrix.0.as_ptr());
        }
    }

    #[inline]
    pub fn total_matrix(&self) -> Matrix {
        let mut matrix = Matrix::new();
        unsafe {
            shade_canvas_get_total_matrix(self.0, matrix.0.as_mut_ptr());
        }
        matrix
    }

    #[inline]
    pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
        unsafe {
            shade_canvas_draw_path(self.0, path.0, paint.0);
        }
    }

    #[inline]
    pub fn draw_rect(&mut self, rect: &Rect, paint: &Paint) {
        unsafe {
            shade_canvas_draw_rect(
                self.0,
                rect.min.x,
                rect.min.y,
                rect.width(),
                rect.height(),
                paint.0,
            );
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PaintStyle {
    Fill,
    Stroke,
    StrokeAndFill,
}

impl From<ShadePaintStyle> for PaintStyle {
    fn from(s: ShadePaintStyle) -> Self {
        match s {
            SHADE_PAINT_STYLE_FILL => PaintStyle::Fill,
            SHADE_PAINT_STYLE_STROKE => PaintStyle::Stroke,
            SHADE_PAINT_STYLE_STROKE_AND_FILL => PaintStyle::StrokeAndFill,
            _ => {
                unreachable!();
            }
        }
    }
}

impl From<PaintStyle> for ShadePaintStyle {
    fn from(s: PaintStyle) -> Self {
        match s {
            PaintStyle::Fill => SHADE_PAINT_STYLE_FILL,
            PaintStyle::Stroke => SHADE_PAINT_STYLE_STROKE,
            PaintStyle::StrokeAndFill => SHADE_PAINT_STYLE_STROKE_AND_FILL,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StrokeCap {
    Butt,
    Round,
    Square,
}

impl From<ShadeStrokeCap> for StrokeCap {
    fn from(c: ShadeStrokeCap) -> Self {
        match c {
            SHADE_STROKE_CAP_BUTT => StrokeCap::Butt,
            SHADE_STROKE_CAP_ROUND => StrokeCap::Round,
            SHADE_STROKE_CAP_SQUARE => StrokeCap::Square,
            _ => {
                unreachable!();
            }
        }
    }
}

impl From<StrokeCap> for ShadeStrokeCap {
    fn from(c: StrokeCap) -> Self {
        match c {
            StrokeCap::Butt => SHADE_STROKE_CAP_BUTT,
            StrokeCap::Round => SHADE_STROKE_CAP_ROUND,
            StrokeCap::Square => SHADE_STROKE_CAP_SQUARE,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StrokeJoin {
    Miter,
    Round,
    Bevel,
}

impl From<ShadeStrokeJoin> for StrokeJoin {
    fn from(j: ShadeStrokeJoin) -> Self {
        match j {
            SHADE_STROKE_JOIN_MITER => StrokeJoin::Miter,
            SHADE_STROKE_JOIN_ROUND => StrokeJoin::Round,
            SHADE_STROKE_JOIN_BEVEL => StrokeJoin::Bevel,
            _ => {
                unreachable!();
            }
        }
    }
}

impl From<StrokeJoin> for ShadeStrokeJoin {
    fn from(j: StrokeJoin) -> Self {
        match j {
            StrokeJoin::Miter => SHADE_STROKE_JOIN_MITER,
            StrokeJoin::Round => SHADE_STROKE_JOIN_ROUND,
            StrokeJoin::Bevel => SHADE_STROKE_JOIN_BEVEL,
        }
    }
}

#[derive(Debug)]
pub struct Paint(pub(crate) *mut ShadePaint);

impl Paint {
    #[inline]
    pub fn new() -> Option<Self> {
        let paint = unsafe { shade_paint_create() };
        if paint.is_null() {
            None
        } else {
            Some(Self(paint))
        }
    }

    #[inline]
    pub fn style(&self) -> PaintStyle {
        unsafe { shade_paint_get_style(self.0) }.into()
    }

    #[inline]
    pub fn set_style(&mut self, style: PaintStyle) {
        unsafe {
            shade_paint_set_style(self.0, style.into());
        }
    }

    #[inline]
    pub fn color(&self) -> Color {
        Color(unsafe { shade_paint_get_color(self.0) })
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        unsafe {
            shade_paint_set_color(self.0, color.0);
        }
    }

    #[inline]
    pub fn is_anti_alias(&self) -> bool {
        unsafe { shade_paint_is_anti_alias(self.0) }
    }

    #[inline]
    pub fn set_anti_alias(&mut self, value: bool) {
        unsafe {
            shade_paint_set_anti_alias(self.0, value);
        }
    }

    #[inline]
    pub fn stroke_width(&self) -> f32 {
        unsafe { shade_paint_get_stroke_width(self.0) }
    }

    #[inline]
    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe {
            shade_paint_set_stroke_width(self.0, width);
        }
    }

    #[inline]
    pub fn stroke_cap(&self) -> StrokeCap {
        unsafe { shade_paint_get_stroke_cap(self.0) }.into()
    }

    #[inline]
    pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
        unsafe {
            shade_paint_set_stroke_cap(self.0, cap.into());
        }
    }

    #[inline]
    pub fn stroke_join(&self) -> StrokeJoin {
        unsafe { shade_paint_get_stroke_join(self.0) }.into()
    }

    #[inline]
    pub fn set_stroke_join(&mut self, join: StrokeJoin) {
        unsafe {
            shade_paint_set_stroke_join(self.0, join.into());
        }
    }

    #[inline]
    pub fn stroke_miter(&self) -> f32 {
        unsafe { shade_paint_get_stroke_miter(self.0) }
    }

    #[inline]
    pub fn set_stroke_miter(&mut self, miter: f32) {
        unsafe {
            shade_paint_set_stroke_miter(self.0, miter);
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        unsafe {
            shade_paint_reset(self.0);
        }
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe {
            shade_paint_destroy(self.0);
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PathFillType {
    Winding,
    EvenOdd,
    InverseWinding,
    InverseEvenOdd,
}

impl From<ShadePathFillType> for PathFillType {
    fn from(t: ShadePathFillType) -> Self {
        match t {
            SHADE_PATH_FILL_TYPE_WINDING => PathFillType::Winding,
            SHADE_PATH_FILL_TYPE_EVEN_ODD => PathFillType::EvenOdd,
            SHADE_PATH_FILL_TYPE_INVERSE_WINDING => PathFillType::InverseWinding,
            SHADE_PATH_FILL_TYPE_INVERSE_EVEN_ODD => PathFillType::InverseEvenOdd,
            _ => {
                unreachable!();
            }
        }
    }
}

impl From<PathFillType> for ShadePathFillType {
    fn from(t: PathFillType) -> Self {
        match t {
            PathFillType::Winding => SHADE_PATH_FILL_TYPE_WINDING,
            PathFillType::EvenOdd => SHADE_PATH_FILL_TYPE_EVEN_ODD,
            PathFillType::InverseWinding => SHADE_PATH_FILL_TYPE_INVERSE_WINDING,
            PathFillType::InverseEvenOdd => SHADE_PATH_FILL_TYPE_INVERSE_EVEN_ODD,
        }
    }
}

#[derive(Debug)]
pub struct Path(pub(crate) *mut ShadePath);

impl Path {
    #[inline]
    pub fn new() -> Option<Self> {
        let path = unsafe { shade_path_create() };
        if path.is_null() {
            None
        } else {
            Some(Self(path))
        }
    }

    #[inline]
    pub fn fill_type(&self) -> PathFillType {
        unsafe { shade_path_get_fill_type(self.0) }.into()
    }

    #[inline]
    pub fn set_fill_type(&mut self, fill_type: PathFillType) {
        unsafe {
            shade_path_set_fill_type(self.0, fill_type.into());
        }
    }

    #[inline]
    pub fn move_to(&mut self, x: f32, y: f32) {
        unsafe {
            shade_path_move_to(self.0, x, y);
        }
    }

    #[inline]
    pub fn line_to(&mut self, x: f32, y: f32) {
        unsafe {
            shade_path_line_to(self.0, x, y);
        }
    }

    #[inline]
    pub fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe {
            shade_path_quad_to(self.0, x1, y1, x2, y2);
        }
    }

    #[inline]
    pub fn conic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, weight: f32) {
        unsafe {
            shade_path_conic_to(self.0, x1, y1, x2, y2, weight);
        }
    }

    #[inline]
    pub fn cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        unsafe {
            shade_path_cubic_to(self.0, x1, y1, x2, y2, x3, y3);
        }
    }

    #[inline]
    pub fn close(&mut self) {
        unsafe {
            shade_path_close(self.0);
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        unsafe {
            shade_path_reset(self.0);
        }
    }

    #[inline]
    pub fn rewind(&mut self) {
        unsafe {
            shade_path_rewind(self.0);
        }
    }

    #[inline]
    pub fn count_points(&self) -> usize {
        unsafe { shade_path_count_points(self.0) }
    }

    #[inline]
    pub fn count_verbs(&self) -> usize {
        unsafe { shade_path_count_verbs(self.0) }
    }

    #[inline]
    pub fn bounds(&self) -> Rect {
        let mut left = 0f32;
        let mut top = 0f32;
        let mut right = 0f32;
        let mut bottom = 0f32;
        unsafe {
            shade_path_get_bounds(
                self.0,
                &mut left as *mut _,
                &mut top as *mut _,
                &mut right as *mut _,
                &mut bottom as *mut _,
            );
        }
        Rect::new(left, top, right, bottom)
    }

    #[inline]
    pub fn compute_tight_bounds(&self) -> Rect {
        let mut left = 0f32;
        let mut top = 0f32;
        let mut right = 0f32;
        let mut bottom = 0f32;
        unsafe {
            shade_path_compute_tight_bounds(
                self.0,
                &mut left as *mut _,
                &mut top as *mut _,
                &mut right as *mut _,
                &mut bottom as *mut _,
            );
        }
        Rect::new(left, top, right, bottom)
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe {
            shade_path_destroy(self.0);
        }
    }
}
