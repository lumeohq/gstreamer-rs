// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Bus, Object};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstDeviceMonitor")]
    pub struct DeviceMonitor(Object<ffi::GstDeviceMonitor, ffi::GstDeviceMonitorClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_device_monitor_get_type(),
    }
}

impl DeviceMonitor {
    pub const NONE: Option<&'static DeviceMonitor> = None;

    #[doc(alias = "gst_device_monitor_new")]
    pub fn new() -> DeviceMonitor {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_device_monitor_new()) }
    }
}

impl Default for DeviceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for DeviceMonitor {}
unsafe impl Sync for DeviceMonitor {}

pub trait DeviceMonitorExt: IsA<DeviceMonitor> + 'static {
    #[doc(alias = "gst_device_monitor_get_bus")]
    #[doc(alias = "get_bus")]
    fn bus(&self) -> Bus {
        unsafe {
            from_glib_full(ffi::gst_device_monitor_get_bus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_monitor_get_providers")]
    #[doc(alias = "get_providers")]
    fn providers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_providers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_monitor_get_show_all_devices")]
    #[doc(alias = "get_show_all_devices")]
    fn shows_all_devices(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_monitor_get_show_all_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_monitor_set_show_all_devices")]
    fn set_show_all_devices(&self, show_all: bool) {
        unsafe {
            ffi::gst_device_monitor_set_show_all_devices(
                self.as_ref().to_glib_none().0,
                show_all.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_device_monitor_start")]
    fn start(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_device_monitor_start(self.as_ref().to_glib_none().0),
                "Failed to start"
            )
        }
    }

    #[doc(alias = "gst_device_monitor_stop")]
    fn stop(&self) {
        unsafe {
            ffi::gst_device_monitor_stop(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "show-all")]
    fn shows_all(&self) -> bool {
        ObjectExt::property(self.as_ref(), "show-all")
    }

    #[doc(alias = "show-all")]
    fn set_show_all(&self, show_all: bool) {
        ObjectExt::set_property(self.as_ref(), "show-all", show_all)
    }

    #[doc(alias = "show-all")]
    fn connect_show_all_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_all_trampoline<
            P: IsA<DeviceMonitor>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDeviceMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-all\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_all_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DeviceMonitor>> DeviceMonitorExt for O {}
