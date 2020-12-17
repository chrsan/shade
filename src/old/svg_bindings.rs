/* automatically generated by rust-bindgen 0.55.1 */

use crate::core_bindings::ShadeCanvas;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ShadeSvgDom {
    _unused: [u8; 0],
}
extern "C" {
    pub fn shade_svg_dom_create(
        data: *const ::std::os::raw::c_char,
        data_size: usize,
    ) -> *mut ShadeSvgDom;
}
extern "C" {
    pub fn shade_svg_dom_destroy(dom: *mut ShadeSvgDom);
}
extern "C" {
    pub fn shade_svg_dom_get_container_size(
        dom: *mut ShadeSvgDom,
        width: *mut f32,
        height: *mut f32,
    );
}
extern "C" {
    pub fn shade_svg_dom_set_container_size(dom: *mut ShadeSvgDom, width: f32, height: f32);
}
extern "C" {
    pub fn shade_svg_dom_render(dom: *mut ShadeSvgDom, canvas: *mut ShadeCanvas);
}