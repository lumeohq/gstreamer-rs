// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::EncodingTarget;
use glib::translate::*;
use std::mem;

#[doc(alias = "gst_codec_utils_aac_get_channels")]
pub fn codec_utils_aac_get_channels(audio_config: &[u8]) -> u32 {
    assert_initialized_main_thread!();
    let len = audio_config.len() as _;
    unsafe { ffi::gst_codec_utils_aac_get_channels(audio_config.to_glib_none().0, len) }
}

#[doc(alias = "gst_codec_utils_aac_get_index_from_sample_rate")]
pub fn codec_utils_aac_get_index_from_sample_rate(rate: u32) -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_codec_utils_aac_get_index_from_sample_rate(rate) }
}

#[doc(alias = "gst_codec_utils_aac_get_level")]
pub fn codec_utils_aac_get_level(audio_config: &[u8]) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = audio_config.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_aac_get_level(
            audio_config.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get AAC level"))
    }
}

#[doc(alias = "gst_codec_utils_aac_get_profile")]
pub fn codec_utils_aac_get_profile(audio_config: &[u8]) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = audio_config.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_aac_get_profile(
            audio_config.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get AAC profile"))
    }
}

#[doc(alias = "gst_codec_utils_aac_get_sample_rate")]
pub fn codec_utils_aac_get_sample_rate(audio_config: &[u8]) -> u32 {
    assert_initialized_main_thread!();
    let len = audio_config.len() as _;
    unsafe { ffi::gst_codec_utils_aac_get_sample_rate(audio_config.to_glib_none().0, len) }
}

#[doc(alias = "gst_codec_utils_aac_get_sample_rate_from_index")]
pub fn codec_utils_aac_get_sample_rate_from_index(sr_idx: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_codec_utils_aac_get_sample_rate_from_index(sr_idx) }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
#[doc(alias = "gst_codec_utils_caps_from_mime_codec")]
pub fn codec_utils_caps_from_mime_codec(codecs_field: &str) -> Option<gst::Caps> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_codec_utils_caps_from_mime_codec(
            codecs_field.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_codec_utils_h264_get_level")]
pub fn codec_utils_h264_get_level(sps: &[u8]) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = sps.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_h264_get_level(
            sps.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get H264 level"))
    }
}

#[doc(alias = "gst_codec_utils_h264_get_level_idc")]
pub fn codec_utils_h264_get_level_idc(level: &str) -> u8 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_codec_utils_h264_get_level_idc(level.to_glib_none().0) }
}

#[doc(alias = "gst_codec_utils_h264_get_profile")]
pub fn codec_utils_h264_get_profile(sps: &[u8]) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = sps.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_h264_get_profile(
            sps.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get H264 profile"))
    }
}

#[doc(alias = "gst_codec_utils_h265_get_level")]
pub fn codec_utils_h265_get_level(
    profile_tier_level: &[u8],
) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = profile_tier_level.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_h265_get_level(
            profile_tier_level.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get H265 level"))
    }
}

#[doc(alias = "gst_codec_utils_h265_get_level_idc")]
pub fn codec_utils_h265_get_level_idc(level: &str) -> u8 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_codec_utils_h265_get_level_idc(level.to_glib_none().0) }
}

#[doc(alias = "gst_codec_utils_h265_get_profile")]
pub fn codec_utils_h265_get_profile(
    profile_tier_level: &[u8],
) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = profile_tier_level.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_h265_get_profile(
            profile_tier_level.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get H265 profile"))
    }
}

#[doc(alias = "gst_codec_utils_h265_get_tier")]
pub fn codec_utils_h265_get_tier(
    profile_tier_level: &[u8],
) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = profile_tier_level.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_h265_get_tier(
            profile_tier_level.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get H265 tier"))
    }
}

#[doc(alias = "gst_codec_utils_mpeg4video_get_level")]
pub fn codec_utils_mpeg4video_get_level(
    vis_obj_seq: &[u8],
) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = vis_obj_seq.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_mpeg4video_get_level(
            vis_obj_seq.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get MPEG4 video level"))
    }
}

#[doc(alias = "gst_codec_utils_mpeg4video_get_profile")]
pub fn codec_utils_mpeg4video_get_profile(
    vis_obj_seq: &[u8],
) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    let len = vis_obj_seq.len() as _;
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_codec_utils_mpeg4video_get_profile(
            vis_obj_seq.to_glib_none().0,
            len,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to get MPEG4 video profile"))
    }
}

#[doc(alias = "gst_encoding_list_all_targets")]
pub fn encoding_list_all_targets(categoryname: Option<&str>) -> Vec<EncodingTarget> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::gst_encoding_list_all_targets(
            categoryname.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_encoding_list_available_categories")]
pub fn encoding_list_available_categories() -> Vec<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::gst_encoding_list_available_categories()) }
}

#[doc(alias = "gst_pb_utils_get_element_description")]
pub fn pb_utils_get_element_description(factory_name: &str) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_pb_utils_get_element_description(
            factory_name.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_pb_utils_get_sink_description")]
pub fn pb_utils_get_sink_description(protocol: &str) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_pb_utils_get_sink_description(
            protocol.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_pb_utils_get_source_description")]
pub fn pb_utils_get_source_description(protocol: &str) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_pb_utils_get_source_description(
            protocol.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_plugins_base_version")]
pub fn plugins_base_version() -> (u32, u32, u32, u32) {
    skip_assert_initialized!();
    unsafe {
        let mut major = mem::MaybeUninit::uninit();
        let mut minor = mem::MaybeUninit::uninit();
        let mut micro = mem::MaybeUninit::uninit();
        let mut nano = mem::MaybeUninit::uninit();
        ffi::gst_plugins_base_version(
            major.as_mut_ptr(),
            minor.as_mut_ptr(),
            micro.as_mut_ptr(),
            nano.as_mut_ptr(),
        );
        (
            major.assume_init(),
            minor.assume_init(),
            micro.assume_init(),
            nano.assume_init(),
        )
    }
}

#[doc(alias = "gst_plugins_base_version_string")]
pub fn plugins_base_version_string() -> glib::GString {
    skip_assert_initialized!();
    unsafe { from_glib_full(ffi::gst_plugins_base_version_string()) }
}
