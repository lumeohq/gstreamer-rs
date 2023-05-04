// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLContext;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstGLOverlayCompositor")]
    pub struct GLOverlayCompositor(Object<ffi::GstGLOverlayCompositor, ffi::GstGLOverlayCompositorClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_overlay_compositor_get_type(),
    }
}

impl GLOverlayCompositor {
    #[doc(alias = "gst_gl_overlay_compositor_new")]
    pub fn new(context: &impl IsA<GLContext>) -> GLOverlayCompositor {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_gl_overlay_compositor_new(
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_overlay_compositor_draw_overlays")]
    pub fn draw_overlays(&self) {
        unsafe {
            ffi::gst_gl_overlay_compositor_draw_overlays(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_overlay_compositor_free_overlays")]
    pub fn free_overlays(&self) {
        unsafe {
            ffi::gst_gl_overlay_compositor_free_overlays(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub fn is_yinvert(&self) -> bool {
        glib::ObjectExt::property(self, "yinvert")
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub fn set_yinvert(&self, yinvert: bool) {
        glib::ObjectExt::set_property(self, "yinvert", yinvert)
    }

    #[doc(alias = "gst_gl_overlay_compositor_add_caps")]
    pub fn add_caps(caps: &gst::Caps) -> gst::Caps {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_gl_overlay_compositor_add_caps(
                caps.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "yinvert")]
    pub fn connect_yinvert_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_yinvert_trampoline<
            F: Fn(&GLOverlayCompositor) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLOverlayCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::yinvert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_yinvert_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for GLOverlayCompositor {}
unsafe impl Sync for GLOverlayCompositor {}
