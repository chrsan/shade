#include "core.h"

#include <include/core/SkCanvas.h>
#include <include/core/SkColor.h>
#include <include/core/SkGraphics.h>
#include <include/core/SkMatrix.h>
#include <include/core/SkPaint.h>
#include <include/core/SkPathEffect.h>
#include <include/core/SkRect.h>
#include <include/core/SkRefCnt.h>
#include <include/core/SkSurface.h>
#include "include/core/SkImageInfo.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathTypes.h"
#include "include/core/SkPoint.h"

namespace {
SkSurface* SkiaSurfaceCreateForPixels(uint32_t width,
                                      uint32_t height,
                                      SkAlphaType alpha_type,
                                      uint8_t* pixels,
                                      const size_t* row_bytes) {
  SkGraphics::Init();
  SkImageInfo info =
      SkImageInfo::Make(width, height, kN32_SkColorType, alpha_type);
  sk_sp<SkSurface> surface = SkSurface::MakeRasterDirect(
      info, pixels, row_bytes ? *row_bytes : info.minRowBytes());
  return surface.release();
}
}  // namespace

#define SURFACE_CAST reinterpret_cast<SkSurface*>(surface)
#define CANVAS_CAST reinterpret_cast<SkCanvas*>(canvas)
#define PAINT_CAST reinterpret_cast<SkPaint*>(paint)
#define PATH_CAST reinterpret_cast<SkPath*>(path)

