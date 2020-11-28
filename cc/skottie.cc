#include "skottie.h"

#include <include/core/SkCanvas.h>
#include <include/core/SkRect.h>
#include <include/core/SkRefCnt.h>
#include <include/core/SkSize.h>
#include <modules/skottie/include/Skottie.h>

#define ANIMATION_CAST reinterpret_cast<skottie::Animation*>(animation)
#define CANVAS_CAST reinterpret_cast<SkCanvas*>(canvas)

extern "C" {
ShadeSkottieAnimation* shade_skottie_animation_create(
    const char* data,
    size_t data_size,
    ShadeSkottieAnimationProps* props) {
  if (!data || data_size == 0) {
    return nullptr;
  }

  // TODO: ResourceProvider
  auto anim = skottie::Animation::Builder().make(data, data_size);
  if (!anim) {
    return nullptr;
  }

  if (props) {
    props->duration = anim->duration();
    props->fps = anim->fps();
    props->width = anim->size().fWidth;
    props->height = anim->size().fHeight;
  }

  return reinterpret_cast<ShadeSkottieAnimation*>(anim.release());
}

void shade_skottie_animation_destroy(ShadeSkottieAnimation* animation) {
  SkSafeUnref(ANIMATION_CAST);
}

void shade_skottie_animation_render(ShadeSkottieAnimation* animation,
                                    ShadeCanvas* canvas,
                                    const float rect[4]) {
  if (rect) {
    SkRect r;
    r.setLTRB(rect[0], rect[1], rect[2], rect[3]);
    ANIMATION_CAST->render(CANVAS_CAST, &r);
  } else {
    ANIMATION_CAST->render(CANVAS_CAST);
  }
}

void shade_skottie_animation_seek_frame(ShadeSkottieAnimation* animation,
                                        double t) {
  ANIMATION_CAST->seekFrame(t);
}

void shade_skottie_animation_seek_frame_time(ShadeSkottieAnimation* animation,
                                             double t) {
  ANIMATION_CAST->seekFrameTime(t);
}
}
