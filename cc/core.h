#pragma once

#include <memory>
#include "include/core/SkBlendMode.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkImageInfo.h"
#include "include/core/SkMatrix.h"
#include "include/core/SkPaint.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathTypes.h"
#include "include/core/SkPoint.h"
#include "include/core/SkRect.h"
#include "rust/cxx.h"

using Point = SkPoint;

using Rect = SkRect;

using Matrix = SkMatrix;
using MatrixScaleToFit = SkMatrix::ScaleToFit;

Matrix new_identity_matrix();

Matrix invert_matrix(const Matrix& m, bool& ok);

constexpr bool n32_color_type_is_bgra() {
  return kN32_SkColorType == kBGRA_8888_SkColorType;
}

using Canvas = SkCanvas;

std::unique_ptr<Canvas> new_rgba_canvas(std::uint32_t width,
                                        std::uint32_t height,
                                        rust::Vec<std::uint8_t>& pixels,
                                        std::size_t row_bytes,
                                        bool premultiplied);

using BlendMode = SkBlendMode;

using Paint = SkPaint;
using PaintStyle = SkPaint::Style;
using PaintStrokeCap = SkPaint::Cap;
using PaintStrokeJoin = SkPaint::Join;

std::unique_ptr<Paint> new_paint();

using Path = SkPath;
using PathFillType = SkPathFillType;
using PathDirection = SkPathDirection;

std::unique_ptr<Path> new_path();

void dump_path(const Path& p);
