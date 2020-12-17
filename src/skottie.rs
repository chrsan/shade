#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("shade/cc/skottie.h");

        type Animation;

        fn new_animation(data: &[u8]) -> UniquePtr<Animation>;

        fn duration(self: &Animation) -> f64;

        fn fps(self: &Animation) -> f64;

        type SkSize = crate::core::SkSize;

        fn size(self: &Animation) -> SkSize;

        type SkCanvas = crate::core::SkCanvas;

        fn render(self: &Animation, canvas: Pin<&mut SkCanvas>);

        fn seek_frame(self: Pin<&mut Animation>, t: f64);

        fn seek_frame_time(self: Pin<&mut Animation>, t: f64);
    }
}

pub use self::ffi::*;

impl Animation {
    #[inline]
    pub fn new<T: AsRef<[u8]>>(data: T) -> cxx::UniquePtr<Self> {
        new_animation(data.as_ref())
    }
}
