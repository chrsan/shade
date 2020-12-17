#pragma once

#include <memory>
#include "include/core/SkCanvas.h"
#include "include/core/SkRefCnt.h"
#include "modules/svg/include/SkSVGDOM.h"
#include "rust/cxx.h"

class SvgDom {
 public:
  explicit SvgDom(sk_sp<SkSVGDOM> dom);

  void get_container_size(float& width, float& height) const;
  void set_container_size(float width, float height);
  void render(SkCanvas& canvas) const;

 private:
  sk_sp<SkSVGDOM> dom_;
};

std::unique_ptr<SvgDom> new_svg_dom(rust::Slice<const std::uint8_t> data);
