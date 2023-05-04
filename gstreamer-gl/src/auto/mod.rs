// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod gl_base_filter;
pub use self::gl_base_filter::GLBaseFilter;

mod gl_base_memory_allocator;
pub use self::gl_base_memory_allocator::GLBaseMemoryAllocator;

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
mod gl_base_src;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
pub use self::gl_base_src::GLBaseSrc;

mod gl_buffer_pool;
pub use self::gl_buffer_pool::GLBufferPool;

mod gl_color_convert;
pub use self::gl_color_convert::GLColorConvert;

mod gl_context;
pub use self::gl_context::GLContext;

mod gl_display;
pub use self::gl_display::GLDisplay;

mod gl_filter;
pub use self::gl_filter::GLFilter;

mod gl_framebuffer;
pub use self::gl_framebuffer::GLFramebuffer;

mod gl_memory_allocator;
pub use self::gl_memory_allocator::GLMemoryAllocator;

mod gl_overlay_compositor;
pub use self::gl_overlay_compositor::GLOverlayCompositor;

mod glsl_stage;
pub use self::glsl_stage::GLSLStage;

mod gl_shader;
pub use self::gl_shader::GLShader;

mod gl_upload;
pub use self::gl_upload::GLUpload;

mod gl_view_convert;
pub use self::gl_view_convert::GLViewConvert;

mod gl_window;
pub use self::gl_window::GLWindow;

mod gl_allocation_params;
pub use self::gl_allocation_params::GLAllocationParams;

mod gl_video_allocation_params;
pub use self::gl_video_allocation_params::GLVideoAllocationParams;

mod enums;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub use self::enums::GLConfigCaveat;
pub use self::enums::GLContextError;
pub use self::enums::GLFormat;
pub use self::enums::GLQueryType;
pub use self::enums::GLSLError;
pub use self::enums::GLSLVersion;
pub use self::enums::GLStereoDownmix;
pub use self::enums::GLTextureTarget;
pub use self::enums::GLUploadReturn;
pub use self::enums::GLWindowError;

mod flags;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub use self::flags::GLConfigSurfaceType;
pub use self::flags::GLDisplayType;
pub use self::flags::GLPlatform;
pub use self::flags::GLSLProfile;
pub use self::flags::GLAPI;

pub mod functions;

mod constants;
pub use self::constants::BUFFER_POOL_OPTION_GL_SYNC_META;
pub use self::constants::BUFFER_POOL_OPTION_GL_TEXTURE_TARGET_2D;
pub use self::constants::BUFFER_POOL_OPTION_GL_TEXTURE_TARGET_EXTERNAL_OES;
pub use self::constants::BUFFER_POOL_OPTION_GL_TEXTURE_TARGET_RECTANGLE;
pub use self::constants::CAPS_FEATURE_MEMORY_GL_BUFFER;
pub use self::constants::CAPS_FEATURE_MEMORY_GL_MEMORY;
pub use self::constants::GL_API_GLES1_NAME;
pub use self::constants::GL_API_GLES2_NAME;
pub use self::constants::GL_API_OPENGL3_NAME;
pub use self::constants::GL_API_OPENGL_NAME;
pub use self::constants::GL_BASE_MEMORY_ALLOCATOR_NAME;
pub use self::constants::GL_BUFFER_ALLOCATOR_NAME;
pub use self::constants::GL_COLOR_CONVERT_EXT_FORMATS;
pub use self::constants::GL_COLOR_CONVERT_VIDEO_CAPS;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub use self::constants::GL_CONFIG_STRUCTURE_NAME;
pub use self::constants::GL_CONTEXT_TYPE_CGL;
pub use self::constants::GL_CONTEXT_TYPE_EAGL;
pub use self::constants::GL_CONTEXT_TYPE_EGL;
pub use self::constants::GL_CONTEXT_TYPE_GLX;
pub use self::constants::GL_CONTEXT_TYPE_WGL;
pub use self::constants::GL_DISPLAY_CONTEXT_TYPE;
pub use self::constants::GL_MEMORY_ALLOCATOR_NAME;
pub use self::constants::GL_MEMORY_PBO_ALLOCATOR_NAME;
pub use self::constants::GL_MEMORY_VIDEO_EXT_FORMATS;
pub use self::constants::GL_RENDERBUFFER_ALLOCATOR_NAME;
pub use self::constants::GL_TEXTURE_TARGET_2D_STR;
pub use self::constants::GL_TEXTURE_TARGET_EXTERNAL_OES_STR;
pub use self::constants::GL_TEXTURE_TARGET_RECTANGLE_STR;

#[doc(hidden)]
pub mod traits {
    pub use super::gl_base_filter::GLBaseFilterExt;
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    pub use super::gl_base_src::GLBaseSrcExt;
    pub use super::gl_buffer_pool::GLBufferPoolExt;
    pub use super::gl_context::GLContextExt;
    pub use super::gl_display::GLDisplayExt;
    pub use super::gl_filter::GLFilterExt;
    pub use super::gl_framebuffer::GLFramebufferExt;
    pub use super::gl_window::GLWindowExt;
}
