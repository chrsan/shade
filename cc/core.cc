#include "shade/cc/core.h"
#include "include/core/SkColor.h"
#include "include/core/SkGraphics.h"
#include "include/core/SkMatrix.h"
#include "shade/src/core.rs.h"

SkMatrix new_identity_matrix() {
  return SkMatrix();
}

SkMatrix invert_matrix(const SkMatrix& m, bool& ok) {
  SkMatrix inv;
  ok = m.invert(&inv);
  return inv;
}

std::unique_ptr<SkCanvas> new_canvas(std::uint32_t width,
                                     std::uint32_t height,
                                     SkColorType color_type,
                                     SkAlphaType alpha_type,
                                     rust::Vec<std::uint8_t>& pixels,
                                     std::size_t row_bytes) {
  SkGraphics::Init();
  SkImageInfo info = SkImageInfo::Make(width, height, color_type, alpha_type);
  return SkCanvas::MakeRasterDirect(
      info, pixels.data(), row_bytes != 0 ? row_bytes : info.minRowBytes());
}

std::unique_ptr<SkPaint> new_paint() {
  return std::make_unique<SkPaint>();
}

std::unique_ptr<SkPath> new_path() {
  return std::make_unique<SkPath>();
}

void dump_path(const SkPath& p) {
  p.dump();
}
