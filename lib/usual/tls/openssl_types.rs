//! Shared OpenSSL type definitions for TLS modules.
//!
//! This module consolidates OpenSSL types and functions, using openssl-sys
//! where available and providing extern declarations for the rest.

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

// =============================================================================
// OpenSSL Types - from openssl-sys
// =============================================================================

pub type SSL = openssl_sys::SSL;
pub type SSL_CTX = openssl_sys::SSL_CTX;
pub type SSL_METHOD = openssl_sys::SSL_METHOD;
pub type X509 = openssl_sys::X509;
pub type X509_STORE = openssl_sys::X509_STORE;
pub type X509_STORE_CTX = openssl_sys::X509_STORE_CTX;
pub type X509_VERIFY_PARAM = openssl_sys::X509_VERIFY_PARAM;
pub type X509_NAME = openssl_sys::X509_NAME;
pub type X509_NAME_ENTRY = openssl_sys::X509_NAME_ENTRY;
pub type X509_CRL = openssl_sys::X509_CRL;
pub type X509_EXTENSION = openssl_sys::X509_EXTENSION;
pub type EVP_PKEY = openssl_sys::EVP_PKEY;
pub type EVP_MD = openssl_sys::EVP_MD;
pub type EVP_MD_CTX = openssl_sys::EVP_MD_CTX;
pub type EVP_CIPHER_CTX = openssl_sys::EVP_CIPHER_CTX;
pub type BIO = openssl_sys::BIO;
pub type BIO_METHOD = openssl_sys::BIO_METHOD;
pub type DH = openssl_sys::DH;
pub type EC_KEY = openssl_sys::EC_KEY;
pub type RSA = openssl_sys::RSA;
pub type ASN1_TIME = openssl_sys::ASN1_TIME;
pub type ASN1_INTEGER = openssl_sys::ASN1_INTEGER;
pub type ASN1_OBJECT = openssl_sys::ASN1_OBJECT;
pub type ASN1_STRING = openssl_sys::ASN1_STRING;
pub type BIGNUM = openssl_sys::BIGNUM;
pub type OPENSSL_STACK = openssl_sys::OPENSSL_STACK;
pub type GENERAL_NAME = openssl_sys::GENERAL_NAME;
pub type OCSP_RESPONSE = openssl_sys::OCSP_RESPONSE;
pub type OCSP_BASICRESP = openssl_sys::OCSP_BASICRESP;
pub type OSSL_LIB_CTX = openssl_sys::OSSL_LIB_CTX;

// Opaque types not in openssl-sys
extern "C" {
    pub type X509_INFO;
    pub type X509_PUBKEY;
    pub type OCSP_CERTID;
    pub type OCSP_SINGLERESP;
    pub type stack_st;
}

// Type aliases for c2rust compatibility
pub type ssl_st = SSL;
pub type ssl_ctx_st = SSL_CTX;
pub type x509_st = X509;
pub type bio_st = BIO;
pub type evp_pkey_st = EVP_PKEY;
pub type x509_store_ctx_st = X509_STORE_CTX;
pub type X509_VERIFY_PARAM_st = X509_VERIFY_PARAM;
pub type evp_md_st = EVP_MD;
pub type evp_md_ctx_st = EVP_MD_CTX;

// Callback types
pub type pem_password_cb = unsafe extern "C" fn(
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

pub type SSL_verify_cb =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *mut X509_STORE_CTX) -> ::core::ffi::c_int>;

pub type OPENSSL_sk_freefunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;

// =============================================================================
// OpenSSL Constants
// =============================================================================

// SSL/TLS versions
pub use openssl_sys::TLS1_1_VERSION;
pub use openssl_sys::TLS1_2_VERSION;
pub use openssl_sys::TLS1_3_VERSION;
pub use openssl_sys::TLS1_VERSION;

// SSL error codes
pub const SSL_ERROR_NONE: ::core::ffi::c_int = openssl_sys::SSL_ERROR_NONE as _;
pub const SSL_ERROR_SSL: ::core::ffi::c_int = openssl_sys::SSL_ERROR_SSL as _;
pub const SSL_ERROR_WANT_READ: ::core::ffi::c_int = openssl_sys::SSL_ERROR_WANT_READ as _;
pub const SSL_ERROR_WANT_WRITE: ::core::ffi::c_int = openssl_sys::SSL_ERROR_WANT_WRITE as _;
pub const SSL_ERROR_WANT_X509_LOOKUP: ::core::ffi::c_int =
    openssl_sys::SSL_ERROR_WANT_X509_LOOKUP as _;