extern "C" {
ShadeSurface* shade_surface_create_rgba(uint32_t width,
                                        uint32_t height,
                                        uint8_t* pixels,
                                        const size_t* row_bytes) {
  auto surface = SkiaSurfaceCreateForPixels(
      width, height, kUnpremul_SkAlphaType, pixels, row_bytes);
  return reinterpret_cast<ShadeSurface*>(surface);
}

ShadeSurface* shade_surface_create_rgba_premultiplied(uint32_t width,
                                                      uint32_t height,
                                                      uint8_t* pixels,
                                                      const size_t* row_bytes) {
  auto surface = SkiaSurfaceCreateForPixels(width, height, kPremul_SkAlphaType,
                                            pixels, row_bytes);
  return reinterpret_cast<ShadeSurface*>(surface);
}

void shade_surface_destroy(ShadeSurface* surface) {
  SkSafeUnref(SURFACE_CAST);
}

ShadeCanvas* shade_surface_get_canvas(ShadeSurface* surface) {
  auto canvas = SURFACE_CAST->getCanvas();
  return reinterpret_cast<ShadeCanvas*>(canvas);
}

bool shade_is_surface_bgra() {
  return kN32_SkColorType == kBGRA_8888_SkColorType;
}

void shade_canvas_clear(ShadeCanvas* canvas, uint32_t color) {
  CANVAS_CAST->clear(color);
}

void shade_canvas_flush(ShadeCanvas* canvas) {
  CANVAS_CAST->flush();
}

void shade_canvas_save(ShadeCanvas* canvas) {
  CANVAS_CAST->save();
}

void shade_canvas_restore(ShadeCanvas* canvas) {
  CANVAS_CAST->restore();
}

void shade_canvas_scale(ShadeCanvas* canvas, float sx, float sy) {
  CANVAS_CAST->scale(sx, sy);
}

void shade_canvas_translate(ShadeCanvas* canvas, float dx, float dy) {
  CANVAS_CAST->translate(dx, dy);
}

void shade_canvas_set_matrix(ShadeCanvas* canvas, const float matrix[9]) {
  SkMatrix m;
  m.set9(matrix);
  CANVAS_CAST->setMatrix(m);
}

void shade_canvas_concat(ShadeCanvas* canvas, const float matrix[9]) {
  SkMatrix m;
  m.set9(matrix);
  CANVAS_CAST->concat(m);
}

void shade_canvas_get_total_matrix(ShadeCanvas* canvas, float matrix[9]) {
  auto m = CANVAS_CAST->getTotalMatrix();
  m.get9(matrix);
}

void shade_canvas_draw_path(ShadeCanvas* canvas,
                            ShadePath* path,
                            ShadePaint* paint) {
  CANVAS_CAST->drawPath(*PATH_CAST, *PAINT_CAST);
}

void shade_canvas_draw_rect(ShadeCanvas* canvas,
                            float x,
                            float y,
                            float w,
                            float h,
                            ShadePaint* paint) {
  auto rect = SkRect::MakeXYWH(x, y, w, h);
  CANVAS_CAST->drawRect(rect, *PAINT_CAST);
}

bool shade_matrix_create_inverse(const float matrix[9], float inverse[9]) {
  SkMatrix i, m;
  m.set9(matrix);
  if (!m.invert(&i)) {
    return false;
  }

  i.get9(inverse);
  return true;
}

ShadePaint* shade_paint_create() {
  return reinterpret_cast<ShadePaint*>(new SkPaint());
}

void shade_paint_destroy(ShadePaint* paint) {
  auto p = PAINT_CAST;
  p->setShader(nullptr);
  p->setPathEffect(nullptr);
  delete p;
}

ShadePaintStyle shade_paint_get_style(ShadePaint* paint) {
  return static_cast<ShadePaintStyle>(PAINT_CAST->getStyle());
}

void shade_paint_set_style(ShadePaint* paint, ShadePaintStyle style) {
  PAINT_CAST->setStyle(static_cast<SkPaint::Style>(style));
}

uint32_t shade_paint_get_color(ShadePaint* paint) {
  return PAINT_CAST->getColor();
}

void shade_paint_set_color(ShadePaint* paint, uint32_t color) {
  PAINT_CAST->setColor(color);
}

bool shade_paint_is_anti_alias(ShadePaint* paint) {
  return PAINT_CAST->isAntiAlias();
}

void shade_paint_set_anti_alias(ShadePaint* paint, bool aa) {
  PAINT_CAST->setAntiAlias(aa);
}

float shade_paint_get_stroke_width(ShadePaint* paint) {
  return PAINT_CAST->getStrokeWidth();
}

void shade_paint_set_stroke_width(ShadePaint* paint, float width) {
  PAINT_CAST->setStrokeWidth(width);
}

ShadeStrokeCap shade_paint_get_stroke_cap(ShadePaint* paint) {
  return static_cast<ShadeStrokeCap>(PAINT_CAST->getStrokeCap());
}

void shade_paint_set_stroke_cap(ShadePaint* paint, ShadeStrokeCap cap) {
  PAINT_CAST->setStrokeCap(static_cast<SkPaint::Cap>(cap));
}

ShadeStrokeJoin shade_paint_get_stroke_join(ShadePaint* paint) {
  return static_cast<ShadeStrokeJoin>(PAINT_CAST->getStrokeJoin());
}

void shade_paint_set_stroke_join(ShadePaint* paint, ShadeStrokeJoin join) {
  PAINT_CAST->setStrokeJoin(static_cast<SkPaint::Join>(join));
}

float shade_paint_get_stroke_miter(ShadePaint* paint) {
  return PAINT_CAST->getStrokeMiter();
}

void shade_paint_set_stroke_miter(ShadePaint* paint, float miter) {
  PAINT_CAST->setStrokeMiter(miter);
}

void shade_paint_reset(ShadePaint* paint) {
  PAINT_CAST->reset();
}

ShadePath* shade_path_create() {
  return reinterpret_cast<ShadePath*>(new SkPath());
}

void shade_path_destroy(ShadePath* path) {
  delete PATH_CAST;
}

ShadePathFillType shade_path_get_fill_type(ShadePath* path) {
  return static_cast<ShadePathFillType>(PATH_CAST->getFillType());
}

void shade_path_set_fill_type(ShadePath* path, ShadePathFillType type) {
  PATH_CAST->setFillType(static_cast<SkPathFillType>(type));
}

void shade_path_move_to(ShadePath* path, float x, float y) {
  PATH_CAST->moveTo(x, y);
}

void shade_path_line_to(ShadePath* path, float x, float y) {
  PATH_CAST->lineTo(x, y);
}

void shade_path_quad_to(ShadePath* path,
                        float x1,
                        float y1,
                        float x2,
                        float y2) {
  PATH_CAST->quadTo(x1, y1, x2, y2);
}

void shade_path_conic_to(ShadePath* path,
                         float x1,
                         float y1,
                         float x2,
                         float y2,
                         float w) {
  PATH_CAST->conicTo(x1, y1, x2, y2, w);
}

void shade_path_cubic_to(ShadePath* path,
                         float x1,
                         float y1,
                         float x2,
                         float y2,
                         float x3,
                         float y3) {
  PATH_CAST->cubicTo(x1, y1, x2, y2, x3, y3);
}

void shade_path_close(ShadePath* path) {
  PATH_CAST->close();
}

void shade_path_reset(ShadePath* path) {
  PATH_CAST->reset();
}

void shade_path_rewind(ShadePath* path) {
  PATH_CAST->rewind();
}

size_t shade_path_count_points(ShadePath* path) {
  return PATH_CAST->countPoints();
}

size_t shade_path_count_verbs(ShadePath* path) {
  return PATH_CAST->countVerbs();
}

void shade_path_get_bounds(ShadePath* path,
                           float* left,
                           float* top,
                           float* right,
                           float* bottom) {
  auto r = PATH_CAST->getBounds();
  if (left) {
    *left = r.fLeft;
  }

  if (top) {
    *top = r.fTop;
  }

  if (right) {
    *right = r.fRight;
  }

  if (bottom) {
    *bottom = r.fBottom;
  }
}

void shade_path_compute_tight_bounds(ShadePath* path,
                                     float* left,
                                     float* top,
                                     float* right,
                                     float* bottom) {
  auto r = PATH_CAST->computeTightBounds();
  if (left) {
    *left = r.fLeft;
  }

  if (top) {
    *top = r.fTop;
  }

  if (right) {
    *right = r.fRight;
  }

  if (bottom) {
    *bottom = r.fBottom;
  }
}
}
