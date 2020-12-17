#include "shade/cc/skottie.h"
#include "include/core/SkSize.h"
#include "modules/skottie/include/Skottie.h"
#include "shade/src/skottie.rs.h"

Animation::Animation(sk_sp<skottie::Animation> animation)
    : animation_(std::move(animation)) {}

double Animation::duration() const {
  return animation_->duration();
}

double Animation::fps() const {
  return animation_->fps();
}

void Animation::size(float& width, float& height) const {
  SkSize size = animation_->size();
  width = size.fWidth;
  height = size.fHeight;
}

void Animation::render(SkCanvas& canvas) const {
  animation_->render(&canvas);
}

void Animation::seek_frame(double t) {
  animation_->seekFrame(t);
}

void Animation::seek_frame_time(double t) {
  animation_->seekFrameTime(t);
}

std::unique_ptr<Animation> new_animation(rust::Slice<const std::uint8_t> data) {
  if (data.size() == 0) {
    return nullptr;
  }

  sk_sp<skottie::Animation> animation = skottie::Animation::Builder().make(
      reinterpret_cast<const char*>(data.data()), data.size());
  if (!animation) {
    return nullptr;
  }

  return std::make_unique<Animation>(std::move(animation));
}
