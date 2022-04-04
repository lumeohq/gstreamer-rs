// Generated by gir (https://github.com/gtk-rs/gir @ 9aa16ead87e1)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3ff4d3275258)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 233e3205cb17)
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
    PRINT_CONSTANT((gint) GST_MIKEY_CACHE_ALWAYS);
    PRINT_CONSTANT((gint) GST_MIKEY_CACHE_FOR_CSB);
    PRINT_CONSTANT((gint) GST_MIKEY_CACHE_NONE);
    PRINT_CONSTANT((gint) GST_MIKEY_ENC_AES_CM_128);
    PRINT_CONSTANT((gint) GST_MIKEY_ENC_AES_GCM_128);
    PRINT_CONSTANT((gint) GST_MIKEY_ENC_AES_KW_128);
    PRINT_CONSTANT((gint) GST_MIKEY_ENC_NULL);
    PRINT_CONSTANT((gint) GST_MIKEY_KD_TEK);
    PRINT_CONSTANT((gint) GST_MIKEY_KD_TGK);
    PRINT_CONSTANT((gint) GST_MIKEY_KV_INTERVAL);
    PRINT_CONSTANT((gint) GST_MIKEY_KV_NULL);
    PRINT_CONSTANT((gint) GST_MIKEY_KV_SPI);
    PRINT_CONSTANT((gint) GST_MIKEY_MAC_HMAC_SHA_1_160);
    PRINT_CONSTANT((gint) GST_MIKEY_MAC_NULL);
    PRINT_CONSTANT((gint) GST_MIKEY_MAP_TYPE_SRTP);
    PRINT_CONSTANT((gint) GST_MIKEY_PRF_MIKEY_1);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_CERT);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_CHASH);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_DH);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_ERR);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_GEN_EXT);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_ID);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_KEMAC);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_KEY_DATA);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_LAST);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_PKE);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_RAND);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_SIGN);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_SP);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_T);
    PRINT_CONSTANT((gint) GST_MIKEY_PT_V);
    PRINT_CONSTANT((gint) GST_MIKEY_SEC_PROTO_SRTP);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_AEAD_AUTH_TAG_LEN);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_AUTH_ALG);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_AUTH_KEY_LEN);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_AUTH_TAG_LEN);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_ENC_ALG);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_ENC_KEY_LEN);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_FEC_ORDER);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_KEY_DERIV_RATE);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_PRF);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_SALT_KEY_LEN);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_SRTCP_ENC);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_SRTP_AUTH);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_SRTP_ENC);
    PRINT_CONSTANT((gint) GST_MIKEY_SP_SRTP_SRTP_PREFIX_LEN);
    PRINT_CONSTANT((gint) GST_MIKEY_TS_TYPE_COUNTER);
    PRINT_CONSTANT((gint) GST_MIKEY_TS_TYPE_NTP);
    PRINT_CONSTANT((gint) GST_MIKEY_TS_TYPE_NTP_UTC);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_DH_INIT);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_DH_RESP);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_ERROR);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_INVALID);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_PK_INIT);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_PK_VERIFY);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_PSK_INIT);
    PRINT_CONSTANT((gint) GST_MIKEY_TYPE_PSK_VERIFY);
    PRINT_CONSTANT(GST_MIKEY_VERSION);
    PRINT_CONSTANT(GST_SDP_BWTYPE_AS);
    PRINT_CONSTANT(GST_SDP_BWTYPE_CT);
    PRINT_CONSTANT(GST_SDP_BWTYPE_EXT_PREFIX);
    PRINT_CONSTANT(GST_SDP_BWTYPE_RR);
    PRINT_CONSTANT(GST_SDP_BWTYPE_RS);
    PRINT_CONSTANT(GST_SDP_BWTYPE_TIAS);
    PRINT_CONSTANT((gint) GST_SDP_EINVAL);
    PRINT_CONSTANT((gint) GST_SDP_OK);
    return 0;
}
