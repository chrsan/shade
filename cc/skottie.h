#ifndef SHADE_SKOTTIE_H_
#define SHADE_SKOTTIE_H_

#include "core.h"

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ShadeSkottieAnimation ShadeSkottieAnimation;

typedef struct {
  double duration;
  double fps;
  int32_t width;
  int32_t height;
} ShadeSkottieAnimationProps;

ShadeSkottieAnimation* shade_skottie_animation_create(
    const char* data,
    size_t data_size,
    ShadeSkottieAnimationProps* props);
void shade_skottie_animation_destroy(ShadeSkottieAnimation* animation);
void shade_skottie_animation_render(ShadeSkottieAnimation* animation,
                                    ShadeCanvas* canvas,
                                    const float rect[4]);
void shade_skottie_animation_seek_frame(ShadeSkottieAnimation* animation,
                                        double t);
void shade_skottie_animation_seek_frame_time(ShadeSkottieAnimation* animation,
                                             double t);

#ifdef __cplusplus
}
#endif

#endif
