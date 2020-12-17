#pragma once

#include <memory>
#include "include/core/SkCanvas.h"
#include "include/core/SkRect.h"
#include "include/core/SkSize.h"
#include "modules/skottie/include/Skottie.h"
#include "rust/cxx.h"

class Animation {
 public:
  explicit Animation(sk_sp<skottie::Animation> animation);

  double duration() const;
  double fps() const;
  SkSize size() const;

  void render(SkCanvas& canvas) const;

  void seek_frame(double t);
  void seek_frame_time(double t);

 private:
  sk_sp<skottie::Animation> animation_;
};

std::unique_ptr<Animation> new_animation(rust::Slice<const std::uint8_t> data);