pub const SSL_ERROR_SYSCALL: ::core::ffi::c_int = openssl_sys::SSL_ERROR_SYSCALL as _;
pub const SSL_ERROR_ZERO_RETURN: ::core::ffi::c_int = openssl_sys::SSL_ERROR_ZERO_RETURN as _;
pub const SSL_ERROR_WANT_CONNECT: ::core::ffi::c_int = openssl_sys::SSL_ERROR_WANT_CONNECT as _;
pub const SSL_ERROR_WANT_ACCEPT: ::core::ffi::c_int = openssl_sys::SSL_ERROR_WANT_ACCEPT as _;

// SSL options
pub const SSL_OP_NO_SSLv2: ::core::ffi::c_int = 0; // Deprecated, no longer exists
pub const SSL_OP_NO_SSLv3: u64 = openssl_sys::SSL_OP_NO_SSLv3 as u64;
pub const SSL_OP_NO_TLSv1: u64 = openssl_sys::SSL_OP_NO_TLSv1 as u64;
pub const SSL_OP_NO_TLSv1_1: u64 = openssl_sys::SSL_OP_NO_TLSv1_1 as u64;
pub const SSL_OP_NO_TLSv1_2: u64 = openssl_sys::SSL_OP_NO_TLSv1_2 as u64;
pub const SSL_OP_NO_TLSv1_3: u64 = openssl_sys::SSL_OP_NO_TLSv1_3 as u64;

// SSL control commands
pub const SSL_CTRL_MODE: ::core::ffi::c_int = openssl_sys::SSL_CTRL_MODE as _;
pub const SSL_CTRL_SET_MIN_PROTO_VERSION: ::core::ffi::c_int =
    openssl_sys::SSL_CTRL_SET_MIN_PROTO_VERSION as _;
pub const SSL_CTRL_SET_MAX_PROTO_VERSION: ::core::ffi::c_int =
    openssl_sys::SSL_CTRL_SET_MAX_PROTO_VERSION as _;
pub const SSL_CTRL_SET_TMP_ECDH: ::core::ffi::c_int = openssl_sys::SSL_CTRL_SET_TMP_ECDH as _;
pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB: ::core::ffi::c_int =
    openssl_sys::SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB as _;

// SSL callback constants
pub const SSL_CB_HANDSHAKE_START: ::core::ffi::c_int = 0x10;

// SSL file types
pub const SSL_FILETYPE_PEM: ::core::ffi::c_int = openssl_sys::SSL_FILETYPE_PEM;

// X509 verification flags
pub const X509_V_OK: ::core::ffi::c_int = openssl_sys::X509_V_OK as _;
pub const X509_V_FLAG_CRL_CHECK: ::core::ffi::c_uint = openssl_sys::X509_V_FLAG_CRL_CHECK as _;
pub const X509_V_FLAG_CRL_CHECK_ALL: ::core::ffi::c_uint =
    openssl_sys::X509_V_FLAG_CRL_CHECK_ALL as _;

// X509 file type constant
pub const X509_FILETYPE_PEM: ::core::ffi::c_int = openssl_sys::X509_FILETYPE_PEM;

// NID constants
pub const NID_commonName: ::core::ffi::c_int = openssl_sys::NID_commonName;
pub const NID_subject_alt_name: ::core::ffi::c_int = openssl_sys::NID_subject_alt_name;

// GENERAL_NAME types
pub const GEN_OTHERNAME: ::core::ffi::c_int = openssl_sys::GEN_OTHERNAME;
pub const GEN_EMAIL: ::core::ffi::c_int = openssl_sys::GEN_EMAIL;
pub const GEN_DNS: ::core::ffi::c_int = openssl_sys::GEN_DNS;
pub const GEN_X400: ::core::ffi::c_int = openssl_sys::GEN_X400;
pub const GEN_DIRNAME: ::core::ffi::c_int = openssl_sys::GEN_DIRNAME;
pub const GEN_EDIPARTY: ::core::ffi::c_int = openssl_sys::GEN_EDIPARTY;
pub const GEN_URI: ::core::ffi::c_int = openssl_sys::GEN_URI;
pub const GEN_IPADD: ::core::ffi::c_int = openssl_sys::GEN_IPADD;
pub const GEN_RID: ::core::ffi::c_int = openssl_sys::GEN_RID;

