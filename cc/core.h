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

using ScaleToFit = SkMatrix::ScaleToFit;

SkMatrix new_identity_matrix();

SkMatrix invert_matrix(const SkMatrix& m, bool& ok);

constexpr SkColorType n32_color_type() {
  return kN32_SkColorType;
}

constexpr bool n32_color_type_is_bgra() {
  return kN32_SkColorType == kBGRA_8888_SkColorType;
}

std::unique_ptr<SkCanvas> new_canvas(std::uint32_t width,
                                     std::uint32_t height,
                                     SkColorType color_type,
                                     SkAlphaType alpha_type,
                                     rust::Vec<std::uint8_t>& pixels,
                                     std::size_t row_bytes);

using PaintStyle = SkPaint::Style;
using PaintStrokeCap = SkPaint::Cap;
using PaintStrokeJoin = SkPaint::Join;

std::unique_ptr<SkPaint> new_paint();

using PathFillType = SkPathFillType;
using PathDirection = SkPathDirection;

std::unique_ptr<SkPath> new_path();

void dump_path(const SkPath& p);
