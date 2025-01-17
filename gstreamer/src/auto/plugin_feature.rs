// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Object, Plugin};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstPluginFeature")]
    pub struct PluginFeature(Object<ffi::GstPluginFeature, ffi::GstPluginFeatureClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_plugin_feature_get_type(),
    }
}

impl PluginFeature {
    pub const NONE: Option<&'static PluginFeature> = None;
}

unsafe impl Send for PluginFeature {}
unsafe impl Sync for PluginFeature {}

pub trait PluginFeatureExt: IsA<PluginFeature> + 'static {
    #[doc(alias = "gst_plugin_feature_check_version")]
    fn check_version(&self, min_major: u32, min_minor: u32, min_micro: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_plugin_feature_check_version(
                self.as_ref().to_glib_none().0,
                min_major,
                min_minor,
                min_micro,
            ))
        }
    }

    #[doc(alias = "gst_plugin_feature_get_plugin")]
    #[doc(alias = "get_plugin")]
    fn plugin(&self) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_plugin_feature_get_plugin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_plugin_feature_get_plugin_name")]
    #[doc(alias = "get_plugin_name")]
    fn plugin_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_plugin_feature_get_plugin_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<PluginFeature>> PluginFeatureExt for O {}