// NPN negotiation
pub const OPENSSL_NPN_NEGOTIATED: ::core::ffi::c_int = openssl_sys::OPENSSL_NPN_NEGOTIATED;

// OpenSSL version
pub const OPENSSL_VERSION: ::core::ffi::c_int = 0;

// V_ASN1 types
pub const V_ASN1_IA5STRING: ::core::ffi::c_int = openssl_sys::V_ASN1_IA5STRING;
pub const V_ASN1_UTF8STRING: ::core::ffi::c_int = openssl_sys::V_ASN1_UTF8STRING;

// =============================================================================
// OpenSSL Functions - from openssl-sys
// =============================================================================

// SSL context functions
pub use openssl_sys::SSL_CTX_check_private_key;
pub use openssl_sys::SSL_CTX_clear_options;
pub use openssl_sys::SSL_CTX_ctrl;
pub use openssl_sys::SSL_CTX_free;
pub use openssl_sys::SSL_CTX_get0_param;
pub use openssl_sys::SSL_CTX_get_cert_store;
pub use openssl_sys::SSL_CTX_load_verify_locations;
pub use openssl_sys::SSL_CTX_new;
pub use openssl_sys::SSL_CTX_set_cipher_list;
pub use openssl_sys::SSL_CTX_set_ciphersuites;
pub use openssl_sys::SSL_CTX_set_options;
pub use openssl_sys::SSL_CTX_set_verify;
pub use openssl_sys::SSL_CTX_set_verify_depth;
pub use openssl_sys::SSL_CTX_use_PrivateKey;
pub use openssl_sys::SSL_CTX_use_PrivateKey_file;
pub use openssl_sys::SSL_CTX_use_certificate_chain_file;

// SSL functions
pub use openssl_sys::SSL_free;
pub use openssl_sys::SSL_get1_peer_certificate;
pub use openssl_sys::SSL_get_error;
pub use openssl_sys::SSL_get_ex_data;
pub use openssl_sys::SSL_new;
pub use openssl_sys::SSL_read;
pub use openssl_sys::SSL_set_connect_state;
pub use openssl_sys::SSL_shutdown;
pub use openssl_sys::SSL_version;
pub use openssl_sys::SSL_write;

// SSL method functions
pub use openssl_sys::TLS_client_method;
pub use openssl_sys::TLS_method;
pub use openssl_sys::TLS_server_method;

// X509 functions
pub use openssl_sys::X509_STORE_set_flags;
pub use openssl_sys::X509_VERIFY_PARAM_set_flags;
pub use openssl_sys::X509_check_host;
pub use openssl_sys::X509_check_ip_asc;
pub use openssl_sys::X509_free;
pub use openssl_sys::X509_get_ext_d2i;
pub use openssl_sys::X509_get_issuer_name;
pub use openssl_sys::X509_get_pubkey;
pub use openssl_sys::X509_get_serialNumber;
pub use openssl_sys::X509_get_subject_name;
pub use openssl_sys::X509_getm_notAfter;
pub use openssl_sys::X509_getm_notBefore;
pub use openssl_sys::X509_verify_cert_error_string;

// X509_NAME functions
pub use openssl_sys::X509_NAME_ENTRY_get_data;
pub use openssl_sys::X509_NAME_get_entry;
pub use openssl_sys::X509_NAME_get_index_by_NID;

// BIO functions
pub use openssl_sys::BIO_ctrl;
pub use openssl_sys::BIO_new;
pub use openssl_sys::BIO_new_mem_buf;
pub use openssl_sys::BIO_read;
pub use openssl_sys::BIO_s_mem;
pub use openssl_sys::BIO_write;

// EVP functions
pub use openssl_sys::EVP_DigestFinal_ex;
pub use openssl_sys::EVP_DigestInit;
pub use openssl_sys::EVP_DigestUpdate;
pub use openssl_sys::EVP_MD_CTX_free;
pub use openssl_sys::EVP_MD_CTX_new;
pub use openssl_sys::EVP_PKEY_free;
pub use openssl_sys::EVP_PKEY_get_bits;
pub use openssl_sys::EVP_md5;
pub use openssl_sys::EVP_sha256;

// ASN1 functions
pub use openssl_sys::ASN1_INTEGER_get;
pub use openssl_sys::ASN1_STRING_get0_data;
pub use openssl_sys::ASN1_STRING_length;
pub use openssl_sys::ASN1_STRING_type;
pub use openssl_sys::ASN1_TIME_print;

