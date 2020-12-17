use crate::core::{Canvas, Rect};
use crate::skottie_bindings::*;
use std::ptr;

#[derive(Debug)]
pub struct Animation {
    anim: *mut ShadeSkottieAnimation,
    duration: f64,
    fps: f64,
    width: i32,
    height: i32,
}

impl Animation {
    pub fn new(json: &str) -> Option<Self> {
        let mut props = ShadeSkottieAnimationProps::default();
        let anim = unsafe {
            shade_skottie_animation_create(
                json.as_ptr() as *const _,
                json.len(),
                &mut props as *mut _,
            )
        };
        if anim.is_null() {
            None
        } else {
            Some(Self {
                anim,
                duration: props.duration,
                fps: props.fps,
                width: props.width,
                height: props.height,
            })
        }
    }

    #[inline]
    pub fn duration(&self) -> f64 {
        self.duration
    }

    #[inline]
    pub fn fps(&self) -> f64 {
        self.fps
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn render(&self, canvas: &mut Canvas, rect: Option<Rect>) {
        if let Some(ref rect) = rect {
            let r = [rect.min.x, rect.min.y, rect.max.x, rect.max.y];
            unsafe {
                shade_skottie_animation_render(self.anim, canvas.0, r.as_ptr());
            }
        } else {
            unsafe {
                shade_skottie_animation_render(self.anim, canvas.0, ptr::null());
            }
        }
    }

    pub fn seek_frame(&mut self, t: f64) {
        unsafe {
            shade_skottie_animation_seek_frame(self.anim, t);
        }
    }

    pub fn seek_frame_time(&mut self, t: f64) {
        unsafe {
            shade_skottie_animation_seek_frame_time(self.anim, t);
        }
    }
}

impl Drop for Animation {
    fn drop(&mut self) {
        unsafe {
            shade_skottie_animation_destroy(self.anim);
        }
    }
}
