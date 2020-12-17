#include "svg.h"

#include <include/core/SkCanvas.h>
#include <include/core/SkRefCnt.h>
#include <include/core/SkSize.h>
#include <include/core/SkStream.h>
#include <modules/svg/include/SkSVGDOM.h>

#define DOM_CAST reinterpret_cast<SkSVGDOM*>(dom)
#define CANVAS_CAST reinterpret_cast<SkCanvas*>(canvas)

extern "C" {
ShadeSvgDom* shade_svg_dom_create(const char* data, size_t data_size) {
  if (!data || data_size == 0) {
    return nullptr;
  }

  // TODO: SkFontMgr
  SkMemoryStream stream(data, data_size);
  auto dom = SkSVGDOM::MakeFromStream(stream);
  if (!dom) {
    return nullptr;
  }

  return reinterpret_cast<ShadeSvgDom*>(dom.release());
}

void shade_svg_dom_destroy(ShadeSvgDom* dom) {
  SkSafeUnref(DOM_CAST);
}

void shade_svg_dom_get_container_size(ShadeSvgDom* dom,
                                      float* width,
                                      float* height) {
  if (!width || !height) {
    return;
  }

  auto size = DOM_CAST->containerSize();
  if (width) {
    *width = size.width();
  }

  if (height) {
    *height = size.height();
  }
}

void shade_svg_dom_set_container_size(ShadeSvgDom* dom,
                                      float width,
                                      float height) {
  auto size = SkSize::Make(width, height);
  DOM_CAST->setContainerSize(size);
}

void shade_svg_dom_render(ShadeSvgDom* dom, ShadeCanvas* canvas) {
  DOM_CAST->render(CANVAS_CAST);
}
}
