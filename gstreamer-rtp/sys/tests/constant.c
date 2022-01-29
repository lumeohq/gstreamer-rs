// Generated by gir (https://github.com/gtk-rs/gir @ e0d8d8d645b1)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5502d32880f5)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ ec8a582cdebb)
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
    PRINT_CONSTANT((gint) GST_RTCP_FB_TYPE_INVALID);
    PRINT_CONSTANT(GST_RTCP_MAX_BYE_SSRC_COUNT);
    PRINT_CONSTANT(GST_RTCP_MAX_RB_COUNT);
    PRINT_CONSTANT(GST_RTCP_MAX_SDES);
    PRINT_CONSTANT(GST_RTCP_MAX_SDES_ITEM_COUNT);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_AFB);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_FIR);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_PLI);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_RPSI);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_SLI);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_TSTN);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_TSTR);
    PRINT_CONSTANT((gint) GST_RTCP_PSFB_TYPE_VBCN);
    PRINT_CONSTANT(GST_RTCP_REDUCED_SIZE_VALID_MASK);
    PRINT_CONSTANT((gint) GST_RTCP_RTPFB_TYPE_NACK);
    PRINT_CONSTANT((gint) GST_RTCP_RTPFB_TYPE_RTCP_SR_REQ);
    PRINT_CONSTANT((gint) GST_RTCP_RTPFB_TYPE_TMMBN);
    PRINT_CONSTANT((gint) GST_RTCP_RTPFB_TYPE_TMMBR);
    PRINT_CONSTANT((gint) GST_RTCP_RTPFB_TYPE_TWCC);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_APSI);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_CCID);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_CNAME);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_EMAIL);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_END);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_H323_CADDR);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_INVALID);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_LOC);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_MID);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_NAME);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_NOTE);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_PHONE);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_PRIV);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_REPAIRED_RTP_STREAM_ID);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_RGRP);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_RTP_STREAM_ID);
    PRINT_CONSTANT((gint) GST_RTCP_SDES_TOOL);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_APP);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_BYE);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_INVALID);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_PSFB);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_RR);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_RTPFB);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_SDES);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_SR);
    PRINT_CONSTANT((gint) GST_RTCP_TYPE_XR);
    PRINT_CONSTANT(GST_RTCP_VALID_MASK);
    PRINT_CONSTANT(GST_RTCP_VALID_VALUE);
    PRINT_CONSTANT(GST_RTCP_VERSION);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_DLRR);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_DRLE);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_INVALID);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_LRLE);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_PRT);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_RRT);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_SSUMM);
    PRINT_CONSTANT((gint) GST_RTCP_XR_TYPE_VOIP_METRICS);
    PRINT_CONSTANT((guint) GST_RTP_BUFFER_FLAG_LAST);
    PRINT_CONSTANT((guint) GST_RTP_BUFFER_FLAG_REDUNDANT);
    PRINT_CONSTANT((guint) GST_RTP_BUFFER_FLAG_RETRANSMISSION);
    PRINT_CONSTANT((guint) GST_RTP_BUFFER_MAP_FLAG_LAST);
    PRINT_CONSTANT((guint) GST_RTP_BUFFER_MAP_FLAG_SKIP_PADDING);
    PRINT_CONSTANT(GST_RTP_HDREXT_BASE);
    PRINT_CONSTANT(GST_RTP_HDREXT_ELEMENT_CLASS);
    PRINT_CONSTANT(GST_RTP_HDREXT_NTP_56);
    PRINT_CONSTANT(GST_RTP_HDREXT_NTP_56_SIZE);
    PRINT_CONSTANT(GST_RTP_HDREXT_NTP_64);
    PRINT_CONSTANT(GST_RTP_HDREXT_NTP_64_SIZE);
    PRINT_CONSTANT((guint) GST_RTP_HEADER_EXTENSION_DIRECTION_INACTIVE);
    PRINT_CONSTANT((guint) GST_RTP_HEADER_EXTENSION_DIRECTION_INHERITED);
    PRINT_CONSTANT((guint) GST_RTP_HEADER_EXTENSION_DIRECTION_RECVONLY);
    PRINT_CONSTANT((guint) GST_RTP_HEADER_EXTENSION_DIRECTION_SENDONLY);
    PRINT_CONSTANT((guint) GST_RTP_HEADER_EXTENSION_DIRECTION_SENDRECV);
    PRINT_CONSTANT((guint) GST_RTP_HEADER_EXTENSION_ONE_BYTE);
    PRINT_CONSTANT((guint) GST_RTP_HEADER_EXTENSION_TWO_BYTE);
    PRINT_CONSTANT(GST_RTP_HEADER_EXTENSION_URI_METADATA_KEY);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_1016);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_1016_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_CELLB);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_CELLB_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_CN);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_CN_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_DVI4_11025);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_DVI4_11025_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_DVI4_16000);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_DVI4_16000_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_DVI4_22050);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_DVI4_22050_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_DVI4_8000);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_DVI4_8000_STRING);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_DYNAMIC_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_G721);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G721_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_G722);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G722_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_G723);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G723_53);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G723_53_STRING);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G723_63);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G723_63_STRING);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G723_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_G728);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G728_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_G729);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_G729_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_GSM);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_GSM_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_H261);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_H261_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_H263);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_H263_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_JPEG);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_JPEG_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_L16_MONO);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_L16_MONO_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_L16_STEREO);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_L16_STEREO_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_LPC);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_LPC_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_MP2T);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_MP2T_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_MPA);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_MPA_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_MPV);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_MPV_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_NV);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_NV_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_PCMA);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_PCMA_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_PCMU);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_PCMU_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PAYLOAD_QCELP);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_QCELP_STRING);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_TS41);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_TS41_STRING);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_TS48);
    PRINT_CONSTANT(GST_RTP_PAYLOAD_TS48_STRING);
    PRINT_CONSTANT((gint) GST_RTP_PROFILE_AVP);
    PRINT_CONSTANT((gint) GST_RTP_PROFILE_AVPF);
    PRINT_CONSTANT((gint) GST_RTP_PROFILE_SAVP);
    PRINT_CONSTANT((gint) GST_RTP_PROFILE_SAVPF);
    PRINT_CONSTANT((gint) GST_RTP_PROFILE_UNKNOWN);
    PRINT_CONSTANT(GST_RTP_SOURCE_META_MAX_CSRC_COUNT);
    PRINT_CONSTANT(GST_RTP_VERSION);
    return 0;
}
