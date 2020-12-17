#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("shade/cc/svg.h");

        type SvgDom;

        fn new_svg_dom(data: &[u8]) -> UniquePtr<SvgDom>;

        fn get_container_size(self: &SvgDom, width: &mut f32, height: &mut f32);

        fn set_container_size(self: Pin<&mut SvgDom>, width: f32, height: f32);

        type SkCanvas = crate::core::SkCanvas;

        fn render(self: &SvgDom, canvas: Pin<&mut SkCanvas>);
    }
}

pub use self::ffi::*;

impl SvgDom {
    #[inline]
    pub fn new<T: AsRef<[u8]>>(data: T) -> cxx::UniquePtr<SvgDom> {
        new_svg_dom(data.as_ref())
    }
}