// PEM functions
pub use openssl_sys::PEM_read_bio_DHparams;
pub use openssl_sys::PEM_read_bio_PrivateKey;
pub use openssl_sys::PEM_read_bio_X509;

// ERR functions
pub use openssl_sys::ERR_clear_error;
pub use openssl_sys::ERR_get_error;
pub use openssl_sys::ERR_reason_error_string;

// BIGNUM functions
pub use openssl_sys::BN_bn2hex;
pub use openssl_sys::BN_free;

// DH functions
pub use openssl_sys::DH_free;

// EC functions
pub use openssl_sys::EC_KEY_free;
pub use openssl_sys::EC_KEY_new_by_curve_name;

// Stack functions
pub use openssl_sys::OPENSSL_sk_num;
pub use openssl_sys::OPENSSL_sk_pop_free;
pub use openssl_sys::OPENSSL_sk_value;

// Crypto functions
pub use openssl_sys::CRYPTO_free;
pub use openssl_sys::OpenSSL_version;

// OBJ functions
pub use openssl_sys::OBJ_nid2sn;
pub use openssl_sys::OBJ_obj2nid;

// OCSP functions
pub use openssl_sys::d2i_OCSP_RESPONSE;
pub use openssl_sys::OCSP_BASICRESP_free;
pub use openssl_sys::OCSP_basic_verify;
pub use openssl_sys::OCSP_check_validity;
pub use openssl_sys::OCSP_response_get1_basic;
pub use openssl_sys::OCSP_response_status;

// GENERAL_NAME functions
pub use openssl_sys::GENERAL_NAME_free;

// X509_STORE functions
pub use openssl_sys::X509_STORE_CTX_get_current_cert;
pub use openssl_sys::X509_STORE_CTX_get_error;
pub use openssl_sys::X509_STORE_CTX_get_error_depth;
pub use openssl_sys::X509_STORE_CTX_set_error;

// =============================================================================
// OpenSSL Functions - extern "C" for those not in openssl-sys
// =============================================================================

extern "C" {
    // BIO functions not in openssl-sys
    pub fn BIO_free(a: *mut BIO) -> ::core::ffi::c_int;

    // ERR functions not in openssl-sys
    pub fn ERR_peek_error() -> ::core::ffi::c_ulong;
    pub fn ERR_error_string(
        e: ::core::ffi::c_ulong,
        buf: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;

    // X509 functions not in openssl-sys
    pub fn X509_STORE_add_crl(ctx: *mut X509_STORE, x: *mut X509_CRL) -> ::core::ffi::c_int;
    pub fn X509_NAME_oneline(
        a: *const X509_NAME,
        buf: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    pub fn X509_NAME_print_ex(
        out: *mut BIO,
        nm: *const X509_NAME,
        indent: ::core::ffi::c_int,
        flags: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    // EVP functions not in openssl-sys
    pub fn EVP_PKEY_get_base_id(pkey: *const EVP_PKEY) -> ::core::ffi::c_int;

    // PEM functions not in openssl-sys
    pub fn PEM_X509_INFO_read_bio(
        bp: *mut BIO,
        sk: *mut OPENSSL_STACK,
        cb: Option<pem_password_cb>,
        u: *mut ::core::ffi::c_void,
    ) -> *mut OPENSSL_STACK;

    // X509_INFO functions not in openssl-sys
    pub fn X509_INFO_free(a: *mut X509_INFO);

    // OCSP functions not in openssl-sys
    pub fn OCSP_resp_get0(bs: *mut OCSP_BASICRESP, idx: ::core::ffi::c_int)
        -> *mut OCSP_SINGLERESP;
    pub fn OCSP_single_get0_status(
        single: *mut OCSP_SINGLERESP,
        reason: *mut ::core::ffi::c_int,
        revtime: *mut *mut ASN1_TIME,
        thisupd: *mut *mut ASN1_TIME,
        nextupd: *mut *mut ASN1_TIME,
    ) -> ::core::ffi::c_int;

    // OBJ functions not in openssl-sys
    pub fn OBJ_txt2nid(s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;

    // SSL_CTX functions not in openssl-sys
    pub fn SSL_CTX_set_info_callback(
        ctx: *mut SSL_CTX,
        cb: Option<unsafe extern "C" fn(*const SSL, ::core::ffi::c_int, ::core::ffi::c_int) -> ()>,
    );

    // SSL functions not in openssl-sys
    pub fn SSL_set_fd(s: *mut SSL, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;

    // Crypto functions not in openssl-sys
    pub fn OPENSSL_cleanup();
}
