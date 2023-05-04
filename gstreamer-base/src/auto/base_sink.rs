// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstBaseSink")]
    pub struct BaseSink(Object<ffi::GstBaseSink, ffi::GstBaseSinkClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_base_sink_get_type(),
    }
}

impl BaseSink {
    pub const NONE: Option<&'static BaseSink> = None;
}

unsafe impl Send for BaseSink {}
unsafe impl Sync for BaseSink {}

pub trait BaseSinkExt: 'static {
    //#[doc(alias = "gst_base_sink_do_preroll")]
    //fn do_preroll(&self, obj: /*Ignored*/&gst::MiniObject) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_base_sink_get_blocksize")]
    #[doc(alias = "get_blocksize")]
    fn blocksize(&self) -> u32;

    #[doc(alias = "gst_base_sink_get_drop_out_of_segment")]
    #[doc(alias = "get_drop_out_of_segment")]
    fn drops_out_of_segment(&self) -> bool;

    #[doc(alias = "gst_base_sink_get_last_sample")]
    #[doc(alias = "get_last_sample")]
    fn last_sample(&self) -> Option<gst::Sample>;

    #[doc(alias = "gst_base_sink_get_latency")]
    #[doc(alias = "get_latency")]
    fn latency(&self) -> gst::ClockTime;

    #[doc(alias = "gst_base_sink_get_max_bitrate")]
    #[doc(alias = "get_max_bitrate")]
    fn max_bitrate(&self) -> u64;

    #[doc(alias = "gst_base_sink_get_max_lateness")]
    #[doc(alias = "get_max_lateness")]
    fn max_lateness(&self) -> i64;

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_base_sink_get_processing_deadline")]
    #[doc(alias = "get_processing_deadline")]
    fn processing_deadline(&self) -> gst::ClockTime;

    #[doc(alias = "gst_base_sink_get_render_delay")]
    #[doc(alias = "get_render_delay")]
    fn render_delay(&self) -> gst::ClockTime;

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_base_sink_get_stats")]
    #[doc(alias = "get_stats")]
    fn stats(&self) -> gst::Structure;

    #[doc(alias = "gst_base_sink_get_sync")]
    #[doc(alias = "get_sync")]
    fn is_sync(&self) -> bool;

    #[doc(alias = "gst_base_sink_get_throttle_time")]
    #[doc(alias = "get_throttle_time")]
    fn throttle_time(&self) -> u64;

    #[doc(alias = "gst_base_sink_get_ts_offset")]
    #[doc(alias = "get_ts_offset")]
    fn ts_offset(&self) -> gst::ClockTimeDiff;

    #[doc(alias = "gst_base_sink_set_blocksize")]
    fn set_blocksize(&self, blocksize: u32);

    #[doc(alias = "gst_base_sink_set_drop_out_of_segment")]
    fn set_drop_out_of_segment(&self, drop_out_of_segment: bool);

    #[doc(alias = "gst_base_sink_set_max_bitrate")]
    fn set_max_bitrate(&self, max_bitrate: u64);

    #[doc(alias = "gst_base_sink_set_max_lateness")]
    fn set_max_lateness(&self, max_lateness: i64);

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_base_sink_set_processing_deadline")]
    fn set_processing_deadline(&self, processing_deadline: gst::ClockTime);

    #[doc(alias = "gst_base_sink_set_render_delay")]
    fn set_render_delay(&self, delay: gst::ClockTime);

    #[doc(alias = "gst_base_sink_set_sync")]
    fn set_sync(&self, sync: bool);

    #[doc(alias = "gst_base_sink_set_throttle_time")]
    fn set_throttle_time(&self, throttle: u64);

    #[doc(alias = "gst_base_sink_set_ts_offset")]
    fn set_ts_offset(&self, offset: gst::ClockTimeDiff);

    #[doc(alias = "gst_base_sink_wait")]
    fn wait(
        &self,
        time: impl Into<Option<gst::ClockTime>>,
    ) -> (Result<gst::FlowSuccess, gst::FlowError>, gst::ClockTimeDiff);

    #[doc(alias = "gst_base_sink_wait_clock")]
    fn wait_clock(
        &self,
        time: gst::ClockTime,
    ) -> (
        Result<gst::ClockSuccess, gst::ClockError>,
        gst::ClockTimeDiff,
    );

    #[doc(alias = "gst_base_sink_wait_preroll")]
    fn wait_preroll(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "async")]
    fn is_async(&self) -> bool;

    #[doc(alias = "async")]
    fn set_async(&self, async_: bool);

    #[doc(alias = "enable-last-sample")]
    fn enables_last_sample(&self) -> bool;

    #[doc(alias = "enable-last-sample")]
    fn set_enable_last_sample(&self, enable_last_sample: bool);

    fn is_qos(&self) -> bool;

    fn set_qos(&self, qos: bool);

    #[doc(alias = "async")]
    fn connect_async_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "blocksize")]
    fn connect_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "enable-last-sample")]
    fn connect_enable_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "last-sample")]
    fn connect_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "max-bitrate")]
    fn connect_max_bitrate_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "max-lateness")]
    fn connect_max_lateness_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "processing-deadline")]
    fn connect_processing_deadline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "qos")]
    fn connect_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "render-delay")]
    fn connect_render_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "stats")]
    fn connect_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "sync")]
    fn connect_sync_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "throttle-time")]
    fn connect_throttle_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "ts-offset")]
    fn connect_ts_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BaseSink>> BaseSinkExt for O {
    //fn do_preroll(&self, obj: /*Ignored*/&gst::MiniObject) -> Result<gst::FlowSuccess, gst::FlowError> {
    //    unsafe { TODO: call ffi:gst_base_sink_do_preroll() }
    //}

    fn blocksize(&self) -> u32 {
        unsafe { ffi::gst_base_sink_get_blocksize(self.as_ref().to_glib_none().0) }
    }

    fn drops_out_of_segment(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_sink_get_drop_out_of_segment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn last_sample(&self) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_base_sink_get_last_sample(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn latency(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_base_sink_get_latency(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn max_bitrate(&self) -> u64 {
        unsafe { ffi::gst_base_sink_get_max_bitrate(self.as_ref().to_glib_none().0) }
    }

    fn max_lateness(&self) -> i64 {
        unsafe { ffi::gst_base_sink_get_max_lateness(self.as_ref().to_glib_none().0) }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    fn processing_deadline(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_base_sink_get_processing_deadline(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn render_delay(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_base_sink_get_render_delay(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn stats(&self) -> gst::Structure {
        unsafe { from_glib_full(ffi::gst_base_sink_get_stats(self.as_ref().to_glib_none().0)) }
    }

    fn is_sync(&self) -> bool {
        unsafe { from_glib(ffi::gst_base_sink_get_sync(self.as_ref().to_glib_none().0)) }
    }

    fn throttle_time(&self) -> u64 {
        unsafe { ffi::gst_base_sink_get_throttle_time(self.as_ref().to_glib_none().0) }
    }

    fn ts_offset(&self) -> gst::ClockTimeDiff {
        unsafe { ffi::gst_base_sink_get_ts_offset(self.as_ref().to_glib_none().0) }
    }

    fn set_blocksize(&self, blocksize: u32) {
        unsafe {
            ffi::gst_base_sink_set_blocksize(self.as_ref().to_glib_none().0, blocksize);
        }
    }

    fn set_drop_out_of_segment(&self, drop_out_of_segment: bool) {
        unsafe {
            ffi::gst_base_sink_set_drop_out_of_segment(
                self.as_ref().to_glib_none().0,
                drop_out_of_segment.into_glib(),
            );
        }
    }

    fn set_max_bitrate(&self, max_bitrate: u64) {
        unsafe {
            ffi::gst_base_sink_set_max_bitrate(self.as_ref().to_glib_none().0, max_bitrate);
        }
    }

    fn set_max_lateness(&self, max_lateness: i64) {
        unsafe {
            ffi::gst_base_sink_set_max_lateness(self.as_ref().to_glib_none().0, max_lateness);
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    fn set_processing_deadline(&self, processing_deadline: gst::ClockTime) {
        unsafe {
            ffi::gst_base_sink_set_processing_deadline(
                self.as_ref().to_glib_none().0,
                processing_deadline.into_glib(),
            );
        }
    }

    fn set_render_delay(&self, delay: gst::ClockTime) {
        unsafe {
            ffi::gst_base_sink_set_render_delay(self.as_ref().to_glib_none().0, delay.into_glib());
        }
    }

    fn set_sync(&self, sync: bool) {
        unsafe {
            ffi::gst_base_sink_set_sync(self.as_ref().to_glib_none().0, sync.into_glib());
        }
    }

    fn set_throttle_time(&self, throttle: u64) {
        unsafe {
            ffi::gst_base_sink_set_throttle_time(self.as_ref().to_glib_none().0, throttle);
        }
    }

    fn set_ts_offset(&self, offset: gst::ClockTimeDiff) {
        unsafe {
            ffi::gst_base_sink_set_ts_offset(self.as_ref().to_glib_none().0, offset);
        }
    }

    fn wait(
        &self,
        time: impl Into<Option<gst::ClockTime>>,
    ) -> (Result<gst::FlowSuccess, gst::FlowError>, gst::ClockTimeDiff) {
        unsafe {
            let mut jitter = mem::MaybeUninit::uninit();
            let ret = try_from_glib(ffi::gst_base_sink_wait(
                self.as_ref().to_glib_none().0,
                time.into().into_glib(),
                jitter.as_mut_ptr(),
            ));
            (ret, jitter.assume_init())
        }
    }

    fn wait_clock(
        &self,
        time: gst::ClockTime,
    ) -> (
        Result<gst::ClockSuccess, gst::ClockError>,
        gst::ClockTimeDiff,
    ) {
        unsafe {
            let mut jitter = mem::MaybeUninit::uninit();
            let ret = try_from_glib(ffi::gst_base_sink_wait_clock(
                self.as_ref().to_glib_none().0,
                time.into_glib(),
                jitter.as_mut_ptr(),
            ));
            (ret, jitter.assume_init())
        }
    }

    fn wait_preroll(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_base_sink_wait_preroll(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_async(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "async")
    }

    fn set_async(&self, async_: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "async", async_)
    }

    fn enables_last_sample(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "enable-last-sample")
    }

    fn set_enable_last_sample(&self, enable_last_sample: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "enable-last-sample", enable_last_sample)
    }

    fn is_qos(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "qos")
    }

    fn set_qos(&self, qos: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "qos", qos)
    }

    fn connect_async_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_async_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::async\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_async_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocksize_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::blocksize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_blocksize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_enable_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_last_sample_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-last-sample\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_last_sample_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_sample_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::last-sample\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_last_sample_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_max_bitrate_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_bitrate_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-bitrate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_bitrate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_max_lateness_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_lateness_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-lateness\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_lateness_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    fn connect_processing_deadline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_processing_deadline_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::processing-deadline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_processing_deadline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_render_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_render_delay_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::render-delay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_render_delay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    fn connect_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stats_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_sync_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sync_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sync\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sync_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_throttle_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_throttle_time_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::throttle-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_throttle_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ts_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ts_offset_trampoline<
            P: IsA<BaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ts-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ts_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
