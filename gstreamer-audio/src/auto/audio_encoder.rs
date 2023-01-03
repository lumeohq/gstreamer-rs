// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::AudioInfo;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstAudioEncoder")]
    pub struct AudioEncoder(Object<ffi::GstAudioEncoder, ffi::GstAudioEncoderClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_encoder_get_type(),
    }
}

impl AudioEncoder {
    pub const NONE: Option<&'static AudioEncoder> = None;
}

unsafe impl Send for AudioEncoder {}
unsafe impl Sync for AudioEncoder {}

pub trait AudioEncoderExt: 'static {
    #[doc(alias = "gst_audio_encoder_allocate_output_buffer")]
    fn allocate_output_buffer(&self, size: usize) -> gst::Buffer;

    #[doc(alias = "gst_audio_encoder_get_audio_info")]
    #[doc(alias = "get_audio_info")]
    fn audio_info(&self) -> AudioInfo;

    #[doc(alias = "gst_audio_encoder_get_drainable")]
    #[doc(alias = "get_drainable")]
    fn is_drainable(&self) -> bool;

    #[doc(alias = "gst_audio_encoder_get_frame_max")]
    #[doc(alias = "get_frame_max")]
    fn frame_max(&self) -> i32;

    #[doc(alias = "gst_audio_encoder_get_frame_samples_max")]
    #[doc(alias = "get_frame_samples_max")]
    fn frame_samples_max(&self) -> i32;

    #[doc(alias = "gst_audio_encoder_get_frame_samples_min")]
    #[doc(alias = "get_frame_samples_min")]
    fn frame_samples_min(&self) -> i32;

    #[doc(alias = "gst_audio_encoder_get_hard_min")]
    #[doc(alias = "get_hard_min")]
    fn is_hard_min(&self) -> bool;

    #[doc(alias = "gst_audio_encoder_get_hard_resync")]
    #[doc(alias = "get_hard_resync")]
    fn is_hard_resync(&self) -> bool;

    #[doc(alias = "gst_audio_encoder_get_latency")]
    #[doc(alias = "get_latency")]
    fn latency(&self) -> (gst::ClockTime, Option<gst::ClockTime>);

    #[doc(alias = "gst_audio_encoder_get_lookahead")]
    #[doc(alias = "get_lookahead")]
    fn lookahead(&self) -> i32;

    #[doc(alias = "gst_audio_encoder_get_mark_granule")]
    #[doc(alias = "get_mark_granule")]
    fn is_mark_granule(&self) -> bool;

    #[doc(alias = "gst_audio_encoder_get_perfect_timestamp")]
    #[doc(alias = "get_perfect_timestamp")]
    fn is_perfect_timestamp(&self) -> bool;

    #[doc(alias = "gst_audio_encoder_get_tolerance")]
    #[doc(alias = "get_tolerance")]
    fn tolerance(&self) -> gst::ClockTime;

    #[doc(alias = "gst_audio_encoder_merge_tags")]
    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    #[doc(alias = "gst_audio_encoder_proxy_getcaps")]
    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps;

    #[doc(alias = "gst_audio_encoder_set_allocation_caps")]
    fn set_allocation_caps(&self, allocation_caps: Option<&gst::Caps>);

    #[doc(alias = "gst_audio_encoder_set_drainable")]
    fn set_drainable(&self, enabled: bool);

    #[doc(alias = "gst_audio_encoder_set_frame_max")]
    fn set_frame_max(&self, num: i32);

    #[doc(alias = "gst_audio_encoder_set_frame_samples_max")]
    fn set_frame_samples_max(&self, num: i32);

    #[doc(alias = "gst_audio_encoder_set_frame_samples_min")]
    fn set_frame_samples_min(&self, num: i32);

    #[doc(alias = "gst_audio_encoder_set_hard_min")]
    fn set_hard_min(&self, enabled: bool);

    #[doc(alias = "gst_audio_encoder_set_hard_resync")]
    fn set_hard_resync(&self, enabled: bool);

    #[doc(alias = "gst_audio_encoder_set_latency")]
    fn set_latency(&self, min: gst::ClockTime, max: impl Into<Option<gst::ClockTime>>);

    #[doc(alias = "gst_audio_encoder_set_lookahead")]
    fn set_lookahead(&self, num: i32);

    #[doc(alias = "gst_audio_encoder_set_mark_granule")]
    fn set_mark_granule(&self, enabled: bool);

    #[doc(alias = "gst_audio_encoder_set_perfect_timestamp")]
    fn set_perfect_timestamp(&self, enabled: bool);

    #[doc(alias = "gst_audio_encoder_set_tolerance")]
    fn set_tolerance(&self, tolerance: gst::ClockTime);

    #[doc(alias = "hard-resync")]
    fn connect_hard_resync_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "mark-granule")]
    fn connect_mark_granule_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "perfect-timestamp")]
    fn connect_perfect_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "tolerance")]
    fn connect_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AudioEncoder>> AudioEncoderExt for O {
    fn allocate_output_buffer(&self, size: usize) -> gst::Buffer {
        unsafe {
            from_glib_full(ffi::gst_audio_encoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
                size,
            ))
        }
    }

    fn audio_info(&self) -> AudioInfo {
        unsafe {
            from_glib_none(ffi::gst_audio_encoder_get_audio_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_drainable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_encoder_get_drainable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn frame_max(&self) -> i32 {
        unsafe { ffi::gst_audio_encoder_get_frame_max(self.as_ref().to_glib_none().0) }
    }

    fn frame_samples_max(&self) -> i32 {
        unsafe { ffi::gst_audio_encoder_get_frame_samples_max(self.as_ref().to_glib_none().0) }
    }

    fn frame_samples_min(&self) -> i32 {
        unsafe { ffi::gst_audio_encoder_get_frame_samples_min(self.as_ref().to_glib_none().0) }
    }

    fn is_hard_min(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_encoder_get_hard_min(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_hard_resync(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_encoder_get_hard_resync(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn latency(&self) -> (gst::ClockTime, Option<gst::ClockTime>) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            ffi::gst_audio_encoder_get_latency(
                self.as_ref().to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            (
                try_from_glib(min.assume_init()).expect("mandatory glib value is None"),
                from_glib(max.assume_init()),
            )
        }
    }

    fn lookahead(&self) -> i32 {
        unsafe { ffi::gst_audio_encoder_get_lookahead(self.as_ref().to_glib_none().0) }
    }

    fn is_mark_granule(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_encoder_get_mark_granule(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_perfect_timestamp(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_encoder_get_perfect_timestamp(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn tolerance(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_audio_encoder_get_tolerance(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_audio_encoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_audio_encoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn set_allocation_caps(&self, allocation_caps: Option<&gst::Caps>) {
        unsafe {
            ffi::gst_audio_encoder_set_allocation_caps(
                self.as_ref().to_glib_none().0,
                allocation_caps.to_glib_none().0,
            );
        }
    }

    fn set_drainable(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_encoder_set_drainable(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_frame_max(&self, num: i32) {
        unsafe {
            ffi::gst_audio_encoder_set_frame_max(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_frame_samples_max(&self, num: i32) {
        unsafe {
            ffi::gst_audio_encoder_set_frame_samples_max(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_frame_samples_min(&self, num: i32) {
        unsafe {
            ffi::gst_audio_encoder_set_frame_samples_min(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_hard_min(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_encoder_set_hard_min(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_hard_resync(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_encoder_set_hard_resync(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_latency(&self, min: gst::ClockTime, max: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_audio_encoder_set_latency(
                self.as_ref().to_glib_none().0,
                min.into_glib(),
                max.into().into_glib(),
            );
        }
    }

    fn set_lookahead(&self, num: i32) {
        unsafe {
            ffi::gst_audio_encoder_set_lookahead(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_mark_granule(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_encoder_set_mark_granule(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_perfect_timestamp(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_encoder_set_perfect_timestamp(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_tolerance(&self, tolerance: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_encoder_set_tolerance(
                self.as_ref().to_glib_none().0,
                tolerance.into_glib(),
            );
        }
    }

    fn connect_hard_resync_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_hard_resync_trampoline<
            P: IsA<AudioEncoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hard-resync\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hard_resync_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mark_granule_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mark_granule_trampoline<
            P: IsA<AudioEncoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mark-granule\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mark_granule_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_perfect_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_perfect_timestamp_trampoline<
            P: IsA<AudioEncoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::perfect-timestamp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_perfect_timestamp_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tolerance_trampoline<
            P: IsA<AudioEncoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tolerance\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tolerance_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
