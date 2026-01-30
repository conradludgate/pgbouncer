pub mod _types_h {

    pub type __uint8_t = u8;

    pub type __uint16_t = u16;

    pub type __uint32_t = u32;

    pub type __darwin_size_t = usize;

    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod _in_addr_t_h {

    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}

pub mod _size_t_h {

    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _time_t_h {

    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}

pub mod _uint8_t_h {

    pub type uint8_t = u8;
}

pub mod _uint32_t_h {

    pub type uint32_t = u32;
}

pub mod _uint64_t_h {

    pub type uint64_t = u64;
}

pub mod tls_internal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct tls {
        pub config: *mut tls_config,
        pub error: tls_error,
        pub flags: uint32_t,
        pub state: uint32_t,
        pub servername: *mut ::core::ffi::c_char,
        pub socket: ::core::ffi::c_int,
        pub ssl_conn: *mut SSL,
        pub ssl_ctx: *mut SSL_CTX,
        pub ssl_peer_cert: *mut X509,
        pub conninfo: *mut tls_conninfo,
        pub used_dh_bits: ::core::ffi::c_int,
        pub used_ecdh_nid: ::core::ffi::c_int,
        pub ocsp_result: *const ::core::ffi::c_char,
        pub ocsp_info: *mut tls_ocsp_info,
        pub ocsp_query: *mut tls_ocsp_query,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct tls_ocsp_info {
        pub response_status: ::core::ffi::c_int,
        pub cert_status: ::core::ffi::c_int,
        pub crl_reason: ::core::ffi::c_int,
        pub this_update: time_t,
        pub next_update: time_t,
        pub revocation_time: time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct tls_conninfo {
        pub issuer: *mut ::core::ffi::c_char,
        pub subject: *mut ::core::ffi::c_char,
        pub hash: *mut ::core::ffi::c_char,
        pub serial: *mut ::core::ffi::c_char,
        pub fingerprint: *mut ::core::ffi::c_char,
        pub version: *mut ::core::ffi::c_char,
        pub cipher: *mut ::core::ffi::c_char,
        pub notbefore: time_t,
        pub notafter: time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct tls_error {
        pub msg: *mut ::core::ffi::c_char,
        pub num: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct tls_config {
        pub error: tls_error,
        pub ca_file: *const ::core::ffi::c_char,
        pub ca_path: *const ::core::ffi::c_char,
        pub ca_mem: *mut ::core::ffi::c_char,
        pub ca_len: size_t,
        pub ciphers: *const ::core::ffi::c_char,
        pub cipher_suites: *const ::core::ffi::c_char,
        pub ciphers_server: ::core::ffi::c_int,
        pub dheparams: ::core::ffi::c_int,
        pub ecdhecurve: ::core::ffi::c_int,
        pub keypair: *mut tls_keypair,
        pub ocsp_file: *const ::core::ffi::c_char,
        pub ocsp_mem: *mut ::core::ffi::c_char,
        pub ocsp_len: size_t,
        pub protocols: uint32_t,
        pub verify_cert: ::core::ffi::c_int,
        pub verify_client: ::core::ffi::c_int,
        pub verify_depth: ::core::ffi::c_int,
        pub verify_name: ::core::ffi::c_int,
        pub verify_time: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct tls_keypair {
        pub next: *mut tls_keypair,
        pub cert_file: *const ::core::ffi::c_char,
        pub cert_mem: *mut ::core::ffi::c_char,
        pub cert_len: size_t,
        pub key_file: *const ::core::ffi::c_char,
        pub key_mem: *mut ::core::ffi::c_char,
        pub key_len: size_t,
    }

    pub const TLS_SERVER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;

    pub const TLS_OCSP_CLIENT: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::tls_ocsp_query;
    use super::types_h::{ASN1_TIME, SSL, SSL_CTX, X509};
    extern "C" {

        pub fn tls_new() -> *mut tls;

        pub fn tls_set_error(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn tls_set_errorx(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn tls_set_error_libssl(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn tls_asn1_parse_time(
            ctx: *mut tls,
            asn1time: *const ASN1_TIME,
            dst: *mut time_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod types_h {

    pub type SSL_CTX = ssl_ctx_st;

    pub type X509 = x509_st;

    pub type OSSL_HTTP_REQ_CTX = ossl_http_req_ctx_st;

    pub type BIO = bio_st;

    pub type SSL = ssl_st;

    pub type OCSP_RESPONSE = ocsp_response_st;

    pub type ASN1_GENERALIZEDTIME = asn1_string_st;

    pub type ASN1_TIME = asn1_string_st;

    pub type X509_STORE_CTX = x509_store_ctx_st;

    pub type X509_OBJECT = x509_object_st;

    pub type EVP_MD = evp_md_st;

    pub type X509_NAME = X509_name_st;

    pub type X509_STORE = x509_store_st;

    pub type ASN1_ITEM = ASN1_ITEM_st;
    use super::asn1_h::asn1_string_st;
    extern "C" {

        pub type ssl_ctx_st;

        pub type x509_st;

        pub type ossl_http_req_ctx_st;

        pub type bio_st;

        pub type ssl_st;

        pub type ocsp_response_st;

        pub type x509_store_ctx_st;

        pub type x509_object_st;

        pub type evp_md_st;

        pub type X509_name_st;

        pub type x509_store_st;

        pub type ASN1_ITEM_st;
    }
}

pub mod x509_h {
    #[inline]

    pub unsafe extern "C" fn ossl_check_X509_sk_type(
        mut sk: *mut stack_st_X509,
    ) -> *mut OPENSSL_STACK {
        sk as *mut OPENSSL_STACK
    }
    use super::stack_h::OPENSSL_STACK;
    use super::types_h::{X509, X509_NAME};
    extern "C" {

        pub type stack_st_X509;

        pub fn X509_free(a: *mut X509);

        pub fn X509_get_issuer_name(a: *const X509) -> *mut X509_NAME;

        pub fn X509_find_by_subject(sk: *mut stack_st_X509, name: *const X509_NAME) -> *mut X509;
    }
}

pub mod ocsp_h {

    pub type OCSP_REQ_CTX = OSSL_HTTP_REQ_CTX;

    pub type OCSP_BASICRESP = ocsp_basic_response_st;

    pub type OCSP_CERTID = ocsp_cert_id_st;

    pub type OCSP_REQUEST = ocsp_request_st;

    pub type OCSP_ONEREQ = ocsp_one_request_st;

    pub const OCSP_TRUSTOTHER: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

    pub const OCSP_RESPONSE_STATUS_SUCCESSFUL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const V_OCSP_CERTSTATUS_GOOD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const V_OCSP_CERTSTATUS_REVOKED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const V_OCSP_CERTSTATUS_UNKNOWN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::types_h::{
        ASN1_GENERALIZEDTIME, ASN1_ITEM, BIO, EVP_MD, OCSP_RESPONSE, OSSL_HTTP_REQ_CTX, X509,
        X509_STORE,
    };
    use super::x509_h::stack_st_X509;
    extern "C" {

        pub type ocsp_basic_response_st;

        pub type ocsp_cert_id_st;

        pub type ocsp_request_st;

        pub type ocsp_one_request_st;

        pub fn OCSP_sendreq_new(
            io: *mut BIO,
            path: *const ::core::ffi::c_char,
            req: *const OCSP_REQUEST,
            buf_size: ::core::ffi::c_int,
        ) -> *mut OSSL_HTTP_REQ_CTX;

        pub fn OCSP_cert_to_id(
            dgst: *const EVP_MD,
            subject: *const X509,
            issuer: *const X509,
        ) -> *mut OCSP_CERTID;

        pub fn OCSP_request_add0_id(
            req: *mut OCSP_REQUEST,
            cid: *mut OCSP_CERTID,
        ) -> *mut OCSP_ONEREQ;

        pub fn OCSP_response_status(resp: *mut OCSP_RESPONSE) -> ::core::ffi::c_int;

        pub fn OCSP_response_get1_basic(resp: *mut OCSP_RESPONSE) -> *mut OCSP_BASICRESP;

        pub fn OCSP_resp_find_status(
            bs: *mut OCSP_BASICRESP,
            id: *mut OCSP_CERTID,
            status: *mut ::core::ffi::c_int,
            reason: *mut ::core::ffi::c_int,
            revtime: *mut *mut ASN1_GENERALIZEDTIME,
            thisupd: *mut *mut ASN1_GENERALIZEDTIME,
            nextupd: *mut *mut ASN1_GENERALIZEDTIME,
        ) -> ::core::ffi::c_int;

        pub fn OCSP_check_validity(
            thisupd: *mut ASN1_GENERALIZEDTIME,
            nextupd: *mut ASN1_GENERALIZEDTIME,
            sec: ::core::ffi::c_long,
            maxsec: ::core::ffi::c_long,
        ) -> ::core::ffi::c_int;

        pub fn OCSP_BASICRESP_free(a: *mut OCSP_BASICRESP);

        pub fn OCSP_RESPONSE_it() -> *const ASN1_ITEM;

        pub fn d2i_OCSP_RESPONSE(
            a: *mut *mut OCSP_RESPONSE,
            in_0: *mut *const ::core::ffi::c_uchar,
            len: ::core::ffi::c_long,
        ) -> *mut OCSP_RESPONSE;

        pub fn OCSP_RESPONSE_free(a: *mut OCSP_RESPONSE);

        pub fn i2d_OCSP_RESPONSE(
            a: *const OCSP_RESPONSE,
            out: *mut *mut ::core::ffi::c_uchar,
        ) -> ::core::ffi::c_int;

        pub fn OCSP_CERTID_free(a: *mut OCSP_CERTID);

        pub fn d2i_OCSP_REQUEST(
            a: *mut *mut OCSP_REQUEST,
            in_0: *mut *const ::core::ffi::c_uchar,
            len: ::core::ffi::c_long,
        ) -> *mut OCSP_REQUEST;

        pub fn OCSP_REQUEST_new() -> *mut OCSP_REQUEST;

        pub fn OCSP_REQUEST_free(a: *mut OCSP_REQUEST);

        pub fn i2d_OCSP_REQUEST(
            a: *const OCSP_REQUEST,
            out: *mut *mut ::core::ffi::c_uchar,
        ) -> ::core::ffi::c_int;

        pub fn OCSP_REQUEST_it() -> *const ASN1_ITEM;

        pub fn OCSP_response_status_str(s: ::core::ffi::c_long) -> *const ::core::ffi::c_char;

        pub fn OCSP_cert_status_str(s: ::core::ffi::c_long) -> *const ::core::ffi::c_char;

        pub fn OCSP_crl_reason_str(s: ::core::ffi::c_long) -> *const ::core::ffi::c_char;

        pub fn OCSP_basic_verify(
            bs: *mut OCSP_BASICRESP,
            certs: *mut stack_st_X509,
            st: *mut X509_STORE,
            flags: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int;
    }
}

pub mod poll_h {

    pub type nfds_t = ::core::ffi::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct pollfd {
        pub fd: ::core::ffi::c_int,
        pub events: ::core::ffi::c_short,
        pub revents: ::core::ffi::c_short,
    }

    pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

    pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    extern "C" {

        pub fn poll(_: *mut pollfd, _: nfds_t, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}

pub mod asn1_h {

    pub type i2d_of_void = unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct asn1_string_st {
        pub length: ::core::ffi::c_int,
        pub type_0: ::core::ffi::c_int,
        pub data: *mut ::core::ffi::c_uchar,
        pub flags: ::core::ffi::c_long,
    }

    pub type ASN1_VALUE = ASN1_VALUE_st;
    use super::types_h::BIO;
    extern "C" {

        pub type ASN1_VALUE_st;

        pub fn ASN1_i2d_bio(
            i2d: Option<i2d_of_void>,
            out: *mut BIO,
            x: *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod bio_h {

    pub type BIO_METHOD = bio_method_st;

    pub const BIO_CTRL_INFO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const BIO_FLAGS_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

    pub const BIO_FLAGS_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

    pub const BIO_FLAGS_SHOULD_RETRY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

    pub const BIO_C_SET_CONNECT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

    pub const BIO_C_DO_STATE_MACHINE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;

    pub const BIO_C_SET_NBIO: ::core::ffi::c_int = 102 as ::core::ffi::c_int;

    pub const BIO_C_GET_FD: ::core::ffi::c_int = 105 as ::core::ffi::c_int;

    pub const BIO_C_GET_SSL: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
    use super::types_h::BIO;
    extern "C" {

        pub type bio_method_st;

        pub fn BIO_test_flags(b: *const BIO, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn BIO_new(type_0: *const BIO_METHOD) -> *mut BIO;

        pub fn BIO_free(a: *mut BIO) -> ::core::ffi::c_int;

        pub fn BIO_ctrl(
            bp: *mut BIO,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;

        pub fn BIO_free_all(a: *mut BIO);

        pub fn BIO_s_mem() -> *const BIO_METHOD;

        pub fn BIO_s_connect() -> *const BIO_METHOD;
    }
}

pub mod stack_h {

    pub type OPENSSL_STACK = stack_st;
    extern "C" {

        pub type stack_st;

        pub fn OPENSSL_sk_value(
            _: *const OPENSSL_STACK,
            _: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;

        pub fn OPENSSL_sk_free(_: *mut OPENSSL_STACK);
    }
}

pub mod x509_vfy_h {

    pub type X509_LOOKUP_TYPE = ::core::ffi::c_uint;

    pub const X509_LU_CRL: X509_LOOKUP_TYPE = 2;

    pub const X509_LU_X509: X509_LOOKUP_TYPE = 1;

    pub const X509_LU_NONE: X509_LOOKUP_TYPE = 0;
    use super::types_h::{X509, X509_NAME, X509_OBJECT, X509_STORE, X509_STORE_CTX};
    use super::x509_h::stack_st_X509;
    extern "C" {

        pub fn X509_OBJECT_free(a: *mut X509_OBJECT);

        pub fn X509_OBJECT_get0_X509(a: *const X509_OBJECT) -> *mut X509;

        pub fn X509_STORE_CTX_new() -> *mut X509_STORE_CTX;

        pub fn X509_STORE_CTX_free(ctx: *mut X509_STORE_CTX);

        pub fn X509_STORE_CTX_init(
            ctx: *mut X509_STORE_CTX,
            trust_store: *mut X509_STORE,
            target: *mut X509,
            untrusted: *mut stack_st_X509,
        ) -> ::core::ffi::c_int;

        pub fn X509_STORE_CTX_get_obj_by_subject(
            vs: *mut X509_STORE_CTX,
            type_0: X509_LOOKUP_TYPE,
            name: *const X509_NAME,
        ) -> *mut X509_OBJECT;
    }
}

pub mod in6_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union C2RustUnnamed_0 {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
}

pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    use super::_in_addr_t_h::in_addr_t;
}

pub mod ssl_h {

    pub type SSL_METHOD = ssl_method_st;

    pub const SSL_OP_NO_SSLv3: uint64_t =
        (1 as ::core::ffi::c_int as uint64_t) << 25 as ::core::ffi::c_int as uint64_t;

    pub const SSL_OP_NO_TLSv1: uint64_t =
        (1 as ::core::ffi::c_int as uint64_t) << 26 as ::core::ffi::c_int as uint64_t;

    pub const SSL_OP_NO_TLSv1_2: uint64_t =
        (1 as ::core::ffi::c_int as uint64_t) << 27 as ::core::ffi::c_int as uint64_t;

    pub const SSL_OP_NO_TLSv1_1: uint64_t =
        (1 as ::core::ffi::c_int as uint64_t) << 28 as ::core::ffi::c_int as uint64_t;

    pub const SSL_OP_NO_TLSv1_3: uint64_t =
        (1 as ::core::ffi::c_int as uint64_t) << 29 as ::core::ffi::c_int as uint64_t;

    pub const SSL_OP_NO_SSLv2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const SSL_CTRL_MODE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;

    pub const SSL_CTRL_SET_TLSEXT_HOSTNAME: ::core::ffi::c_int = 55 as ::core::ffi::c_int;

    pub const SSL_CTRL_GET_TLSEXT_STATUS_REQ_OCSP_RESP: ::core::ffi::c_int =
        70 as ::core::ffi::c_int;

    pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_OCSP_RESP: ::core::ffi::c_int =
        71 as ::core::ffi::c_int;

    pub const SSL_CTRL_GET_EXTRA_CHAIN_CERTS: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
    use super::_uint64_t_h::uint64_t;
    use super::types_h::{BIO, SSL, SSL_CTX, X509, X509_STORE};
    use super::x509_h::stack_st_X509;
    extern "C" {

        pub type ssl_method_st;

        pub fn SSL_CTX_clear_options(ctx: *mut SSL_CTX, op: uint64_t) -> uint64_t;

        pub fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: uint64_t) -> uint64_t;

        pub fn BIO_new_ssl_connect(ctx: *mut SSL_CTX) -> *mut BIO;

        pub fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;

        pub fn SSL_CTX_free(_: *mut SSL_CTX);

        pub fn SSL_CTX_get_cert_store(_: *const SSL_CTX) -> *mut X509_STORE;

        pub fn SSL_get1_peer_certificate(s: *const SSL) -> *mut X509;

        pub fn SSL_get_peer_cert_chain(s: *const SSL) -> *mut stack_st_X509;

        pub fn SSL_ctrl(
            ssl: *mut SSL,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;

        pub fn SSL_CTX_ctrl(
            ctx: *mut SSL_CTX,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;

        pub fn TLS_client_method() -> *const SSL_METHOD;

        pub fn SSL_get_certificate(ssl: *const SSL) -> *mut X509;

        pub fn SSL_get_ex_data(
            ssl: *const SSL,
            idx: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
    }
}

pub mod safestack_h {
    #[inline]

    pub unsafe extern "C" fn ossl_check_const_OPENSSL_STRING_sk_type(
        mut sk: *const stack_st_OPENSSL_STRING,
    ) -> *const OPENSSL_STACK {
        sk as *const OPENSSL_STACK
    }
    use super::stack_h::OPENSSL_STACK;
    extern "C" {

        pub type stack_st_OPENSSL_STRING;
    }
}

pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;

        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;

        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod tls_h {

    pub const TLS_WANT_POLLIN: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);

    pub const TLS_WANT_POLLOUT: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);

    pub const TLS_NO_OCSP: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    use super::tls_internal_h::{tls, tls_config};
    extern "C" {

        pub fn tls_config_set_ocsp_stapling_mem(
            _config: *mut tls_config,
            _blob: *const uint8_t,
            _len: size_t,
        ) -> ::core::ffi::c_int;

        pub fn tls_configure(_ctx: *mut tls, _config: *mut tls_config) -> ::core::ffi::c_int;

        pub fn usual_tls_free(_ctx: *mut tls);

        pub fn tls_load_file(
            _file: *const ::core::ffi::c_char,
            _len: *mut size_t,
            _password: *mut ::core::ffi::c_char,
        ) -> *mut uint8_t;
    }
}

pub mod _null_h {

    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}

pub mod http_h {
    use super::asn1_h::ASN1_VALUE;
    use super::types_h::{ASN1_ITEM, OSSL_HTTP_REQ_CTX};
    extern "C" {

        pub fn OSSL_HTTP_REQ_CTX_free(rctx: *mut OSSL_HTTP_REQ_CTX);

        pub fn OSSL_HTTP_REQ_CTX_add1_header(
            rctx: *mut OSSL_HTTP_REQ_CTX,
            name: *const ::core::ffi::c_char,
            value: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn OSSL_HTTP_REQ_CTX_set1_req(
            rctx: *mut OSSL_HTTP_REQ_CTX,
            content_type: *const ::core::ffi::c_char,
            it: *const ASN1_ITEM,
            req: *const ASN1_VALUE,
        ) -> ::core::ffi::c_int;

        pub fn OSSL_HTTP_REQ_CTX_nbio_d2i(
            rctx: *mut OSSL_HTTP_REQ_CTX,
            pval: *mut *mut ASN1_VALUE,
            it: *const ASN1_ITEM,
        ) -> ::core::ffi::c_int;

        pub fn OSSL_HTTP_parse_url(
            url: *const ::core::ffi::c_char,
            pssl: *mut ::core::ffi::c_int,
            puser: *mut *mut ::core::ffi::c_char,
            phost: *mut *mut ::core::ffi::c_char,
            pport: *mut *mut ::core::ffi::c_char,
            pport_num: *mut ::core::ffi::c_int,
            ppath: *mut *mut ::core::ffi::c_char,
            pquery: *mut *mut ::core::ffi::c_char,
            pfrag: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}

pub mod x509v3_h {
    use super::safestack_h::stack_st_OPENSSL_STRING;
    use super::types_h::X509;
    extern "C" {

        pub fn X509_email_free(sk: *mut stack_st_OPENSSL_STRING);

        pub fn X509_get1_ocsp(x: *mut X509) -> *mut stack_st_OPENSSL_STRING;
    }
}

pub mod crypto_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn CRYPTO_malloc(
            num: size_t,
            file: *const ::core::ffi::c_char,
            line: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;

        pub fn CRYPTO_free(
            ptr: *mut ::core::ffi::c_void,
            file: *const ::core::ffi::c_char,
            line: ::core::ffi::c_int,
        );
    }
}

pub mod sys__types_h {

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod stdbool_h {

    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod socket_h {

    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
}

pub mod inet_h {
    extern "C" {

        pub fn inet_pton(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_char,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod tls1_h {

    pub const TLSEXT_NAMETYPE_host_name: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const SSL_TLSEXT_ERR_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const SSL_TLSEXT_ERR_ALERT_FATAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const SSL_TLSEXT_ERR_NOACK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
}
pub use self::_in_addr_t_h::in_addr_t;
use self::_malloc_h::{calloc, free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcpy, memset, strdup};
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::asn1_h::{asn1_string_st, i2d_of_void, ASN1_VALUE_st, ASN1_i2d_bio, ASN1_VALUE};
pub use self::bio_h::{
    bio_method_st, BIO_ctrl, BIO_free, BIO_free_all, BIO_new, BIO_s_connect, BIO_s_mem,
    BIO_test_flags, BIO_CTRL_INFO, BIO_C_DO_STATE_MACHINE, BIO_C_GET_FD, BIO_C_GET_SSL,
    BIO_C_SET_CONNECT, BIO_C_SET_NBIO, BIO_FLAGS_READ, BIO_FLAGS_SHOULD_RETRY, BIO_FLAGS_WRITE,
    BIO_METHOD,
};
use self::crypto_h::{CRYPTO_free, CRYPTO_malloc};
use self::http_h::{
    OSSL_HTTP_REQ_CTX_add1_header, OSSL_HTTP_REQ_CTX_free, OSSL_HTTP_REQ_CTX_nbio_d2i,
    OSSL_HTTP_REQ_CTX_set1_req, OSSL_HTTP_parse_url,
};
pub use self::in6_h::{in6_addr, C2RustUnnamed_0};
pub use self::in_h::in_addr;
use self::inet_h::inet_pton;
pub use self::ocsp_h::{
    d2i_OCSP_REQUEST, d2i_OCSP_RESPONSE, i2d_OCSP_REQUEST, i2d_OCSP_RESPONSE,
    ocsp_basic_response_st, ocsp_cert_id_st, ocsp_one_request_st, ocsp_request_st,
    OCSP_BASICRESP_free, OCSP_CERTID_free, OCSP_REQUEST_free, OCSP_REQUEST_it, OCSP_REQUEST_new,
    OCSP_RESPONSE_free, OCSP_RESPONSE_it, OCSP_basic_verify, OCSP_cert_status_str, OCSP_cert_to_id,
    OCSP_check_validity, OCSP_crl_reason_str, OCSP_request_add0_id, OCSP_resp_find_status,
    OCSP_response_get1_basic, OCSP_response_status, OCSP_response_status_str, OCSP_sendreq_new,
    OCSP_BASICRESP, OCSP_CERTID, OCSP_ONEREQ, OCSP_REQUEST, OCSP_REQ_CTX,
    OCSP_RESPONSE_STATUS_SUCCESSFUL, OCSP_TRUSTOTHER, V_OCSP_CERTSTATUS_GOOD,
    V_OCSP_CERTSTATUS_REVOKED, V_OCSP_CERTSTATUS_UNKNOWN,
};
pub use self::poll_h::{nfds_t, poll, pollfd, POLLIN, POLLOUT};
pub use self::safestack_h::{ossl_check_const_OPENSSL_STRING_sk_type, stack_st_OPENSSL_STRING};
pub use self::socket_h::{AF_INET, AF_INET6};
pub use self::ssl_h::{
    ssl_method_st, BIO_new_ssl_connect, SSL_CTX_clear_options, SSL_CTX_ctrl, SSL_CTX_free,
    SSL_CTX_get_cert_store, SSL_CTX_new, SSL_CTX_set_options, SSL_OP_NO_SSLv2, SSL_OP_NO_SSLv3,
    SSL_OP_NO_TLSv1, SSL_OP_NO_TLSv1_1, SSL_OP_NO_TLSv1_2, SSL_OP_NO_TLSv1_3, SSL_ctrl,
    SSL_get1_peer_certificate, SSL_get_certificate, SSL_get_ex_data, SSL_get_peer_cert_chain,
    TLS_client_method, SSL_CTRL_GET_EXTRA_CHAIN_CERTS, SSL_CTRL_GET_TLSEXT_STATUS_REQ_OCSP_RESP,
    SSL_CTRL_MODE, SSL_CTRL_SET_TLSEXT_HOSTNAME, SSL_CTRL_SET_TLSEXT_STATUS_REQ_OCSP_RESP,
    SSL_METHOD,
};
pub use self::stack_h::{stack_st, OPENSSL_sk_free, OPENSSL_sk_value, OPENSSL_STACK};
pub use self::stdbool_h::false_0;
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls1_h::{
    TLSEXT_NAMETYPE_host_name, SSL_TLSEXT_ERR_ALERT_FATAL, SSL_TLSEXT_ERR_NOACK, SSL_TLSEXT_ERR_OK,
};
pub use self::tls_h::{
    tls_config_set_ocsp_stapling_mem, tls_configure, tls_load_file, usual_tls_free, TLS_NO_OCSP,
    TLS_WANT_POLLIN, TLS_WANT_POLLOUT,
};
pub use self::tls_internal_h::{
    tls, tls_asn1_parse_time, tls_config, tls_conninfo, tls_error, tls_keypair, tls_new,
    tls_ocsp_info, tls_set_error, tls_set_error_libssl, tls_set_errorx, TLS_OCSP_CLIENT,
    TLS_SERVER,
};
pub use self::types_h::{
    bio_st, evp_md_st, ocsp_response_st, ossl_http_req_ctx_st, ssl_ctx_st, ssl_st, x509_object_st,
    x509_st, x509_store_ctx_st, x509_store_st, ASN1_ITEM_st, X509_name_st, ASN1_GENERALIZEDTIME,
    ASN1_ITEM, ASN1_TIME, BIO, EVP_MD, OCSP_RESPONSE, OSSL_HTTP_REQ_CTX, SSL, SSL_CTX, X509,
    X509_NAME, X509_OBJECT, X509_STORE, X509_STORE_CTX,
};
pub use self::x509_h::{
    ossl_check_X509_sk_type, stack_st_X509, X509_find_by_subject, X509_free, X509_get_issuer_name,
};
pub use self::x509_vfy_h::{
    X509_OBJECT_free, X509_OBJECT_get0_X509, X509_STORE_CTX_free,
    X509_STORE_CTX_get_obj_by_subject, X509_STORE_CTX_init, X509_STORE_CTX_new, X509_LOOKUP_TYPE,
    X509_LU_CRL, X509_LU_NONE, X509_LU_X509,
};
use self::x509v3_h::{X509_email_free, X509_get1_ocsp};
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tls_ocsp_query {
    pub ocsp_url: *mut ::core::ffi::c_char,
    pub request_data: *mut uint8_t,
    pub request_size: size_t,
    pub bio: *mut BIO,
    pub ssl_ctx: *mut SSL_CTX,
    pub http_req: *mut OCSP_REQ_CTX,
    pub main_cert: *mut X509,
    pub extra_certs: *mut stack_st_X509,
    pub cert_ssl_ctx: *mut SSL_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union C2RustUnnamed {
    pub ip4: in_addr,
    pub ip6: in6_addr,
}

pub const MAXAGE_SEC: ::core::ffi::c_int = 14 as ::core::ffi::c_int
    * 24 as ::core::ffi::c_int
    * 60 as ::core::ffi::c_int
    * 60 as ::core::ffi::c_int;

pub const JITTER_SEC: ::core::ffi::c_int = 60 as ::core::ffi::c_int;

unsafe extern "C" fn tls_ocsp_fill_info(
    mut ctx: *mut tls,
    mut response_status: ::core::ffi::c_int,
    mut cert_status: ::core::ffi::c_int,
    mut crl_reason: ::core::ffi::c_int,
    mut revtime: *mut ASN1_GENERALIZEDTIME,
    mut thisupd: *mut ASN1_GENERALIZEDTIME,
    mut nextupd: *mut ASN1_GENERALIZEDTIME,
) -> ::core::ffi::c_int {
    let mut info = ::core::ptr::null_mut::<tls_ocsp_info>();
    let mut res: ::core::ffi::c_int = 0;
    info = calloc(
        1 as size_t,
        ::core::mem::size_of::<tls_ocsp_info>() as size_t,
    ) as *mut tls_ocsp_info;
    if info.is_null() {
        tls_set_error(ctx, b"calloc\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    (*info).response_status = response_status;
    (*info).cert_status = cert_status;
    (*info).crl_reason = crl_reason;
    res = tls_asn1_parse_time(ctx, revtime, &raw mut (*info).revocation_time);
    if res == 0 as ::core::ffi::c_int {
        res = tls_asn1_parse_time(ctx, thisupd, &raw mut (*info).this_update);
    }
    if res == 0 as ::core::ffi::c_int {
        res = tls_asn1_parse_time(ctx, nextupd, &raw mut (*info).next_update);
    }
    if res == 0 as ::core::ffi::c_int {
        (*ctx).ocsp_info = info;
    } else {
        tls_ocsp_info_free(info);
    }
    res
}

unsafe extern "C" fn tls_ocsp_fill_result(mut ctx: *mut tls, mut res: ::core::ffi::c_int) {
    let mut info = (*ctx).ocsp_info;
    if res < 0 as ::core::ffi::c_int {
        (*ctx).ocsp_result = b"error\0" as *const u8 as *const ::core::ffi::c_char;
    } else if (*info).response_status != OCSP_RESPONSE_STATUS_SUCCESSFUL {
        (*ctx).ocsp_result =
            OCSP_response_status_str((*info).response_status as ::core::ffi::c_long);
    } else if (*info).cert_status != V_OCSP_CERTSTATUS_REVOKED {
        (*ctx).ocsp_result = OCSP_cert_status_str((*info).cert_status as ::core::ffi::c_long);
    } else {
        (*ctx).ocsp_result = OCSP_crl_reason_str((*info).crl_reason as ::core::ffi::c_long);
    };
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_info_free(mut info: *mut tls_ocsp_info) {
    free(info as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn tls_get_ocsp_info(
    mut ctx: *mut tls,
    mut response_status: *mut ::core::ffi::c_int,
    mut cert_status: *mut ::core::ffi::c_int,
    mut crl_reason: *mut ::core::ffi::c_int,
    mut this_update: *mut time_t,
    mut next_update: *mut time_t,
    mut revoction_time: *mut time_t,
    mut result_text: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut no_ocsp: tls_ocsp_info = tls_ocsp_info {
        response_status: -(1 as ::core::ffi::c_int),
        cert_status: -(1 as ::core::ffi::c_int),
        crl_reason: -(1 as ::core::ffi::c_int),
        this_update: 0 as time_t,
        next_update: 0 as time_t,
        revocation_time: 0 as time_t,
    };
    let mut info: *const tls_ocsp_info = (*ctx).ocsp_info;
    let mut ocsp_result = (*ctx).ocsp_result;
    let mut ret = 0 as ::core::ffi::c_int;
    if info.is_null() {
        info = &raw const no_ocsp;
        ret = -(1 as ::core::ffi::c_int);
    }
    if ocsp_result.is_null() {
        ret = TLS_NO_OCSP;
        ocsp_result = b"no-ocsp\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if !response_status.is_null() {
        *response_status = (*info).response_status;
    }
    if !cert_status.is_null() {
        *cert_status = (*info).cert_status;
    }
    if !crl_reason.is_null() {
        *crl_reason = (*info).crl_reason;
    }
    if !this_update.is_null() {
        *this_update = (*info).this_update;
    }
    if !next_update.is_null() {
        *next_update = (*info).next_update;
    }
    if !revoction_time.is_null() {
        *revoction_time = (*info).revocation_time;
    }
    if !result_text.is_null() {
        *result_text = ocsp_result;
    }
    ret
}

unsafe extern "C" fn tls_ocsp_get_certid(
    mut main_cert: *mut X509,
    mut extra_certs: *mut stack_st_X509,
    mut ssl_ctx: *mut SSL_CTX,
) -> *mut OCSP_CERTID {
    let mut issuer_name = ::core::ptr::null_mut::<X509_NAME>();
    let mut issuer = ::core::ptr::null_mut::<X509>();
    let mut storectx = ::core::ptr::null_mut::<X509_STORE_CTX>();
    let mut tmpobj = ::core::ptr::null_mut::<X509_OBJECT>();
    let mut cid = ::core::ptr::null_mut::<OCSP_CERTID>();
    let mut store = ::core::ptr::null_mut::<X509_STORE>();
    let mut ok: ::core::ffi::c_int = 0;
    issuer_name = X509_get_issuer_name(main_cert);
    if issuer_name.is_null() {
        return ::core::ptr::null_mut::<OCSP_CERTID>();
    }
    if !extra_certs.is_null() {
        issuer = X509_find_by_subject(extra_certs, issuer_name);
        if !issuer.is_null() {
            return OCSP_cert_to_id(::core::ptr::null::<EVP_MD>(), main_cert, issuer);
        }
    }
    store = SSL_CTX_get_cert_store(ssl_ctx);
    if !store.is_null() {
        storectx = X509_STORE_CTX_new();
        if !storectx.is_null() {
            ok = X509_STORE_CTX_init(storectx, store, main_cert, extra_certs);
            if ok == 1 as ::core::ffi::c_int {
                tmpobj = X509_STORE_CTX_get_obj_by_subject(storectx, X509_LU_X509, issuer_name);
                if !tmpobj.is_null() {
                    cid = OCSP_cert_to_id(
                        ::core::ptr::null::<EVP_MD>(),
                        main_cert,
                        X509_OBJECT_get0_X509(tmpobj),
                    );
                    X509_OBJECT_free(tmpobj);
                    X509_STORE_CTX_free(storectx);
                    return cid;
                }
            }
        }
    }
    if !storectx.is_null() {
        X509_STORE_CTX_free(storectx);
    }
    ::core::ptr::null_mut::<OCSP_CERTID>()
}

unsafe extern "C" fn tls_ocsp_verify_response(
    mut ctx: *mut tls,
    mut main_cert: *mut X509,
    mut extra_certs: *mut stack_st_X509,
    mut ssl_ctx: *mut SSL_CTX,
    mut resp: *mut OCSP_RESPONSE,
) -> ::core::ffi::c_int {
    let mut br = ::core::ptr::null_mut::<OCSP_BASICRESP>();
    let mut ocsp_chain = ::core::ptr::null_mut::<stack_st_X509>();
    let mut revtime = ::core::ptr::null_mut::<ASN1_GENERALIZEDTIME>();
    let mut thisupd = ::core::ptr::null_mut::<ASN1_GENERALIZEDTIME>();
    let mut nextupd = ::core::ptr::null_mut::<ASN1_GENERALIZEDTIME>();
    let mut cid = ::core::ptr::null_mut::<OCSP_CERTID>();
    let mut combined = ::core::ptr::null_mut::<stack_st_X509>();
    let mut response_status = 0 as ::core::ffi::c_int;
    let mut cert_status = 0 as ::core::ffi::c_int;
    let mut crl_reason = 0 as ::core::ffi::c_int;
    let mut ssl_res: ::core::ffi::c_int = 0;
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut flags: ::core::ffi::c_ulong = 0;
    br = OCSP_response_get1_basic(resp);
    if br.is_null() {
        tls_set_errorx(
            ctx,
            b"ocsp error: cannot load\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        flags = OCSP_TRUSTOTHER as ::core::ffi::c_ulong;
        ocsp_chain = extra_certs;
        ssl_res = OCSP_basic_verify(br, ocsp_chain, SSL_CTX_get_cert_store(ssl_ctx), flags);
        if ssl_res != 1 as ::core::ffi::c_int {
            tls_set_error_libssl(
                ctx,
                b"ocsp verify failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            response_status = OCSP_response_status(resp);
            if response_status != OCSP_RESPONSE_STATUS_SUCCESSFUL {
                tls_set_errorx(
                    ctx,
                    b"ocsp verify failed: unsuccessful response - %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    OCSP_response_status_str(response_status as ::core::ffi::c_long),
                );
            } else {
                cid = tls_ocsp_get_certid(main_cert, extra_certs, ssl_ctx);
                if cid.is_null() {
                    tls_set_errorx(
                        ctx,
                        b"ocsp verify failed: no issuer cert\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    ssl_res = OCSP_resp_find_status(
                        br,
                        cid,
                        &raw mut cert_status,
                        &raw mut crl_reason,
                        &raw mut revtime,
                        &raw mut thisupd,
                        &raw mut nextupd,
                    );
                    if ssl_res != 1 as ::core::ffi::c_int {
                        tls_set_errorx(
                            ctx,
                            b"ocsp verify failed: no result for cert\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        ssl_res = OCSP_check_validity(
                            thisupd,
                            nextupd,
                            JITTER_SEC as ::core::ffi::c_long,
                            MAXAGE_SEC as ::core::ffi::c_long,
                        );
                        if ssl_res != 1 as ::core::ffi::c_int {
                            tls_set_errorx(
                                ctx,
                                b"ocsp verify failed: bad age\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            ssl_res = tls_ocsp_fill_info(
                                ctx,
                                response_status,
                                cert_status,
                                crl_reason,
                                revtime,
                                thisupd,
                                nextupd,
                            );
                            if ssl_res == 0 as ::core::ffi::c_int {
                                if cert_status != V_OCSP_CERTSTATUS_GOOD
                                    && cert_status != V_OCSP_CERTSTATUS_UNKNOWN
                                {
                                    tls_set_errorx(
                                        ctx,
                                        b"ocsp verify failed: revoked cert - %s\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        OCSP_crl_reason_str(crl_reason as ::core::ffi::c_long),
                                    );
                                } else {
                                    ret = 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    OPENSSL_sk_free(ossl_check_X509_sk_type(combined));
    OCSP_CERTID_free(cid);
    OCSP_BASICRESP_free(br);
    ret
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_verify_callback(
    mut ssl: *mut SSL,
    mut _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut resp = ::core::ptr::null_mut::<OCSP_RESPONSE>();
    let mut extra_certs = ::core::ptr::null_mut::<stack_st_X509>();
    let mut peer = ::core::ptr::null_mut::<X509>();
    let mut raw = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut size: ::core::ffi::c_int = 0;
    let mut res = -(1 as ::core::ffi::c_int);
    let mut ctx = ::core::ptr::null_mut::<tls>();
    ctx = SSL_get_ex_data(ssl, 0 as ::core::ffi::c_int) as *mut tls;
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    size = SSL_ctrl(
        ssl,
        SSL_CTRL_GET_TLSEXT_STATUS_REQ_OCSP_RESP,
        0 as ::core::ffi::c_long,
        &raw mut raw as *mut ::core::ffi::c_void,
    ) as ::core::ffi::c_int;
    if size <= 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    peer = SSL_get1_peer_certificate(ssl);
    if peer.is_null() {
        tls_set_errorx(
            ctx,
            b"ocsp verify failed: no peer cert\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        resp = d2i_OCSP_RESPONSE(
            ::core::ptr::null_mut::<*mut OCSP_RESPONSE>(),
            &raw mut raw,
            size as ::core::ffi::c_long,
        );
        if resp.is_null() {
            tls_set_errorx(
                ctx,
                b"ocsp verify failed: parse failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            extra_certs = SSL_get_peer_cert_chain(ssl);
            res = tls_ocsp_verify_response(ctx, peer, extra_certs, (*ctx).ssl_ctx, resp);
        }
    }
    tls_ocsp_fill_result(ctx, res);
    OCSP_RESPONSE_free(resp);
    X509_free(peer);
    if res == 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_stapling_callback(
    mut ssl: *mut SSL,
    mut _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ctx = ::core::ptr::null_mut::<tls>();
    let mut mem = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fmem = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut xmem = ::core::ptr::null_mut::<uint8_t>();
    let mut len: size_t = 0;
    let mut ret = SSL_TLSEXT_ERR_ALERT_FATAL;
    ctx = SSL_get_ex_data(ssl, 0 as ::core::ffi::c_int) as *mut tls;
    if ctx.is_null() {
        return SSL_TLSEXT_ERR_NOACK;
    }
    if !(*(*ctx).config).ocsp_file.is_null() {
        mem = tls_load_file(
            (*(*ctx).config).ocsp_file,
            &raw mut len,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
        fmem = mem;
        if mem.is_null() {
            current_block = 17639227011838245702;
        } else {
            current_block = 13109137661213826276;
        }
    } else {
        mem = (*(*ctx).config).ocsp_mem;
        len = (*(*ctx).config).ocsp_len;
        if mem.is_null() {
            return SSL_TLSEXT_ERR_NOACK;
        }
        current_block = 13109137661213826276;
    }
    if current_block == 13109137661213826276 {
        xmem = CRYPTO_malloc(
            len,
            b"lib/usual/tls/tls_ocsp.c\0" as *const u8 as *const ::core::ffi::c_char,
            411 as ::core::ffi::c_int,
        ) as *mut uint8_t;
        if !xmem.is_null() {
            memcpy(
                xmem as *mut ::core::ffi::c_void,
                mem as *const ::core::ffi::c_void,
                len,
            );
            if SSL_ctrl(
                (*ctx).ssl_conn,
                SSL_CTRL_SET_TLSEXT_STATUS_REQ_OCSP_RESP,
                len as ::core::ffi::c_long,
                xmem as *mut ::core::ffi::c_void,
            ) != 1 as ::core::ffi::c_long
            {
                CRYPTO_free(
                    xmem as *mut ::core::ffi::c_void,
                    b"lib/usual/tls/tls_ocsp.c\0" as *const u8 as *const ::core::ffi::c_char,
                    415 as ::core::ffi::c_int,
                );
            } else {
                ret = SSL_TLSEXT_ERR_OK;
            }
        }
    }
    free(fmem as *mut ::core::ffi::c_void);
    ret
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_client_free(mut ctx: *mut tls) {
    let mut q = ::core::ptr::null_mut::<tls_ocsp_query>();
    if ctx.is_null() {
        return;
    }
    q = (*ctx).ocsp_query;
    if !q.is_null() {
        if !(*q).http_req.is_null() {
            OSSL_HTTP_REQ_CTX_free((*q).http_req as *mut OSSL_HTTP_REQ_CTX);
        }
        BIO_free_all((*q).bio);
        SSL_CTX_free((*q).ssl_ctx);
        free((*q).ocsp_url as *mut ::core::ffi::c_void);
        free((*q).request_data as *mut ::core::ffi::c_void);
        free(q as *mut ::core::ffi::c_void);
        (*ctx).ocsp_query = ::core::ptr::null_mut::<tls_ocsp_query>();
    }
}

unsafe extern "C" fn tls_ocsp_client_new() -> *mut tls {
    let mut ctx = ::core::ptr::null_mut::<tls>();
    ctx = tls_new();
    if ctx.is_null() {
        return ::core::ptr::null_mut::<tls>();
    }
    (*ctx).flags = TLS_OCSP_CLIENT as uint32_t;
    (*ctx).ocsp_query = calloc(
        1 as size_t,
        ::core::mem::size_of::<tls_ocsp_query>() as size_t,
    ) as *mut tls_ocsp_query;
    if (*ctx).ocsp_query.is_null() {
        usual_tls_free(ctx);
        return ::core::ptr::null_mut::<tls>();
    }
    ctx
}

unsafe extern "C" fn tls_build_ocsp_request(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut q = ::core::ptr::null_mut::<tls_ocsp_query>();
    let mut ok: ::core::ffi::c_int = 0;
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut req = ::core::ptr::null_mut::<OCSP_REQUEST>();
    let mut cid = ::core::ptr::null_mut::<OCSP_CERTID>();
    let mut onereq = ::core::ptr::null_mut::<OCSP_ONEREQ>();
    let mut mem = ::core::ptr::null_mut::<BIO>();
    let mut data = ::core::ptr::null_mut::<::core::ffi::c_void>();
    q = (*ctx).ocsp_query;
    cid = tls_ocsp_get_certid((*q).main_cert, (*q).extra_certs, (*q).cert_ssl_ctx);
    if cid.is_null() {
        tls_set_errorx(
            ctx,
            b"Cannot create cert-id\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        req = OCSP_REQUEST_new();
        if req.is_null() {
            tls_set_error_libssl(
                ctx,
                b"Cannot create request\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            onereq = OCSP_request_add0_id(req, cid);
            if onereq.is_null() {
                tls_set_error_libssl(
                    ctx,
                    b"Cannot add cert-id to request\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                cid = ::core::ptr::null_mut::<OCSP_CERTID>();
                mem = BIO_new(BIO_s_mem());
                if mem.is_null() {
                    tls_set_errorx(ctx, b"BIO_new\0" as *const u8 as *const ::core::ffi::c_char);
                } else {
                    ok = ASN1_i2d_bio(
                        ::core::mem::transmute::<
                            Option<
                                unsafe extern "C" fn(
                                    *const OCSP_REQUEST,
                                    *mut *mut ::core::ffi::c_uchar,
                                )
                                    -> ::core::ffi::c_int,
                            >,
                            Option<i2d_of_void>,
                        >(if 1 as ::core::ffi::c_int != 0 {
                            Some(
                                i2d_OCSP_REQUEST
                                    as unsafe extern "C" fn(
                                        *const OCSP_REQUEST,
                                        *mut *mut ::core::ffi::c_uchar,
                                    )
                                        -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        }),
                        mem,
                        (if 1 as ::core::ffi::c_int != 0 {
                            req as *const OCSP_REQUEST
                        } else {
                            ::core::ptr::null::<OCSP_REQUEST>()
                        }) as *mut ::core::ffi::c_void,
                    );
                    if ok == 0 {
                        tls_set_error_libssl(
                            ctx,
                            b"i2d_OCSP_RESPONSE_bio\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        (*q).request_size = BIO_ctrl(
                            mem,
                            BIO_CTRL_INFO,
                            0 as ::core::ffi::c_long,
                            &raw mut data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        ) as size_t;
                        (*q).request_data = malloc((*q).request_size) as *mut uint8_t;
                        if (*q).request_data.is_null() {
                            tls_set_error(
                                ctx,
                                b"Failed to allocate request data\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            memcpy(
                                (*q).request_data as *mut ::core::ffi::c_void,
                                data,
                                (*q).request_size,
                            );
                            req = ::core::ptr::null_mut::<OCSP_REQUEST>();
                            ret = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    OCSP_CERTID_free(cid);
    OCSP_REQUEST_free(req);
    BIO_free(mem);
    ret
}

unsafe extern "C" fn tls_ocsp_setup(
    mut ocsp_ctx_p: *mut *mut tls,
    mut config: *mut tls_config,
    mut target: *mut tls,
) -> ::core::ffi::c_int {
    let mut ctx = ::core::ptr::null_mut::<tls>();
    let mut q = ::core::ptr::null_mut::<tls_ocsp_query>();
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut ocsp_urls = ::core::ptr::null_mut::<stack_st_OPENSSL_STRING>();
    ctx = tls_ocsp_client_new();
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    *ocsp_ctx_p = ctx;
    q = (*ctx).ocsp_query;
    if !config.is_null() {
        (*ctx).flags = TLS_SERVER as uint32_t;
        ret = tls_configure(ctx, config);
        (*ctx).flags = TLS_OCSP_CLIENT as uint32_t;
        if ret != 0 as ::core::ffi::c_int {
            return ret;
        }
        (*q).main_cert = SSL_get_certificate((*ctx).ssl_conn);
        (*q).cert_ssl_ctx = (*ctx).ssl_ctx;
        SSL_CTX_ctrl(
            (*ctx).ssl_ctx,
            SSL_CTRL_GET_EXTRA_CHAIN_CERTS,
            0 as ::core::ffi::c_long,
            &raw mut (*q).extra_certs as *mut ::core::ffi::c_void,
        );
    } else {
        (*q).main_cert = SSL_get1_peer_certificate((*target).ssl_conn);
        (*q).extra_certs = SSL_get_peer_cert_chain((*target).ssl_conn);
        (*q).cert_ssl_ctx = (*target).ssl_ctx;
        X509_free((*q).main_cert);
    }
    if (*q).main_cert.is_null() {
        tls_set_errorx(ctx, b"No cert\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    ocsp_urls = X509_get1_ocsp((*q).main_cert);
    if ocsp_urls.is_null() {
        return TLS_NO_OCSP;
    }
    (*q).ocsp_url = strdup(OPENSSL_sk_value(
        ossl_check_const_OPENSSL_STRING_sk_type(ocsp_urls),
        0 as ::core::ffi::c_int,
    ) as *mut ::core::ffi::c_char);
    if (*q).ocsp_url.is_null() {
        tls_set_errorx(
            ctx,
            b"Cannot copy URL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        ret = tls_build_ocsp_request(ctx);
        if ret == 0 as ::core::ffi::c_int {
            *ocsp_ctx_p = ctx;
        }
    }
    X509_email_free(ocsp_urls);
    ret
}

unsafe extern "C" fn tls_ocsp_process_response_parsed(
    mut ctx: *mut tls,
    mut config: *mut tls_config,
    mut resp: *mut OCSP_RESPONSE,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q = (*ctx).ocsp_query;
    let mut mem = ::core::ptr::null_mut::<BIO>();
    let mut len: size_t = 0;
    let mut data = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut ok: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    res = tls_ocsp_verify_response(
        ctx,
        (*q).main_cert,
        (*q).extra_certs,
        (*q).cert_ssl_ctx,
        resp,
    );
    if res >= 0 as ::core::ffi::c_int {
        if !config.is_null() {
            mem = BIO_new(BIO_s_mem());
            if mem.is_null() {
                tls_set_error_libssl(ctx, b"BIO_new\0" as *const u8 as *const ::core::ffi::c_char);
                current_block = 12901458585090576971;
            } else {
                ok = ASN1_i2d_bio(
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *const OCSP_RESPONSE,
                                *mut *mut ::core::ffi::c_uchar,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<i2d_of_void>,
                    >(if 1 as ::core::ffi::c_int != 0 {
                        Some(
                            i2d_OCSP_RESPONSE
                                as unsafe extern "C" fn(
                                    *const OCSP_RESPONSE,
                                    *mut *mut ::core::ffi::c_uchar,
                                )
                                    -> ::core::ffi::c_int,
                        )
                    } else {
                        None
                    }),
                    mem,
                    (if 1 as ::core::ffi::c_int != 0 {
                        resp as *const OCSP_RESPONSE
                    } else {
                        ::core::ptr::null::<OCSP_RESPONSE>()
                    }) as *mut ::core::ffi::c_void,
                );
                if ok == 0 {
                    tls_set_error_libssl(
                        ctx,
                        b"i2d_OCSP_RESPONSE_bio\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    current_block = 12901458585090576971;
                } else {
                    len = BIO_ctrl(
                        mem,
                        BIO_CTRL_INFO,
                        0 as ::core::ffi::c_long,
                        &raw mut data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    ) as size_t;
                    res = tls_config_set_ocsp_stapling_mem(config, data, len);
                    if res < 0 as ::core::ffi::c_int {
                        current_block = 12901458585090576971;
                    } else {
                        current_block = 5399440093318478209;
                    }
                }
            }
        } else {
            current_block = 5399440093318478209;
        }
        match current_block {
            12901458585090576971 => {}
            _ => {
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    BIO_free(mem);
    tls_ocsp_fill_result(ctx, ret);
    ret
}

unsafe extern "C" fn tls_ocsp_create_request(
    mut ocsp_ctx_p: *mut *mut tls,
    mut config: *mut tls_config,
    mut target: *mut tls,
    mut ocsp_url: *mut *mut ::core::ffi::c_char,
    mut request_blob: *mut *mut ::core::ffi::c_void,
    mut request_size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    let mut q = ::core::ptr::null_mut::<tls_ocsp_query>();
    res = tls_ocsp_setup(ocsp_ctx_p, config, target);
    if res != 0 as ::core::ffi::c_int {
        return res;
    }
    q = (**ocsp_ctx_p).ocsp_query;
    *ocsp_url = (*q).ocsp_url;
    *request_blob = (*q).request_data as *mut ::core::ffi::c_void;
    *request_size = (*q).request_size;
    0 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_check_peer_request(
    mut ocsp_ctx_p: *mut *mut tls,
    mut target: *mut tls,
    mut ocsp_url: *mut *mut ::core::ffi::c_char,
    mut request_blob: *mut *mut ::core::ffi::c_void,
    mut request_size: *mut size_t,
) -> ::core::ffi::c_int {
    tls_ocsp_create_request(
        ocsp_ctx_p,
        ::core::ptr::null_mut::<tls_config>(),
        target,
        ocsp_url,
        request_blob,
        request_size,
    )
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_refresh_stapling_request(
    mut ocsp_ctx_p: *mut *mut tls,
    mut config: *mut tls_config,
    mut ocsp_url: *mut *mut ::core::ffi::c_char,
    mut request_blob: *mut *mut ::core::ffi::c_void,
    mut request_size: *mut size_t,
) -> ::core::ffi::c_int {
    tls_ocsp_create_request(
        ocsp_ctx_p,
        config,
        ::core::ptr::null_mut::<tls>(),
        ocsp_url,
        request_blob,
        request_size,
    )
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_process_response(
    mut ctx: *mut tls,
    mut response_blob: *const ::core::ffi::c_void,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut resp = ::core::ptr::null_mut::<OCSP_RESPONSE>();
    let mut raw = response_blob as *const ::core::ffi::c_uchar;
    resp = d2i_OCSP_RESPONSE(
        ::core::ptr::null_mut::<*mut OCSP_RESPONSE>(),
        &raw mut raw,
        size as ::core::ffi::c_long,
    );
    if resp.is_null() {
        (*ctx).ocsp_result = b"parse-failed\0" as *const u8 as *const ::core::ffi::c_char;
        tls_set_error_libssl(
            ctx,
            b"parse failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    ret = tls_ocsp_process_response_parsed(ctx, (*ctx).config, resp);
    OCSP_RESPONSE_free(resp);
    ret
}

unsafe extern "C" fn tls_ocsp_build_http_req(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut q = (*ctx).ocsp_query;
    let mut ok: ::core::ffi::c_int = 0;
    let mut req = ::core::ptr::null_mut::<OCSP_REQUEST>();
    let mut sreq = ::core::ptr::null_mut::<OCSP_REQ_CTX>();
    let mut data = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut https = 0 as ::core::ffi::c_int;
    let mut host = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut port = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ok = OSSL_HTTP_parse_url(
        (*q).ocsp_url,
        &raw mut https,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut host,
        &raw mut port,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut path,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if ok != 1 as ::core::ffi::c_int {
        tls_set_error_libssl(
            ctx,
            b"Cannot parse URL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        sreq = OCSP_sendreq_new(
            (*q).bio,
            path,
            ::core::ptr::null::<OCSP_REQUEST>(),
            -(1 as ::core::ffi::c_int),
        ) as *mut OCSP_REQ_CTX;
        if sreq.is_null() {
            tls_set_error_libssl(
                ctx,
                b"OCSP HTTP request setup failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            (*q).http_req = sreq;
            ok = OSSL_HTTP_REQ_CTX_add1_header(
                sreq as *mut OSSL_HTTP_REQ_CTX,
                b"Host\0" as *const u8 as *const ::core::ffi::c_char,
                host,
            );
            if ok != 0 {
                data = (*q).request_data;
                req = d2i_OCSP_REQUEST(
                    ::core::ptr::null_mut::<*mut OCSP_REQUEST>(),
                    &raw mut data,
                    (*q).request_size as ::core::ffi::c_long,
                );
                if !req.is_null() {
                    ok = OSSL_HTTP_REQ_CTX_set1_req(
                        sreq as *mut OSSL_HTTP_REQ_CTX,
                        b"application/ocsp-request\0" as *const u8 as *const ::core::ffi::c_char,
                        OCSP_REQUEST_it(),
                        req as *mut ASN1_VALUE,
                    );
                } else {
                    ok = false_0;
                }
                OCSP_REQUEST_free(req);
            }
            if ok != 0 {
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    free(host as *mut ::core::ffi::c_void);
    free(port as *mut ::core::ffi::c_void);
    free(path as *mut ::core::ffi::c_void);
    ret
}

unsafe extern "C" fn tls_ocsp_connection_setup(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ssl = ::core::ptr::null_mut::<SSL>();
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut ok: ::core::ffi::c_int = 0;
    let mut https = 0 as ::core::ffi::c_int;
    let mut q = (*ctx).ocsp_query;
    let mut addrbuf = C2RustUnnamed {
        ip4: in_addr { s_addr: 0 },
    };
    let mut host = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut port = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ok = OSSL_HTTP_parse_url(
        (*q).ocsp_url,
        &raw mut https,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut host,
        &raw mut port,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut path,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if ok != 1 as ::core::ffi::c_int {
        tls_set_error_libssl(
            ctx,
            b"Cannot parse URL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if https != 0 {
            (*q).ssl_ctx = SSL_CTX_new(TLS_client_method());
            if (*q).ssl_ctx.is_null() {
                tls_set_error_libssl(
                    ctx,
                    b"Cannot init SSL\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 4394515198446853333;
            } else {
                SSL_CTX_set_options((*q).ssl_ctx, SSL_OP_NO_SSLv2 as uint64_t | SSL_OP_NO_SSLv3);
                SSL_CTX_clear_options(
                    (*q).ssl_ctx,
                    SSL_OP_NO_TLSv1 | SSL_OP_NO_TLSv1_1 | SSL_OP_NO_TLSv1_2 | SSL_OP_NO_TLSv1_3,
                );
                SSL_CTX_ctrl(
                    (*q).ssl_ctx,
                    SSL_CTRL_MODE,
                    0x4 as ::core::ffi::c_long,
                    NULL,
                );
                (*q).bio = BIO_new_ssl_connect((*q).ssl_ctx);
                if !(*q).bio.is_null() {
                    if inet_pton(AF_INET, host, &raw mut addrbuf as *mut ::core::ffi::c_void)
                        != 1 as ::core::ffi::c_int
                        && inet_pton(AF_INET6, host, &raw mut addrbuf as *mut ::core::ffi::c_void)
                            != 1 as ::core::ffi::c_int
                    {
                        if BIO_ctrl(
                            (*q).bio,
                            BIO_C_GET_SSL,
                            0 as ::core::ffi::c_long,
                            &raw mut ssl as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        ) == 0
                        {
                            tls_set_errorx(
                                ctx,
                                b"cannot get ssl struct\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 4394515198446853333;
                        } else if SSL_ctrl(
                            ssl,
                            SSL_CTRL_SET_TLSEXT_HOSTNAME,
                            TLSEXT_NAMETYPE_host_name as ::core::ffi::c_long,
                            host as *mut ::core::ffi::c_void,
                        ) == 0 as ::core::ffi::c_long
                        {
                            tls_set_errorx(
                                ctx,
                                b"server name indication failure\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 4394515198446853333;
                        } else {
                            current_block = 13242334135786603907;
                        }
                    } else {
                        current_block = 13242334135786603907;
                    }
                } else {
                    current_block = 13242334135786603907;
                }
            }
        } else {
            (*q).bio = BIO_new(BIO_s_connect());
            current_block = 13242334135786603907;
        }
        match current_block {
            4394515198446853333 => {}
            _ => {
                if (*q).bio.is_null() {
                    tls_set_error_libssl(
                        ctx,
                        b"Cannot connect\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    BIO_ctrl(
                        (*q).bio,
                        BIO_C_SET_CONNECT,
                        0 as ::core::ffi::c_long,
                        host as *mut ::core::ffi::c_void,
                    );
                    BIO_ctrl(
                        (*q).bio,
                        BIO_C_SET_CONNECT,
                        1 as ::core::ffi::c_long,
                        port as *mut ::core::ffi::c_void,
                    );
                    BIO_ctrl((*q).bio, BIO_C_SET_NBIO, 1 as ::core::ffi::c_long, NULL);
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    free(host as *mut ::core::ffi::c_void);
    free(port as *mut ::core::ffi::c_void);
    free(path as *mut ::core::ffi::c_void);
    ret
}

unsafe extern "C" fn tls_ocsp_evloop(
    mut ctx: *mut tls,
    mut fd_p: *mut ::core::ffi::c_int,
    mut config: *mut tls_config,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q = (*ctx).ocsp_query;
    let mut ocsp_resp = ::core::ptr::null_mut::<OCSP_RESPONSE>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = 0;
    if (*q).http_req.is_null() {
        ok = BIO_ctrl(
            (*q).bio,
            BIO_C_DO_STATE_MACHINE,
            0 as ::core::ffi::c_long,
            NULL,
        ) as ::core::ffi::c_int;
        if ok != 1 as ::core::ffi::c_int && BIO_test_flags((*q).bio, BIO_FLAGS_SHOULD_RETRY) == 0 {
            tls_set_error_libssl(
                ctx,
                b"Connection failure\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 681489541387924420;
        } else {
            *fd_p = BIO_ctrl(
                (*q).bio,
                BIO_C_GET_FD,
                0 as ::core::ffi::c_long,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
            ) as ::core::ffi::c_int;
            if *fd_p < 0 as ::core::ffi::c_int {
                tls_set_error_libssl(
                    ctx,
                    b"Cannot get FD\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 681489541387924420;
            } else {
                if ok != 1 as ::core::ffi::c_int {
                    return TLS_WANT_POLLOUT;
                }
                ret = tls_ocsp_build_http_req(ctx);
                if ret != 0 as ::core::ffi::c_int {
                    current_block = 681489541387924420;
                } else {
                    current_block = 2868539653012386629;
                }
            }
        }
    } else {
        current_block = 2868539653012386629;
    }
    if current_block == 2868539653012386629 {
        ok = OSSL_HTTP_REQ_CTX_nbio_d2i(
            (*q).http_req as *mut OSSL_HTTP_REQ_CTX,
            &raw mut ocsp_resp as *mut *mut ASN1_VALUE,
            OCSP_RESPONSE_it(),
        );
        if ok == 1 as ::core::ffi::c_int {
            ret = tls_ocsp_process_response_parsed(ctx, config, ocsp_resp);
            return ret;
        } else if ok == 0 as ::core::ffi::c_int {
            tls_set_error_libssl(
                ctx,
                b"OCSP request failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            if BIO_test_flags((*q).bio, BIO_FLAGS_READ) != 0 {
                return TLS_WANT_POLLIN;
            } else if BIO_test_flags((*q).bio, BIO_FLAGS_WRITE) != 0 {
                return TLS_WANT_POLLOUT;
            }
            tls_set_error_libssl(
                ctx,
                b"Unexpected request error\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    tls_ocsp_fill_result(ctx, -(1 as ::core::ffi::c_int));
    -(1 as ::core::ffi::c_int)
}

unsafe extern "C" fn tls_ocsp_do_poll(
    mut ctx: *mut tls,
    mut errcode: ::core::ffi::c_int,
    mut fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut res: ::core::ffi::c_int = 0;
    memset(
        &raw mut pfd as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pollfd>() as size_t,
    );
    pfd.fd = fd;
    if errcode == TLS_WANT_POLLIN {
        pfd.events = POLLIN as ::core::ffi::c_short;
    } else if errcode == TLS_WANT_POLLOUT {
        pfd.events = POLLOUT as ::core::ffi::c_short;
    } else {
        tls_set_error(
            ctx,
            b"bad code to poll\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    res = poll(&raw mut pfd, 1 as nfds_t, -(1 as ::core::ffi::c_int));
    if res == 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    } else if res == 0 as ::core::ffi::c_int {
        tls_set_errorx(
            ctx,
            b"poll timed out\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    tls_set_error(
        ctx,
        b"poll error\0" as *const u8 as *const ::core::ffi::c_char,
    );
    -(1 as ::core::ffi::c_int)
}

unsafe extern "C" fn tls_ocsp_query_async(
    mut ocsp_ctx_p: *mut *mut tls,
    mut fd_p: *mut ::core::ffi::c_int,
    mut config: *mut tls_config,
    mut target: *mut tls,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ctx = *ocsp_ctx_p;
    let mut ret: ::core::ffi::c_int = 0;
    if ctx.is_null() {
        ret = tls_ocsp_setup(&raw mut ctx, config, target);
        if ret != 0 as ::core::ffi::c_int {
            current_block = 9055416853173976648;
        } else {
            ret = tls_ocsp_connection_setup(ctx);
            if ret != 0 as ::core::ffi::c_int {
                current_block = 9055416853173976648;
            } else {
                *ocsp_ctx_p = ctx;
                current_block = 6873731126896040597;
            }
        }
        match current_block {
            6873731126896040597 => {}
            _ => {
                usual_tls_free(ctx);
                return -(1 as ::core::ffi::c_int);
            }
        }
    }
    tls_ocsp_evloop(ctx, fd_p, config)
}

unsafe extern "C" fn tls_ocsp_common_query(
    mut ocsp_ctx_p: *mut *mut tls,
    mut fd_p: *mut ::core::ffi::c_int,
    mut config: *mut tls_config,
    mut target: *mut tls,
) -> ::core::ffi::c_int {
    let mut ctx = ::core::ptr::null_mut::<tls>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut fd = -(1 as ::core::ffi::c_int);
    if !fd_p.is_null() {
        return tls_ocsp_query_async(ocsp_ctx_p, fd_p, config, target);
    }
    loop {
        ret = tls_ocsp_query_async(&raw mut ctx, &raw mut fd, config, target);
        if ret != TLS_WANT_POLLIN && ret != TLS_WANT_POLLOUT {
            break;
        }
        ret = tls_ocsp_do_poll(ctx, ret, fd);
        if ret != 0 as ::core::ffi::c_int {
            break;
        }
    }
    *ocsp_ctx_p = ctx;
    ret
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_check_peer(
    mut ocsp_ctx_p: *mut *mut tls,
    mut async_fd_p: *mut ::core::ffi::c_int,
    mut target: *mut tls,
) -> ::core::ffi::c_int {
    tls_ocsp_common_query(
        ocsp_ctx_p,
        async_fd_p,
        ::core::ptr::null_mut::<tls_config>(),
        target,
    )
}
#[no_mangle]

pub unsafe extern "C" fn tls_ocsp_refresh_stapling(
    mut ocsp_ctx_p: *mut *mut tls,
    mut async_fd_p: *mut ::core::ffi::c_int,
    mut config: *mut tls_config,
) -> ::core::ffi::c_int {
    tls_ocsp_common_query(
        ocsp_ctx_p,
        async_fd_p,
        config,
        ::core::ptr::null_mut::<tls>(),
    )
}
