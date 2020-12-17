#include "shade/cc/svg.h"
#include "include/core/SkStream.h"
#include "shade/src/svg.rs.h"

SvgDom::SvgDom(sk_sp<SkSVGDOM> dom) : dom_(std::move(dom)) {}

SkSize SvgDom::get_container_size() const {
  return dom_->containerSize();
}

void SvgDom::set_container_size(const SkSize& size) {
  dom_->setContainerSize(size);
}

void SvgDom::render(SkCanvas& canvas) const {
  dom_->render(&canvas);
}

std::unique_ptr<SvgDom> new_svg_dom(rust::Slice<const std::uint8_t> data) {
  if (data.size() == 0) {
    return nullptr;
  }

  SkMemoryStream stream(data.data(), data.size());
  sk_sp<SkSVGDOM> dom = SkSVGDOM::MakeFromStream(stream);
  if (!dom) {
    return nullptr;
  }

  return std::make_unique<SvgDom>(std::move(dom));
}
