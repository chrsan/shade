#ifndef SHADE_CORE_H_
#define SHADE_CORE_H_

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#else
#include <stdbool.h>
#endif

typedef enum {
  SHADE_PAINT_STYLE_FILL,
  SHADE_PAINT_STYLE_STROKE,
  SHADE_PAINT_STYLE_STROKE_AND_FILL,
} ShadePaintStyle;

typedef enum {
  SHADE_STROKE_CAP_BUTT,
  SHADE_STROKE_CAP_ROUND,
  SHADE_STROKE_CAP_SQUARE,
} ShadeStrokeCap;

typedef enum {
  SHADE_STROKE_JOIN_MITER,
  SHADE_STROKE_JOIN_ROUND,
  SHADE_STROKE_JOIN_BEVEL,
} ShadeStrokeJoin;

typedef enum {
  SHADE_PATH_FILL_TYPE_WINDING,
  SHADE_PATH_FILL_TYPE_EVEN_ODD,
  SHADE_PATH_FILL_TYPE_INVERSE_WINDING,
  SHADE_PATH_FILL_TYPE_INVERSE_EVEN_ODD,
} ShadePathFillType;

typedef struct ShadeSurface ShadeSurface;
typedef struct ShadeCanvas ShadeCanvas;
typedef struct ShadePaint ShadePaint;
typedef struct ShadePath ShadePath;

ShadeSurface* shade_surface_create_rgba(uint32_t width,
                                        uint32_t height,
                                        uint8_t* pixels,
                                        const size_t* row_bytes);
ShadeSurface* shade_surface_create_rgba_premultiplied(uint32_t width,
                                                      uint32_t height,
                                                      uint8_t* pixels,
                                                      const size_t* row_bytes);
void shade_surface_destroy(ShadeSurface* surface);
ShadeCanvas* shade_surface_get_canvas(ShadeSurface* surface);
bool shade_is_surface_bgra();

void shade_canvas_clear(ShadeCanvas* canvas, uint32_t color);
void shade_canvas_flush(ShadeCanvas* canvas);
void shade_canvas_save(ShadeCanvas* canvas);
void shade_canvas_restore(ShadeCanvas* canvas);
void shade_canvas_scale(ShadeCanvas* canvas, float sx, float sy);
void shade_canvas_translate(ShadeCanvas* canvas, float dx, float dy);
void shade_canvas_set_matrix(ShadeCanvas* canvas, const float matrix[9]);
void shade_canvas_concat(ShadeCanvas* canvas, const float matrix[9]);
void shade_canvas_get_total_matrix(ShadeCanvas* canvas, float matrix[9]);
void shade_canvas_draw_path(ShadeCanvas* canvas,
                            ShadePath* path,
                            ShadePaint* paint);
void shade_canvas_draw_rect(ShadeCanvas* canvas,
                            float x,
                            float y,
                            float w,
                            float h,
                            ShadePaint* paint);

bool shade_matrix_create_inverse(const float matrix[9], float inverse[9]);

ShadePaint* shade_paint_create();
void shade_paint_destroy(ShadePaint* paint);
ShadePaintStyle shade_paint_get_style(ShadePaint* paint);
void shade_paint_set_style(ShadePaint* paint, ShadePaintStyle style);
uint32_t shade_paint_get_color(ShadePaint* paint);
void shade_paint_set_color(ShadePaint* paint, uint32_t color);
bool shade_paint_is_anti_alias(ShadePaint* paint);
void shade_paint_set_anti_alias(ShadePaint* paint, bool aa);
float shade_paint_get_stroke_width(ShadePaint* paint);
void shade_paint_set_stroke_width(ShadePaint* paint, float width);
ShadeStrokeCap shade_paint_get_stroke_cap(ShadePaint* paint);
void shade_paint_set_stroke_cap(ShadePaint* paint, ShadeStrokeCap cap);
ShadeStrokeJoin shade_paint_get_stroke_join(ShadePaint* paint);
void shade_paint_set_stroke_join(ShadePaint* paint, ShadeStrokeJoin join);
float shade_paint_get_stroke_miter(ShadePaint* paint);
void shade_paint_set_stroke_miter(ShadePaint* paint, float miter);
void shade_paint_reset(ShadePaint* paint);

ShadePath* shade_path_create();
void shade_path_destroy(ShadePath* path);
ShadePathFillType shade_path_get_fill_type(ShadePath* path);
void shade_path_set_fill_type(ShadePath* path, ShadePathFillType type);
void shade_path_move_to(ShadePath* path, float x, float y);
void shade_path_line_to(ShadePath* path, float x, float y);
void shade_path_quad_to(ShadePath* path,
                        float x1,
                        float y1,
                        float x2,
                        float y2);
void shade_path_conic_to(ShadePath* path,
                         float x1,
                         float y1,
                         float x2,
                         float y2,
                         float w);
void shade_path_cubic_to(ShadePath* path,
                         float x1,
                         float y1,
                         float x2,
                         float y2,
                         float x3,
                         float y3);
void shade_path_close(ShadePath* path);
void shade_path_reset(ShadePath* path);
void shade_path_rewind(ShadePath* path);
size_t shade_path_count_points(ShadePath* path);
size_t shade_path_count_verbs(ShadePath* path);
void shade_path_get_bounds(ShadePath* path,
                           float* left,
                           float* top,
                           float* right,
                           float* bottom);
void shade_path_compute_tight_bounds(ShadePath* path,
                                     float* left,
                                     float* top,
                                     float* right,
                                     float* bottom);

#ifdef __cplusplus
}
#endif

#endif
