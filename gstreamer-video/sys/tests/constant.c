// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GST_ANCILLARY_META_FIELD_INTERLACED_FIRST);
    PRINT_CONSTANT((gint) GST_ANCILLARY_META_FIELD_INTERLACED_SECOND);
    PRINT_CONSTANT((gint) GST_ANCILLARY_META_FIELD_PROGRESSIVE);
    PRINT_CONSTANT(GST_BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META);
    PRINT_CONSTANT(GST_BUFFER_POOL_OPTION_VIDEO_ALIGNMENT);
    PRINT_CONSTANT(GST_BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META);
    PRINT_CONSTANT(GST_BUFFER_POOL_OPTION_VIDEO_META);
    PRINT_CONSTANT(GST_CAPS_FEATURE_FORMAT_INTERLACED);
    PRINT_CONSTANT(GST_CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META);
    PRINT_CONSTANT(GST_CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META);
    PRINT_CONSTANT(GST_CAPS_FEATURE_META_GST_VIDEO_META);
    PRINT_CONSTANT(GST_CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION);
    PRINT_CONSTANT((gint) GST_COLOR_BALANCE_HARDWARE);
    PRINT_CONSTANT((gint) GST_COLOR_BALANCE_SOFTWARE);
    PRINT_CONSTANT(GST_META_TAG_VIDEO_COLORSPACE_STR);
    PRINT_CONSTANT(GST_META_TAG_VIDEO_ORIENTATION_STR);
    PRINT_CONSTANT(GST_META_TAG_VIDEO_SIZE_STR);
    PRINT_CONSTANT(GST_META_TAG_VIDEO_STR);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_ACTIVATE);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_DOWN);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_INVALID);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_LEFT);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_MENU1);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_MENU2);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_MENU3);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_MENU4);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_MENU5);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_MENU6);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_MENU7);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_NEXT_ANGLE);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_PREV_ANGLE);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_RIGHT);
    PRINT_CONSTANT((gint) GST_NAVIGATION_COMMAND_UP);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_COMMAND);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_INVALID);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_KEY_PRESS);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_KEY_RELEASE);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_MOUSE_BUTTON_PRESS);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_MOUSE_BUTTON_RELEASE);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_MOUSE_DOUBLE_CLICK);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_MOUSE_MOVE);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_MOUSE_SCROLL);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_TOUCH_CANCEL);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_TOUCH_DOWN);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_TOUCH_FRAME);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_TOUCH_MOTION);
    PRINT_CONSTANT((gint) GST_NAVIGATION_EVENT_TOUCH_UP);
    PRINT_CONSTANT((gint) GST_NAVIGATION_MESSAGE_ANGLES_CHANGED);
    PRINT_CONSTANT((gint) GST_NAVIGATION_MESSAGE_COMMANDS_CHANGED);
    PRINT_CONSTANT((gint) GST_NAVIGATION_MESSAGE_EVENT);
    PRINT_CONSTANT((gint) GST_NAVIGATION_MESSAGE_INVALID);
    PRINT_CONSTANT((gint) GST_NAVIGATION_MESSAGE_MOUSE_OVER);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_BUTTON1_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_BUTTON2_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_BUTTON3_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_BUTTON4_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_BUTTON5_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_CONTROL_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_HYPER_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_LOCK_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_META_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_MOD1_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_MOD2_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_MOD3_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_MOD4_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_MOD5_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_NONE);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_SHIFT_MASK);
    PRINT_CONSTANT((guint) GST_NAVIGATION_MODIFIER_SUPER_MASK);
    PRINT_CONSTANT((gint) GST_NAVIGATION_QUERY_ANGLES);
    PRINT_CONSTANT((gint) GST_NAVIGATION_QUERY_COMMANDS);
    PRINT_CONSTANT((gint) GST_NAVIGATION_QUERY_INVALID);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_14_9_LETTER_14_9_PILLAR);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_14_9_TOP_ALIGNED);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_16_9_LETTER_14_9_CENTER);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_16_9_LETTER_16_9_FULL);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_16_9_LETTER_4_3_CENTER);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_16_9_TOP_ALIGNED);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_4_3_FULL_14_9_CENTER);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_4_3_FULL_16_9_FULL);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_4_3_FULL_4_3_PILLAR);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_GREATER_THAN_16_9);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_SPEC_ATSC_A53);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_SPEC_DVB_ETSI);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_SPEC_SMPTE_ST2016_1);
    PRINT_CONSTANT((gint) GST_VIDEO_AFD_UNAVAILABLE);
    PRINT_CONSTANT((gint) GST_VIDEO_ALPHA_MODE_COPY);
    PRINT_CONSTANT((gint) GST_VIDEO_ALPHA_MODE_MULT);
    PRINT_CONSTANT((gint) GST_VIDEO_ALPHA_MODE_SET);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID16_S2016_3_AFD_BAR);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID16_S334_EIA_608);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID16_S334_EIA_708);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_CAMERA_POSITION);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_DELETION);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_3G_AUDIO_DATA_FIRST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_3G_AUDIO_DATA_LAST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_ERROR_DETECTION);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_HDTV_AUDIO_DATA_FIRST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_HDTV_AUDIO_DATA_LAST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_SDTV_AUDIO_DATA_1_FIRST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_SDTV_AUDIO_DATA_1_LAST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_SDTV_AUDIO_DATA_2_FIRST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_HANC_SDTV_AUDIO_DATA_2_LAST);
    PRINT_CONSTANT((gint) GST_VIDEO_ANCILLARY_DID_UNDEFINED);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_BOTTOM_FIELD);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_FIRST_IN_BUNDLE);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_INTERLACED);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_LAST);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_MARKER);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_MULTIPLE_VIEW);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_ONEFIELD);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_RFF);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_TFF);
    PRINT_CONSTANT((guint) GST_VIDEO_BUFFER_FLAG_TOP_FIELD);
    PRINT_CONSTANT((gint) GST_VIDEO_CAPTION_TYPE_CEA608_RAW);
    PRINT_CONSTANT((gint) GST_VIDEO_CAPTION_TYPE_CEA608_S334_1A);
    PRINT_CONSTANT((gint) GST_VIDEO_CAPTION_TYPE_CEA708_CDP);
    PRINT_CONSTANT((gint) GST_VIDEO_CAPTION_TYPE_CEA708_RAW);
    PRINT_CONSTANT((gint) GST_VIDEO_CAPTION_TYPE_UNKNOWN);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_FLAG_INTERLACED);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_FLAG_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_CHROMA_METHOD_LINEAR);
    PRINT_CONSTANT((gint) GST_VIDEO_CHROMA_METHOD_NEAREST);
    PRINT_CONSTANT((gint) GST_VIDEO_CHROMA_MODE_DOWNSAMPLE_ONLY);
    PRINT_CONSTANT((gint) GST_VIDEO_CHROMA_MODE_FULL);
    PRINT_CONSTANT((gint) GST_VIDEO_CHROMA_MODE_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_CHROMA_MODE_UPSAMPLE_ONLY);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_ALT_LINE);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_COSITED);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_DV);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_H_COSITED);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_JPEG);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_MPEG2);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_UNKNOWN);
    PRINT_CONSTANT((guint) GST_VIDEO_CHROMA_SITE_V_COSITED);
    PRINT_CONSTANT((guint) GST_VIDEO_CODEC_FRAME_FLAG_CORRUPTED);
    PRINT_CONSTANT((guint) GST_VIDEO_CODEC_FRAME_FLAG_DECODE_ONLY);
    PRINT_CONSTANT((guint) GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME);
    PRINT_CONSTANT((guint) GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME_HEADERS);
    PRINT_CONSTANT((guint) GST_VIDEO_CODEC_FRAME_FLAG_SYNC_POINT);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_BT2020);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_BT2020_10);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_BT2100_HLG);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_BT2100_PQ);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_BT601);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_BT709);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_SMPTE240M);
    PRINT_CONSTANT(GST_VIDEO_COLORIMETRY_SRGB);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_MATRIX_BT2020);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_MATRIX_BT601);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_MATRIX_BT709);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_MATRIX_FCC);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_MATRIX_RGB);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_MATRIX_SMPTE240M);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_MATRIX_UNKNOWN);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_ADOBERGB);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_BT2020);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_BT470BG);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_BT470M);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_BT709);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_EBU3213);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_FILM);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_SMPTE170M);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_SMPTE240M);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_SMPTEEG432);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_SMPTERP431);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_SMPTEST428);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_PRIMARIES_UNKNOWN);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_RANGE_0_255);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_RANGE_16_235);
    PRINT_CONSTANT((gint) GST_VIDEO_COLOR_RANGE_UNKNOWN);
    PRINT_CONSTANT(GST_VIDEO_COMP_A);
    PRINT_CONSTANT(GST_VIDEO_COMP_B);
    PRINT_CONSTANT(GST_VIDEO_COMP_G);
    PRINT_CONSTANT(GST_VIDEO_COMP_INDEX);
    PRINT_CONSTANT(GST_VIDEO_COMP_PALETTE);
    PRINT_CONSTANT(GST_VIDEO_COMP_R);
    PRINT_CONSTANT(GST_VIDEO_COMP_U);
    PRINT_CONSTANT(GST_VIDEO_COMP_V);
    PRINT_CONSTANT(GST_VIDEO_COMP_Y);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_ALPHA_MODE);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_ALPHA_VALUE);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_ASYNC_TASKS);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_BORDER_ARGB);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_CHROMA_MODE);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_CHROMA_RESAMPLER_METHOD);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_DEST_HEIGHT);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_DEST_WIDTH);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_DEST_X);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_DEST_Y);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_DITHER_METHOD);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_DITHER_QUANTIZATION);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_FILL_BORDER);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_GAMMA_MODE);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_MATRIX_MODE);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_PRIMARIES_MODE);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_RESAMPLER_METHOD);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_RESAMPLER_TAPS);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_SRC_HEIGHT);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_SRC_WIDTH);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_SRC_X);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_SRC_Y);
    PRINT_CONSTANT(GST_VIDEO_CONVERTER_OPT_THREADS);
    PRINT_CONSTANT(GST_VIDEO_DECODER_MAX_ERRORS);
    PRINT_CONSTANT((guint) GST_VIDEO_DECODER_REQUEST_SYNC_POINT_CORRUPT_OUTPUT);
    PRINT_CONSTANT((guint) GST_VIDEO_DECODER_REQUEST_SYNC_POINT_DISCARD_INPUT);
    PRINT_CONSTANT(GST_VIDEO_DECODER_SINK_NAME);
    PRINT_CONSTANT(GST_VIDEO_DECODER_SRC_NAME);
    PRINT_CONSTANT((gint) GST_VIDEO_DITHER_BAYER);
    PRINT_CONSTANT((guint) GST_VIDEO_DITHER_FLAG_INTERLACED);
    PRINT_CONSTANT((guint) GST_VIDEO_DITHER_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_DITHER_FLAG_QUANTIZE);
    PRINT_CONSTANT((gint) GST_VIDEO_DITHER_FLOYD_STEINBERG);
    PRINT_CONSTANT((gint) GST_VIDEO_DITHER_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_DITHER_SIERRA_LITE);
    PRINT_CONSTANT((gint) GST_VIDEO_DITHER_VERTERR);
    PRINT_CONSTANT(GST_VIDEO_ENCODER_SINK_NAME);
    PRINT_CONSTANT(GST_VIDEO_ENCODER_SRC_NAME);
    PRINT_CONSTANT((gint) GST_VIDEO_FIELD_ORDER_BOTTOM_FIELD_FIRST);
    PRINT_CONSTANT((gint) GST_VIDEO_FIELD_ORDER_TOP_FIELD_FIRST);
    PRINT_CONSTANT((gint) GST_VIDEO_FIELD_ORDER_UNKNOWN);
    PRINT_CONSTANT((guint) GST_VIDEO_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_FLAG_PREMULTIPLIED_ALPHA);
    PRINT_CONSTANT((guint) GST_VIDEO_FLAG_VARIABLE_FPS);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A420);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A420_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A420_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A420_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A420_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A420_16BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A420_16LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A422);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A422_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A422_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A422_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A422_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A422_16BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A422_16LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A444);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A444_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A444_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A444_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A444_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A444_16BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_A444_16LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ABGR);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ABGR64_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ABGR64_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ARGB);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ARGB64);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ARGB64_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ARGB64_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_AV12);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_AYUV);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_AYUV64);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGR);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGR10A2_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGR15);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGR16);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGRA);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGRA64_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGRA64_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGRP);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_BGRx);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_DMA_DRM);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_ENCODED);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_ALPHA);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_COMPLEX);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_GRAY);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_LE);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_PALETTE);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_RGB);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_SUBTILES);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_TILED);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_UNPACK);
    PRINT_CONSTANT((guint) GST_VIDEO_FORMAT_FLAG_YUV);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBR);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBRA);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBRA_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBRA_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBRA_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBRA_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBR_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBR_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBR_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBR_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBR_16BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GBR_16LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GRAY10_LE32);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GRAY16_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GRAY16_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_GRAY8);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I420);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I420_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I420_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I420_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I420_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I422_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I422_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I422_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_I422_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_IYU1);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_IYU2);
    PRINT_CONSTANT(GST_VIDEO_FORMAT_LAST);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_MT2110R);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_MT2110T);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_10BE_8L128);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_10LE32);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_10LE40);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_10LE40_4L4);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_16L32S);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_32L32);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_4L4);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_64Z32);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV12_8L128);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV16);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV16_10LE32);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV21);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV24);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_NV61);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_P010_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_P010_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_P012_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_P012_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_P016_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_P016_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RBGA);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGB);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGB10A2_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGB15);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGB16);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGB8P);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGBA);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGBA64_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGBA64_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGBP);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_RGBx);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_UNKNOWN);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_UYVP);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_UYVY);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_VUYA);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_VYUY);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y210);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y212_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y212_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y216_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y216_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y410);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y412_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y412_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y416_BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y416_LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y41B);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y42B);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y444);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y444_10BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y444_10LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y444_12BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y444_12LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y444_16BE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_Y444_16LE);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_YUV9);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_YUY2);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_YV12);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_YVU9);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_YVYU);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_r210);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_v210);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_v216);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_v308);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_xBGR);
    PRINT_CONSTANT((gint) GST_VIDEO_FORMAT_xRGB);
    PRINT_CONSTANT(GST_VIDEO_FPS_RANGE);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_BOTTOM_FIELD);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_FIRST_IN_BUNDLE);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_INTERLACED);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_MULTIPLE_VIEW);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_ONEFIELD);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_RFF);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_TFF);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_FLAG_TOP_FIELD);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_MAP_FLAG_LAST);
    PRINT_CONSTANT((guint) GST_VIDEO_FRAME_MAP_FLAG_NO_REF);
    PRINT_CONSTANT((gint) GST_VIDEO_GAMMA_MODE_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_GAMMA_MODE_REMAP);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_ORIENTATION_X_FLIP_Y_FLIP);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_ORIENTATION_X_FLIP_Y_NORMAL);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_ORIENTATION_X_NORMAL_Y_FLIP);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_ORIENTATION_X_NORMAL_Y_NORMAL);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_TYPE_LUMINANCE);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_TYPE_LUMINANCE_ALPHA);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_TYPE_R);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_TYPE_RG);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_TYPE_RGB);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_TYPE_RGB16);
    PRINT_CONSTANT((gint) GST_VIDEO_GL_TEXTURE_TYPE_RGBA);
    PRINT_CONSTANT((gint) GST_VIDEO_INTERLACE_MODE_ALTERNATE);
    PRINT_CONSTANT((gint) GST_VIDEO_INTERLACE_MODE_FIELDS);
    PRINT_CONSTANT((gint) GST_VIDEO_INTERLACE_MODE_INTERLEAVED);
    PRINT_CONSTANT((gint) GST_VIDEO_INTERLACE_MODE_MIXED);
    PRINT_CONSTANT((gint) GST_VIDEO_INTERLACE_MODE_PROGRESSIVE);
    PRINT_CONSTANT((gint) GST_VIDEO_MATRIX_MODE_FULL);
    PRINT_CONSTANT((gint) GST_VIDEO_MATRIX_MODE_INPUT_ONLY);
    PRINT_CONSTANT((gint) GST_VIDEO_MATRIX_MODE_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_MATRIX_MODE_OUTPUT_ONLY);
    PRINT_CONSTANT(GST_VIDEO_MAX_COMPONENTS);
    PRINT_CONSTANT(GST_VIDEO_MAX_PLANES);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_MIXED_MONO);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED);
    PRINT_CONSTANT((guint) GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_CHECKERBOARD);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_COLUMN_INTERLEAVED);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_LEFT);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_MONO);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_RIGHT);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_ROW_INTERLEAVED);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_SIDE_BY_SIDE);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_SIDE_BY_SIDE_QUINCUNX);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_FRAME_PACKING_TOP_BOTTOM);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_CHECKERBOARD);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_COLUMN_INTERLEAVED);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_FRAME_BY_FRAME);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_LEFT);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_MONO);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_MULTIVIEW_FRAME_BY_FRAME);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_RIGHT);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_ROW_INTERLEAVED);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_SEPARATED);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_SIDE_BY_SIDE);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_SIDE_BY_SIDE_QUINCUNX);
    PRINT_CONSTANT((gint) GST_VIDEO_MULTIVIEW_MODE_TOP_BOTTOM);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_180);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_90L);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_90R);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_AUTO);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_CUSTOM);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_HORIZ);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_IDENTITY);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_UL_LR);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_UR_LL);
    PRINT_CONSTANT((gint) GST_VIDEO_ORIENTATION_VERT);
    PRINT_CONSTANT((guint) GST_VIDEO_OVERLAY_FORMAT_FLAG_GLOBAL_ALPHA);
    PRINT_CONSTANT((guint) GST_VIDEO_OVERLAY_FORMAT_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_OVERLAY_FORMAT_FLAG_PREMULTIPLIED_ALPHA);
    PRINT_CONSTANT((guint) GST_VIDEO_PACK_FLAG_INTERLACED);
    PRINT_CONSTANT((guint) GST_VIDEO_PACK_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_PACK_FLAG_TRUNCATE_RANGE);
    PRINT_CONSTANT((gint) GST_VIDEO_PRIMARIES_MODE_FAST);
    PRINT_CONSTANT((gint) GST_VIDEO_PRIMARIES_MODE_MERGE_ONLY);
    PRINT_CONSTANT((gint) GST_VIDEO_PRIMARIES_MODE_NONE);
    PRINT_CONSTANT((guint) GST_VIDEO_RESAMPLER_FLAG_HALF_TAPS);
    PRINT_CONSTANT((guint) GST_VIDEO_RESAMPLER_FLAG_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_RESAMPLER_METHOD_CUBIC);
    PRINT_CONSTANT((gint) GST_VIDEO_RESAMPLER_METHOD_LANCZOS);
    PRINT_CONSTANT((gint) GST_VIDEO_RESAMPLER_METHOD_LINEAR);
    PRINT_CONSTANT((gint) GST_VIDEO_RESAMPLER_METHOD_NEAREST);
    PRINT_CONSTANT((gint) GST_VIDEO_RESAMPLER_METHOD_SINC);
    PRINT_CONSTANT(GST_VIDEO_RESAMPLER_OPT_CUBIC_B);
    PRINT_CONSTANT(GST_VIDEO_RESAMPLER_OPT_CUBIC_C);
    PRINT_CONSTANT(GST_VIDEO_RESAMPLER_OPT_ENVELOPE);
    PRINT_CONSTANT(GST_VIDEO_RESAMPLER_OPT_MAX_TAPS);
    PRINT_CONSTANT(GST_VIDEO_RESAMPLER_OPT_SHARPEN);
    PRINT_CONSTANT(GST_VIDEO_RESAMPLER_OPT_SHARPNESS);
    PRINT_CONSTANT((guint) GST_VIDEO_SCALER_FLAG_INTERLACED);
    PRINT_CONSTANT((guint) GST_VIDEO_SCALER_FLAG_NONE);
    PRINT_CONSTANT(GST_VIDEO_SCALER_OPT_DITHER_METHOD);
    PRINT_CONSTANT(GST_VIDEO_SIZE_RANGE);
    PRINT_CONSTANT((gint) GST_VIDEO_TILE_MODE_LINEAR);
    PRINT_CONSTANT((gint) GST_VIDEO_TILE_MODE_UNKNOWN);
    PRINT_CONSTANT((gint) GST_VIDEO_TILE_MODE_ZFLIPZ_2X2);
    PRINT_CONSTANT((gint) GST_VIDEO_TILE_TYPE_INDEXED);
    PRINT_CONSTANT(GST_VIDEO_TILE_TYPE_MASK);
    PRINT_CONSTANT(GST_VIDEO_TILE_TYPE_SHIFT);
    PRINT_CONSTANT(GST_VIDEO_TILE_X_TILES_MASK);
    PRINT_CONSTANT(GST_VIDEO_TILE_Y_TILES_SHIFT);
    PRINT_CONSTANT((guint) GST_VIDEO_TIME_CODE_FLAGS_DROP_FRAME);
    PRINT_CONSTANT((guint) GST_VIDEO_TIME_CODE_FLAGS_INTERLACED);
    PRINT_CONSTANT((guint) GST_VIDEO_TIME_CODE_FLAGS_NONE);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_ADOBERGB);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_ARIB_STD_B67);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_BT2020_10);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_BT2020_12);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_BT601);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_BT709);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_GAMMA10);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_GAMMA18);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_GAMMA20);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_GAMMA22);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_GAMMA28);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_LOG100);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_LOG316);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_SMPTE2084);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_SMPTE240M);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_SRGB);
    PRINT_CONSTANT((gint) GST_VIDEO_TRANSFER_UNKNOWN);
    PRINT_CONSTANT((gint) GST_VIDEO_VBI_PARSER_RESULT_DONE);
    PRINT_CONSTANT((gint) GST_VIDEO_VBI_PARSER_RESULT_ERROR);
    PRINT_CONSTANT((gint) GST_VIDEO_VBI_PARSER_RESULT_OK);
    return 0;
}
