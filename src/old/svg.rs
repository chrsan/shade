use crate::core::Canvas;
use crate::svg_bindings::*;

#[derive(Debug)]
pub struct Dom(*mut ShadeSvgDom);

impl Dom {
    pub fn new(svg: &str) -> Option<Self> {
        let dom = unsafe { shade_svg_dom_create(svg.as_ptr() as *const _, svg.len()) };
        if dom.is_null() {
            None
        } else {
            Some(Self(dom))
        }
    }

    #[inline]
    pub fn container_size(&self) -> (f32, f32) {
        let mut width = 0f32;
        let mut height = 0f32;
        unsafe {
            shade_svg_dom_get_container_size(self.0, &mut width as *mut _, &mut height as *mut _);
        }
        (width, height)
    }

    #[inline]
    pub fn set_container_size(&mut self, width: f32, height: f32) {
        unsafe {
            shade_svg_dom_set_container_size(self.0, width, height);
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        unsafe {
            shade_svg_dom_render(self.0, canvas.0);
        }
    }
}

impl Drop for Dom {
    fn drop(&mut self) {
        unsafe {
            shade_svg_dom_destroy(self.0);
        }
    }
}
