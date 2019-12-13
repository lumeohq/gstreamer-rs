use glib::translate::*;
use gst::Caps;
use gst_audio_sys::GstAudioRingBufferSpec;

use AudioInfo;
use AudioRingBufferFormatType;

#[repr(C)]
pub struct AudioRingBufferSpec(pub(crate) GstAudioRingBufferSpec);

impl AudioRingBufferSpec {
    pub fn get_type(&self) -> AudioRingBufferFormatType {
        AudioRingBufferFormatType::from_glib(self.0.type_)
    }

    pub fn set_type(&mut self, value: AudioRingBufferFormatType) {
        self.0.type_ = value.to_glib();
    }

    pub fn get_caps(&mut self) -> Caps {
        unsafe { Caps::from_glib_none(self.0.caps) }
    }

    pub fn get_audio_info(&mut self) -> AudioInfo {
        unsafe { AudioInfo::from_glib_none(&mut self.0.info) }
    }

    pub fn get_latency_time(&self) -> u64 {
        self.0.latency_time
    }

    pub fn set_latency_time(&mut self, value: u64) {
        self.0.latency_time = value;
    }

    pub fn get_buffer_time(&self) -> u64 {
        self.0.buffer_time
    }

    pub fn set_buffer_time(&mut self, value: u64) {
        self.0.buffer_time = value;
    }

    pub fn get_segsize(&self) -> i32 {
        self.0.segsize
    }

    pub fn set_segsize(&mut self, value: i32) {
        self.0.segsize = value;
    }

    pub fn get_segtotal(&self) -> i32 {
        self.0.segtotal
    }

    pub fn set_segtotal(&mut self, value: i32) {
        self.0.segtotal = value;
    }

    pub fn get_seglatency(&self) -> i32 {
        self.0.seglatency
    }

    pub fn set_seglatency(&mut self, value: i32) {
        self.0.seglatency = value;
    }
}
