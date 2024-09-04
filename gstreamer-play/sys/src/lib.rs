// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gobject_sys as gobject;
use gstreamer_sys as gst;
use gstreamer_video_sys as gst_video;

#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};
#[allow(unused_imports)]
use libc::{intptr_t, off_t, size_t, ssize_t, time_t, uintptr_t, FILE};
#[allow(unused_imports)]
use std::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GstPlayColorBalanceType = c_int;
pub const GST_PLAY_COLOR_BALANCE_HUE: GstPlayColorBalanceType = 3;
pub const GST_PLAY_COLOR_BALANCE_BRIGHTNESS: GstPlayColorBalanceType = 0;
pub const GST_PLAY_COLOR_BALANCE_SATURATION: GstPlayColorBalanceType = 2;
pub const GST_PLAY_COLOR_BALANCE_CONTRAST: GstPlayColorBalanceType = 1;

pub type GstPlayError = c_int;
pub const GST_PLAY_ERROR_FAILED: GstPlayError = 0;

pub type GstPlayMessage = c_int;
pub const GST_PLAY_MESSAGE_URI_LOADED: GstPlayMessage = 0;
pub const GST_PLAY_MESSAGE_POSITION_UPDATED: GstPlayMessage = 1;
pub const GST_PLAY_MESSAGE_DURATION_CHANGED: GstPlayMessage = 2;
pub const GST_PLAY_MESSAGE_STATE_CHANGED: GstPlayMessage = 3;
pub const GST_PLAY_MESSAGE_BUFFERING: GstPlayMessage = 4;
pub const GST_PLAY_MESSAGE_END_OF_STREAM: GstPlayMessage = 5;
pub const GST_PLAY_MESSAGE_ERROR: GstPlayMessage = 6;
pub const GST_PLAY_MESSAGE_WARNING: GstPlayMessage = 7;
pub const GST_PLAY_MESSAGE_VIDEO_DIMENSIONS_CHANGED: GstPlayMessage = 8;
pub const GST_PLAY_MESSAGE_MEDIA_INFO_UPDATED: GstPlayMessage = 9;
pub const GST_PLAY_MESSAGE_VOLUME_CHANGED: GstPlayMessage = 10;
pub const GST_PLAY_MESSAGE_MUTE_CHANGED: GstPlayMessage = 11;
pub const GST_PLAY_MESSAGE_SEEK_DONE: GstPlayMessage = 12;

pub type GstPlaySnapshotFormat = c_int;
pub const GST_PLAY_THUMBNAIL_RAW_NATIVE: GstPlaySnapshotFormat = 0;
pub const GST_PLAY_THUMBNAIL_RAW_xRGB: GstPlaySnapshotFormat = 1;
pub const GST_PLAY_THUMBNAIL_RAW_BGRx: GstPlaySnapshotFormat = 2;
pub const GST_PLAY_THUMBNAIL_JPG: GstPlaySnapshotFormat = 3;
pub const GST_PLAY_THUMBNAIL_PNG: GstPlaySnapshotFormat = 4;

pub type GstPlayState = c_int;
pub const GST_PLAY_STATE_STOPPED: GstPlayState = 0;
pub const GST_PLAY_STATE_BUFFERING: GstPlayState = 1;
pub const GST_PLAY_STATE_PAUSED: GstPlayState = 2;
pub const GST_PLAY_STATE_PLAYING: GstPlayState = 3;

// Records
#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlayAudioInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayAudioInfoClass = _GstPlayAudioInfoClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlayClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayClass = _GstPlayClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlayMediaInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayMediaInfoClass = _GstPlayMediaInfoClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlaySignalAdapterClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlaySignalAdapterClass = _GstPlaySignalAdapterClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlayStreamInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayStreamInfoClass = _GstPlayStreamInfoClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlaySubtitleInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlaySubtitleInfoClass = _GstPlaySubtitleInfoClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlayVideoInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayVideoInfoClass = _GstPlayVideoInfoClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GstPlayVideoOverlayVideoRendererClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayVideoOverlayVideoRendererClass = _GstPlayVideoOverlayVideoRendererClass;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstPlayVideoRendererInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub create_video_sink: Option<
        unsafe extern "C" fn(*mut GstPlayVideoRenderer, *mut GstPlay) -> *mut gst::GstElement,
    >,
}

impl ::std::fmt::Debug for GstPlayVideoRendererInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayVideoRendererInterface @ {self:p}"))
            .field("parent_iface", &self.parent_iface)
            .field("create_video_sink", &self.create_video_sink)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstPlayVisualization {
    pub name: *mut c_char,
    pub description: *mut c_char,
}

