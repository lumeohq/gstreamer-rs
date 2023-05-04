// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::VideoCodecFrame;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use crate::VideoDecoderRequestSyncPointFlags;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstVideoDecoder")]
    pub struct VideoDecoder(Object<ffi::GstVideoDecoder, ffi::GstVideoDecoderClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_video_decoder_get_type(),
    }
}

impl VideoDecoder {
    pub const NONE: Option<&'static VideoDecoder> = None;
}

unsafe impl Send for VideoDecoder {}
unsafe impl Sync for VideoDecoder {}

pub trait VideoDecoderExt: 'static {
    #[doc(alias = "gst_video_decoder_add_to_frame")]
    fn add_to_frame(&self, n_bytes: i32);

    #[doc(alias = "gst_video_decoder_allocate_output_buffer")]
    fn allocate_output_buffer(&self) -> Result<gst::Buffer, glib::BoolError>;

    #[doc(alias = "gst_video_decoder_drop_frame")]
    fn drop_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_drop_subframe")]
    fn drop_subframe(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_video_decoder_finish_frame")]
    fn finish_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_finish_subframe")]
    fn finish_subframe(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_video_decoder_get_buffer_pool")]
    #[doc(alias = "get_buffer_pool")]
    fn buffer_pool(&self) -> Option<gst::BufferPool>;

    #[doc(alias = "gst_video_decoder_get_estimate_rate")]
    #[doc(alias = "get_estimate_rate")]
    fn estimate_rate(&self) -> i32;

    #[doc(alias = "gst_video_decoder_get_max_decode_time")]
    #[doc(alias = "get_max_decode_time")]
    fn max_decode_time(&self, frame: &VideoCodecFrame) -> gst::ClockTimeDiff;

    #[doc(alias = "gst_video_decoder_get_max_errors")]
    #[doc(alias = "get_max_errors")]
    fn max_errors(&self) -> i32;

    #[doc(alias = "gst_video_decoder_get_needs_format")]
    #[doc(alias = "get_needs_format")]
    fn needs_format(&self) -> bool;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_get_needs_sync_point")]
    #[doc(alias = "get_needs_sync_point")]
    fn needs_sync_point(&self) -> bool;

    #[doc(alias = "gst_video_decoder_get_packetized")]
    #[doc(alias = "get_packetized")]
    fn is_packetized(&self) -> bool;

    #[doc(alias = "gst_video_decoder_get_pending_frame_size")]
    #[doc(alias = "get_pending_frame_size")]
    fn pending_frame_size(&self) -> usize;

    #[doc(alias = "gst_video_decoder_get_qos_proportion")]
    #[doc(alias = "get_qos_proportion")]
    fn qos_proportion(&self) -> f64;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_get_subframe_mode")]
    #[doc(alias = "get_subframe_mode")]
    fn is_subframe_mode(&self) -> bool;

    #[doc(alias = "gst_video_decoder_have_frame")]
    fn have_frame(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_have_last_subframe")]
    fn have_last_subframe(
        &self,
        frame: &VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_video_decoder_merge_tags")]
    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    #[doc(alias = "gst_video_decoder_proxy_getcaps")]
    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps;

    #[doc(alias = "gst_video_decoder_release_frame")]
    fn release_frame(&self, frame: VideoCodecFrame);

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_request_sync_point")]
    fn request_sync_point(&self, frame: &VideoCodecFrame, flags: VideoDecoderRequestSyncPointFlags);

    #[doc(alias = "gst_video_decoder_set_estimate_rate")]
    fn set_estimate_rate(&self, enabled: bool);

    #[doc(alias = "gst_video_decoder_set_max_errors")]
    fn set_max_errors(&self, num: i32);

    #[doc(alias = "gst_video_decoder_set_needs_format")]
    fn set_needs_format(&self, enabled: bool);

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_set_needs_sync_point")]
    fn set_needs_sync_point(&self, enabled: bool);

    #[doc(alias = "gst_video_decoder_set_packetized")]
    fn set_packetized(&self, packetized: bool);

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_set_subframe_mode")]
    fn set_subframe_mode(&self, subframe_mode: bool);

    #[doc(alias = "gst_video_decoder_set_use_default_pad_acceptcaps")]
    fn set_use_default_pad_acceptcaps(&self, use_: bool);

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "automatic-request-sync-point-flags")]
    fn automatic_request_sync_point_flags(&self) -> VideoDecoderRequestSyncPointFlags;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "automatic-request-sync-point-flags")]
    fn set_automatic_request_sync_point_flags(
        &self,
        automatic_request_sync_point_flags: VideoDecoderRequestSyncPointFlags,
    );

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "automatic-request-sync-points")]
    fn is_automatic_request_sync_points(&self) -> bool;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "automatic-request-sync-points")]
    fn set_automatic_request_sync_points(&self, automatic_request_sync_points: bool);

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "discard-corrupted-frames")]
    fn is_discard_corrupted_frames(&self) -> bool;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "discard-corrupted-frames")]
    fn set_discard_corrupted_frames(&self, discard_corrupted_frames: bool);

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-force-key-unit-interval")]
    fn min_force_key_unit_interval(&self) -> u64;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-force-key-unit-interval")]
    fn set_min_force_key_unit_interval(&self, min_force_key_unit_interval: u64);

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn is_qos(&self) -> bool;

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn set_qos(&self, qos: bool);

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "automatic-request-sync-point-flags")]
    fn connect_automatic_request_sync_point_flags_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "automatic-request-sync-points")]
    fn connect_automatic_request_sync_points_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "discard-corrupted-frames")]
    fn connect_discard_corrupted_frames_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "max-errors")]
    fn connect_max_errors_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-force-key-unit-interval")]
    fn connect_min_force_key_unit_interval_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "qos")]
    fn connect_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VideoDecoder>> VideoDecoderExt for O {
    fn add_to_frame(&self, n_bytes: i32) {
        unsafe {
            ffi::gst_video_decoder_add_to_frame(self.as_ref().to_glib_none().0, n_bytes);
        }
    }

    fn allocate_output_buffer(&self) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_video_decoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to allocate output buffer"))
        }
    }

    fn drop_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_video_decoder_drop_frame(
                self.as_ref().to_glib_none().0,
                frame.into_glib_ptr(),
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn drop_subframe(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_video_decoder_drop_subframe(
                self.as_ref().to_glib_none().0,
                frame.into_glib_ptr(),
            ))
        }
    }

    fn finish_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_video_decoder_finish_frame(
                self.as_ref().to_glib_none().0,
                frame.into_glib_ptr(),
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn finish_subframe(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_video_decoder_finish_subframe(
                self.as_ref().to_glib_none().0,
                frame.into_glib_ptr(),
            ))
        }
    }

    fn buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(ffi::gst_video_decoder_get_buffer_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn estimate_rate(&self) -> i32 {
        unsafe { ffi::gst_video_decoder_get_estimate_rate(self.as_ref().to_glib_none().0) }
    }

    fn max_decode_time(&self, frame: &VideoCodecFrame) -> gst::ClockTimeDiff {
        unsafe {
            ffi::gst_video_decoder_get_max_decode_time(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
            )
        }
    }

    fn max_errors(&self) -> i32 {
        unsafe { ffi::gst_video_decoder_get_max_errors(self.as_ref().to_glib_none().0) }
    }

    fn needs_format(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_decoder_get_needs_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn needs_sync_point(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_decoder_get_needs_sync_point(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_packetized(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_decoder_get_packetized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pending_frame_size(&self) -> usize {
        unsafe { ffi::gst_video_decoder_get_pending_frame_size(self.as_ref().to_glib_none().0) }
    }

    fn qos_proportion(&self) -> f64 {
        unsafe { ffi::gst_video_decoder_get_qos_proportion(self.as_ref().to_glib_none().0) }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn is_subframe_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_decoder_get_subframe_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn have_frame(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_video_decoder_have_frame(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn have_last_subframe(
        &self,
        frame: &VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_video_decoder_have_last_subframe(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
            ))
        }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_video_decoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_video_decoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn release_frame(&self, frame: VideoCodecFrame) {
        unsafe {
            ffi::gst_video_decoder_release_frame(
                self.as_ref().to_glib_none().0,
                frame.into_glib_ptr(),
            );
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn request_sync_point(
        &self,
        frame: &VideoCodecFrame,
        flags: VideoDecoderRequestSyncPointFlags,
    ) {
        unsafe {
            ffi::gst_video_decoder_request_sync_point(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
                flags.into_glib(),
            );
        }
    }

    fn set_estimate_rate(&self, enabled: bool) {
        unsafe {
            ffi::gst_video_decoder_set_estimate_rate(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_max_errors(&self, num: i32) {
        unsafe {
            ffi::gst_video_decoder_set_max_errors(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_needs_format(&self, enabled: bool) {
        unsafe {
            ffi::gst_video_decoder_set_needs_format(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn set_needs_sync_point(&self, enabled: bool) {
        unsafe {
            ffi::gst_video_decoder_set_needs_sync_point(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_packetized(&self, packetized: bool) {
        unsafe {
            ffi::gst_video_decoder_set_packetized(
                self.as_ref().to_glib_none().0,
                packetized.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn set_subframe_mode(&self, subframe_mode: bool) {
        unsafe {
            ffi::gst_video_decoder_set_subframe_mode(
                self.as_ref().to_glib_none().0,
                subframe_mode.into_glib(),
            );
        }
    }

    fn set_use_default_pad_acceptcaps(&self, use_: bool) {
        unsafe {
            ffi::gst_video_decoder_set_use_default_pad_acceptcaps(
                self.as_ref().to_glib_none().0,
                use_.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn automatic_request_sync_point_flags(&self) -> VideoDecoderRequestSyncPointFlags {
        glib::ObjectExt::property(self.as_ref(), "automatic-request-sync-point-flags")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn set_automatic_request_sync_point_flags(
        &self,
        automatic_request_sync_point_flags: VideoDecoderRequestSyncPointFlags,
    ) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "automatic-request-sync-point-flags",
            automatic_request_sync_point_flags,
        )
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn is_automatic_request_sync_points(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "automatic-request-sync-points")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn set_automatic_request_sync_points(&self, automatic_request_sync_points: bool) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "automatic-request-sync-points",
            automatic_request_sync_points,
        )
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn is_discard_corrupted_frames(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "discard-corrupted-frames")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn set_discard_corrupted_frames(&self, discard_corrupted_frames: bool) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "discard-corrupted-frames",
            discard_corrupted_frames,
        )
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn min_force_key_unit_interval(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "min-force-key-unit-interval")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn set_min_force_key_unit_interval(&self, min_force_key_unit_interval: u64) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "min-force-key-unit-interval",
            min_force_key_unit_interval,
        )
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn is_qos(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "qos")
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn set_qos(&self, qos: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "qos", qos)
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn connect_automatic_request_sync_point_flags_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_automatic_request_sync_point_flags_trampoline<
            P: IsA<VideoDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::automatic-request-sync-point-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_automatic_request_sync_point_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn connect_automatic_request_sync_points_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_automatic_request_sync_points_trampoline<
            P: IsA<VideoDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::automatic-request-sync-points\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_automatic_request_sync_points_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn connect_discard_corrupted_frames_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_discard_corrupted_frames_trampoline<
            P: IsA<VideoDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::discard-corrupted-frames\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_discard_corrupted_frames_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn connect_max_errors_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_errors_trampoline<
            P: IsA<VideoDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-errors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_errors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn connect_min_force_key_unit_interval_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_force_key_unit_interval_trampoline<
            P: IsA<VideoDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-force-key-unit-interval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_force_key_unit_interval_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn connect_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<
            P: IsA<VideoDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
