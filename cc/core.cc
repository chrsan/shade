#include "shade/cc/core.h"
#include "include/core/SkColor.h"
#include "include/core/SkGraphics.h"
#include "include/core/SkMatrix.h"
#include "shade/src/core.rs.h"

Matrix new_identity_matrix() {
  return SkMatrix();
}

Matrix invert_matrix(const Matrix& m, bool& ok) {
  SkMatrix inv;
  ok = m.invert(&inv);
  return inv;
}

std::unique_ptr<Canvas> new_rgba_canvas(std::uint32_t width,
                                        std::uint32_t height,
                                        rust::Vec<std::uint8_t>& pixels,
                                        std::size_t row_bytes,
                                        bool premultiplied) {
  SkGraphics::Init();
  SkAlphaType alpha_type =
      premultiplied ? kPremul_SkAlphaType : kUnpremul_SkAlphaType;
  SkImageInfo info =
      SkImageInfo::Make(width, height, kN32_SkColorType, alpha_type);
  return SkCanvas::MakeRasterDirect(
      info, pixels.data(), row_bytes != 0 ? row_bytes : info.minRowBytes());
}

std::unique_ptr<Paint> new_paint() {
  return std::make_unique<Paint>();
}

std::unique_ptr<Path> new_path() {
  return std::make_unique<Path>();
}

void dump_path(const Path& p) {
  p.dump();
}