impl ::std::fmt::Debug for GstPlayVisualization {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayVisualization @ {self:p}"))
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

// Classes
#[repr(C)]
#[allow(dead_code)]
pub struct GstPlay {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlay @ {self:p}")).finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GstPlayAudioInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayAudioInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayAudioInfo @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GstPlayMediaInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayMediaInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayMediaInfo @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GstPlaySignalAdapter {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlaySignalAdapter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlaySignalAdapter @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GstPlayStreamInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayStreamInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayStreamInfo @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GstPlaySubtitleInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlaySubtitleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlaySubtitleInfo @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GstPlayVideoInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayVideoInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayVideoInfo @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GstPlayVideoOverlayVideoRenderer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayVideoOverlayVideoRenderer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayVideoOverlayVideoRenderer @ {self:p}"))
            .finish()
    }
}

// Interfaces
#[repr(C)]
#[allow(dead_code)]
pub struct GstPlayVideoRenderer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayVideoRenderer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "GstPlayVideoRenderer @ {self:p}")
    }
}

extern "C" {

    //=========================================================================
    // GstPlayColorBalanceType
    //=========================================================================
    pub fn gst_play_color_balance_type_get_type() -> GType;
    pub fn gst_play_color_balance_type_get_name(type_: GstPlayColorBalanceType) -> *const c_char;

    //=========================================================================
    // GstPlayError
    //=========================================================================
    pub fn gst_play_error_get_type() -> GType;
    pub fn gst_play_error_get_name(error: GstPlayError) -> *const c_char;
    pub fn gst_play_error_quark() -> glib::GQuark;

    //=========================================================================
    // GstPlayMessage
    //=========================================================================
    pub fn gst_play_message_get_type() -> GType;
    pub fn gst_play_message_get_name(message_type: GstPlayMessage) -> *const c_char;
    pub fn gst_play_message_parse_buffering_percent(
        msg: *mut gst::GstMessage,
        percent: *mut c_uint,
    );
    pub fn gst_play_message_parse_duration_updated(
        msg: *mut gst::GstMessage,
        duration: *mut gst::GstClockTime,
    );
    pub fn gst_play_message_parse_error(
        msg: *mut gst::GstMessage,
        error: *mut *mut glib::GError,
        details: *mut *mut gst::GstStructure,
    );
    pub fn gst_play_message_parse_media_info_updated(
        msg: *mut gst::GstMessage,
        info: *mut *mut GstPlayMediaInfo,
    );
    pub fn gst_play_message_parse_muted_changed(msg: *mut gst::GstMessage, muted: *mut gboolean);
    pub fn gst_play_message_parse_position_updated(
        msg: *mut gst::GstMessage,
        position: *mut gst::GstClockTime,
    );
    pub fn gst_play_message_parse_state_changed(
        msg: *mut gst::GstMessage,
        state: *mut GstPlayState,
    );
    pub fn gst_play_message_parse_type(msg: *mut gst::GstMessage, type_: *mut GstPlayMessage);
    pub fn gst_play_message_parse_video_dimensions_changed(
        msg: *mut gst::GstMessage,
        width: *mut c_uint,
        height: *mut c_uint,
    );
    pub fn gst_play_message_parse_volume_changed(msg: *mut gst::GstMessage, volume: *mut c_double);
    pub fn gst_play_message_parse_warning(
        msg: *mut gst::GstMessage,
        error: *mut *mut glib::GError,
        details: *mut *mut gst::GstStructure,
    );

    //=========================================================================
    // GstPlayState
    //=========================================================================
    pub fn gst_play_state_get_type() -> GType;
    pub fn gst_play_state_get_name(state: GstPlayState) -> *const c_char;

    //=========================================================================
    // GstPlayVisualization
    //=========================================================================
    pub fn gst_play_visualization_get_type() -> GType;
    pub fn gst_play_visualization_copy(
        vis: *const GstPlayVisualization,
    ) -> *mut GstPlayVisualization;
    pub fn gst_play_visualization_free(vis: *mut GstPlayVisualization);

    //=========================================================================
    // GstPlay
    //=========================================================================
    pub fn gst_play_get_type() -> GType;
    pub fn gst_play_new(video_renderer: *mut GstPlayVideoRenderer) -> *mut GstPlay;
    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    pub fn gst_play_config_get_pipeline_dump_in_error_details(
        config: *const gst::GstStructure,
    ) -> gboolean;
    pub fn gst_play_config_get_position_update_interval(config: *const gst::GstStructure)
        -> c_uint;
    pub fn gst_play_config_get_seek_accurate(config: *const gst::GstStructure) -> gboolean;
    pub fn gst_play_config_get_user_agent(config: *const gst::GstStructure) -> *mut c_char;
    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    pub fn gst_play_config_set_pipeline_dump_in_error_details(
        config: *mut gst::GstStructure,
        value: gboolean,
    );
    pub fn gst_play_config_set_position_update_interval(
        config: *mut gst::GstStructure,
        interval: c_uint,
    );
    pub fn gst_play_config_set_seek_accurate(config: *mut gst::GstStructure, accurate: gboolean);
    pub fn gst_play_config_set_user_agent(config: *mut gst::GstStructure, agent: *const c_char);
    pub fn gst_play_get_audio_streams(info: *const GstPlayMediaInfo) -> *mut glib::GList;
    pub fn gst_play_get_subtitle_streams(info: *const GstPlayMediaInfo) -> *mut glib::GList;
    pub fn gst_play_get_video_streams(info: *const GstPlayMediaInfo) -> *mut glib::GList;
    pub fn gst_play_is_play_message(msg: *mut gst::GstMessage) -> gboolean;
    pub fn gst_play_visualizations_free(viss: *mut *mut GstPlayVisualization);
    pub fn gst_play_visualizations_get() -> *mut *mut GstPlayVisualization;
    pub fn gst_play_get_audio_video_offset(play: *mut GstPlay) -> i64;
    pub fn gst_play_get_color_balance(
        play: *mut GstPlay,
        type_: GstPlayColorBalanceType,
    ) -> c_double;
    pub fn gst_play_get_config(play: *mut GstPlay) -> *mut gst::GstStructure;
    pub fn gst_play_get_current_audio_track(play: *mut GstPlay) -> *mut GstPlayAudioInfo;
    pub fn gst_play_get_current_subtitle_track(play: *mut GstPlay) -> *mut GstPlaySubtitleInfo;
    pub fn gst_play_get_current_video_track(play: *mut GstPlay) -> *mut GstPlayVideoInfo;
    pub fn gst_play_get_current_visualization(play: *mut GstPlay) -> *mut c_char;
    pub fn gst_play_get_duration(play: *mut GstPlay) -> gst::GstClockTime;
    pub fn gst_play_get_media_info(play: *mut GstPlay) -> *mut GstPlayMediaInfo;
    pub fn gst_play_get_message_bus(play: *mut GstPlay) -> *mut gst::GstBus;
    pub fn gst_play_get_multiview_flags(play: *mut GstPlay) -> gst_video::GstVideoMultiviewFlags;
    pub fn gst_play_get_multiview_mode(
        play: *mut GstPlay,
    ) -> gst_video::GstVideoMultiviewFramePacking;
    pub fn gst_play_get_mute(play: *mut GstPlay) -> gboolean;
    pub fn gst_play_get_pipeline(play: *mut GstPlay) -> *mut gst::GstElement;
    pub fn gst_play_get_position(play: *mut GstPlay) -> gst::GstClockTime;
    pub fn gst_play_get_rate(play: *mut GstPlay) -> c_double;
    pub fn gst_play_get_subtitle_uri(play: *mut GstPlay) -> *mut c_char;
    pub fn gst_play_get_subtitle_video_offset(play: *mut GstPlay) -> i64;
    pub fn gst_play_get_uri(play: *mut GstPlay) -> *mut c_char;
    pub fn gst_play_get_video_snapshot(
        play: *mut GstPlay,
        format: GstPlaySnapshotFormat,
        config: *const gst::GstStructure,
    ) -> *mut gst::GstSample;
    pub fn gst_play_get_volume(play: *mut GstPlay) -> c_double;
    pub fn gst_play_has_color_balance(play: *mut GstPlay) -> gboolean;
    pub fn gst_play_pause(play: *mut GstPlay);
    pub fn gst_play_play(play: *mut GstPlay);
    pub fn gst_play_seek(play: *mut GstPlay, position: gst::GstClockTime);
    pub fn gst_play_set_audio_track(play: *mut GstPlay, stream_index: c_int) -> gboolean;
    pub fn gst_play_set_audio_track_enabled(play: *mut GstPlay, enabled: gboolean);
    pub fn gst_play_set_audio_video_offset(play: *mut GstPlay, offset: i64);
    pub fn gst_play_set_color_balance(
        play: *mut GstPlay,
        type_: GstPlayColorBalanceType,
        value: c_double,
    );
    pub fn gst_play_set_config(play: *mut GstPlay, config: *mut gst::GstStructure) -> gboolean;
    pub fn gst_play_set_multiview_flags(
        play: *mut GstPlay,
        flags: gst_video::GstVideoMultiviewFlags,
    );
    pub fn gst_play_set_multiview_mode(
        play: *mut GstPlay,
        mode: gst_video::GstVideoMultiviewFramePacking,
    );
    pub fn gst_play_set_mute(play: *mut GstPlay, val: gboolean);
    pub fn gst_play_set_rate(play: *mut GstPlay, rate: c_double);
    pub fn gst_play_set_subtitle_track(play: *mut GstPlay, stream_index: c_int) -> gboolean;
    pub fn gst_play_set_subtitle_track_enabled(play: *mut GstPlay, enabled: gboolean);
    pub fn gst_play_set_subtitle_uri(play: *mut GstPlay, uri: *const c_char);
    pub fn gst_play_set_subtitle_video_offset(play: *mut GstPlay, offset: i64);
    pub fn gst_play_set_uri(play: *mut GstPlay, uri: *const c_char);
    pub fn gst_play_set_video_track(play: *mut GstPlay, stream_index: c_int) -> gboolean;
    pub fn gst_play_set_video_track_enabled(play: *mut GstPlay, enabled: gboolean);
    pub fn gst_play_set_visualization(play: *mut GstPlay, name: *const c_char) -> gboolean;
    pub fn gst_play_set_visualization_enabled(play: *mut GstPlay, enabled: gboolean);
    pub fn gst_play_set_volume(play: *mut GstPlay, val: c_double);
    pub fn gst_play_stop(play: *mut GstPlay);

    //=========================================================================
    // GstPlayAudioInfo
    //=========================================================================
    pub fn gst_play_audio_info_get_type() -> GType;
    pub fn gst_play_audio_info_get_bitrate(info: *const GstPlayAudioInfo) -> c_int;
    pub fn gst_play_audio_info_get_channels(info: *const GstPlayAudioInfo) -> c_int;
    pub fn gst_play_audio_info_get_language(info: *const GstPlayAudioInfo) -> *const c_char;
    pub fn gst_play_audio_info_get_max_bitrate(info: *const GstPlayAudioInfo) -> c_int;
    pub fn gst_play_audio_info_get_sample_rate(info: *const GstPlayAudioInfo) -> c_int;

    //=========================================================================
    // GstPlayMediaInfo
    //=========================================================================
    pub fn gst_play_media_info_get_type() -> GType;
    pub fn gst_play_media_info_get_audio_streams(info: *const GstPlayMediaInfo)
        -> *mut glib::GList;
    pub fn gst_play_media_info_get_container_format(info: *const GstPlayMediaInfo)
        -> *const c_char;
    pub fn gst_play_media_info_get_duration(info: *const GstPlayMediaInfo) -> gst::GstClockTime;
    pub fn gst_play_media_info_get_image_sample(
        info: *const GstPlayMediaInfo,
    ) -> *mut gst::GstSample;
    pub fn gst_play_media_info_get_number_of_audio_streams(info: *const GstPlayMediaInfo)
        -> c_uint;
    pub fn gst_play_media_info_get_number_of_streams(info: *const GstPlayMediaInfo) -> c_uint;
    pub fn gst_play_media_info_get_number_of_subtitle_streams(
        info: *const GstPlayMediaInfo,
    ) -> c_uint;
    pub fn gst_play_media_info_get_number_of_video_streams(info: *const GstPlayMediaInfo)
        -> c_uint;
    pub fn gst_play_media_info_get_stream_list(info: *const GstPlayMediaInfo) -> *mut glib::GList;
    pub fn gst_play_media_info_get_subtitle_streams(
        info: *const GstPlayMediaInfo,
    ) -> *mut glib::GList;
    pub fn gst_play_media_info_get_tags(info: *const GstPlayMediaInfo) -> *mut gst::GstTagList;
    pub fn gst_play_media_info_get_title(info: *const GstPlayMediaInfo) -> *const c_char;
    pub fn gst_play_media_info_get_uri(info: *const GstPlayMediaInfo) -> *const c_char;
    pub fn gst_play_media_info_get_video_streams(info: *const GstPlayMediaInfo)
        -> *mut glib::GList;
    pub fn gst_play_media_info_is_live(info: *const GstPlayMediaInfo) -> gboolean;
    pub fn gst_play_media_info_is_seekable(info: *const GstPlayMediaInfo) -> gboolean;

    //=========================================================================
    // GstPlaySignalAdapter
    //=========================================================================
    pub fn gst_play_signal_adapter_get_type() -> GType;
    pub fn gst_play_signal_adapter_new(play: *mut GstPlay) -> *mut GstPlaySignalAdapter;
    pub fn gst_play_signal_adapter_new_sync_emit(play: *mut GstPlay) -> *mut GstPlaySignalAdapter;
    pub fn gst_play_signal_adapter_new_with_main_context(
        play: *mut GstPlay,
        context: *mut glib::GMainContext,
    ) -> *mut GstPlaySignalAdapter;
    pub fn gst_play_signal_adapter_get_play(adapter: *mut GstPlaySignalAdapter) -> *mut GstPlay;

    //=========================================================================
    // GstPlayStreamInfo
    //=========================================================================
    pub fn gst_play_stream_info_get_type() -> GType;
    pub fn gst_play_stream_info_get_caps(info: *const GstPlayStreamInfo) -> *mut gst::GstCaps;
    pub fn gst_play_stream_info_get_codec(info: *const GstPlayStreamInfo) -> *const c_char;
    pub fn gst_play_stream_info_get_index(info: *const GstPlayStreamInfo) -> c_int;
    pub fn gst_play_stream_info_get_stream_type(info: *const GstPlayStreamInfo) -> *const c_char;
    pub fn gst_play_stream_info_get_tags(info: *const GstPlayStreamInfo) -> *mut gst::GstTagList;

    //=========================================================================
    // GstPlaySubtitleInfo
    //=========================================================================
    pub fn gst_play_subtitle_info_get_type() -> GType;
    pub fn gst_play_subtitle_info_get_language(info: *const GstPlaySubtitleInfo) -> *const c_char;

    //=========================================================================
    // GstPlayVideoInfo
    //=========================================================================
    pub fn gst_play_video_info_get_type() -> GType;
    pub fn gst_play_video_info_get_bitrate(info: *const GstPlayVideoInfo) -> c_int;
    pub fn gst_play_video_info_get_framerate(
        info: *const GstPlayVideoInfo,
        fps_n: *mut c_int,
        fps_d: *mut c_int,
    );
    pub fn gst_play_video_info_get_height(info: *const GstPlayVideoInfo) -> c_int;
    pub fn gst_play_video_info_get_max_bitrate(info: *const GstPlayVideoInfo) -> c_int;
    pub fn gst_play_video_info_get_pixel_aspect_ratio(
        info: *const GstPlayVideoInfo,
        par_n: *mut c_uint,
        par_d: *mut c_uint,
    );
    pub fn gst_play_video_info_get_width(info: *const GstPlayVideoInfo) -> c_int;

    //=========================================================================
    // GstPlayVideoOverlayVideoRenderer
    //=========================================================================
    pub fn gst_play_video_overlay_video_renderer_get_type() -> GType;
    pub fn gst_play_video_overlay_video_renderer_new(
        window_handle: gpointer,
    ) -> *mut GstPlayVideoRenderer;
    pub fn gst_play_video_overlay_video_renderer_new_with_sink(
        window_handle: gpointer,
        video_sink: *mut gst::GstElement,
    ) -> *mut GstPlayVideoRenderer;
    pub fn gst_play_video_overlay_video_renderer_expose(
        self_: *mut GstPlayVideoOverlayVideoRenderer,
    );
    pub fn gst_play_video_overlay_video_renderer_get_render_rectangle(
        self_: *mut GstPlayVideoOverlayVideoRenderer,
        x: *mut c_int,
        y: *mut c_int,
        width: *mut c_int,
        height: *mut c_int,
    );
    pub fn gst_play_video_overlay_video_renderer_get_window_handle(
        self_: *mut GstPlayVideoOverlayVideoRenderer,
    ) -> gpointer;
    pub fn gst_play_video_overlay_video_renderer_set_render_rectangle(
        self_: *mut GstPlayVideoOverlayVideoRenderer,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn gst_play_video_overlay_video_renderer_set_window_handle(
        self_: *mut GstPlayVideoOverlayVideoRenderer,
        window_handle: gpointer,
    );

    //=========================================================================
    // GstPlayVideoRenderer
    //=========================================================================
    pub fn gst_play_video_renderer_get_type() -> GType;

}
