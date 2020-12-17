#ifndef SHADE_SVG_H_
#define SHADE_SVG_H_

#include "core.h"

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ShadeSvgDom ShadeSvgDom;

ShadeSvgDom* shade_svg_dom_create(const char* data, size_t data_size);
void shade_svg_dom_destroy(ShadeSvgDom* dom);
void shade_svg_dom_get_container_size(ShadeSvgDom* dom,
                                      float* width,
                                      float* height);
void shade_svg_dom_set_container_size(ShadeSvgDom* dom,
                                      float width,
                                      float height);
void shade_svg_dom_render(ShadeSvgDom* dom, ShadeCanvas* canvas);

#ifdef __cplusplus
}
#endif

#endif
