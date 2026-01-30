
pub mod _types_h {
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _time_t_h {
    
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
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
    
    pub const TLS_SERVER_CONN: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    
    pub const TLS_HANDSHAKE_COMPLETE: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {
        
        pub type tls_ocsp_query;
        
        pub fn tls_new() -> *mut tls;
        
        pub fn tls_configure_keypair(
            ctx: *mut tls,
            ssl_ctx: *mut SSL_CTX,
            keypair: *mut tls_keypair,
            required: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_configure_ssl(ctx: *mut tls) -> ::core::ffi::c_int;
        
        pub fn tls_configure_ssl_verify(
            ctx: *mut tls,
            verify: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_set_errorx(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        
        pub fn tls_ssl_error(
            ctx: *mut tls,
            ssl_conn: *mut SSL,
            ssl_ret: ::core::ffi::c_int,
            prefix: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_ocsp_stapling_callback(
            ssl: *mut SSL,
            arg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod types_h {
    
    pub type X509 = x509_st;
    
    pub type SSL_CTX = ssl_ctx_st;
    
    pub type SSL = ssl_st;
    
    pub type EC_KEY = ec_key_st;
    extern "C" {
        
        pub type x509_st;
        
        pub type ssl_ctx_st;
        
        pub type ssl_st;
        
        pub type ec_key_st;
    }
}

pub mod x509_h {
    extern "C" {
        
        pub type stack_st_X509_NAME;
    }
}

pub mod ssl_h {
    
    pub type SSL_METHOD = ssl_method_st;
    
    pub type SSL_CTX_alpn_select_cb_func = Option<
        unsafe extern "C" fn(
            *mut SSL,
            *mut *const ::core::ffi::c_uchar,
            *mut ::core::ffi::c_uchar,
            *const ::core::ffi::c_uchar,
            ::core::ffi::c_uint,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >;
    
    pub const SSL_OP_SERVER_PREFERENCE: uint64_t =
        (1 as ::core::ffi::c_int as uint64_t) << 22 as ::core::ffi::c_int as uint64_t;
    
    pub const SSL_OP_CIPHER_SERVER_PREFERENCE: uint64_t = SSL_OP_SERVER_PREFERENCE;
    
    pub const SSL_OP_SINGLE_ECDH_USE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const OPENSSL_NPN_NEGOTIATED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const SSL_VERIFY_PEER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    pub const SSL_VERIFY_FAIL_IF_NO_PEER_CERT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    pub const SSL_CTRL_SET_TMP_ECDH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
    use super::_uint64_t_h::uint64_t;
    use super::types_h::{SSL, SSL_CTX};
    use super::x509_h::stack_st_X509_NAME;
    extern "C" {
        
        pub type ssl_method_st;
        
        pub fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: uint64_t) -> uint64_t;
        
        pub fn SSL_select_next_proto(
            out: *mut *mut ::core::ffi::c_uchar,
            outlen: *mut ::core::ffi::c_uchar,
            server: *const ::core::ffi::c_uchar,
            server_len: ::core::ffi::c_uint,
            client: *const ::core::ffi::c_uchar,
            client_len: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        
        pub fn SSL_CTX_set_alpn_select_cb(
            ctx: *mut SSL_CTX,
            cb: SSL_CTX_alpn_select_cb_func,
            arg: *mut ::core::ffi::c_void,
        );
        
        pub fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
        
        pub fn SSL_set_rfd(s: *mut SSL, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn SSL_set_wfd(s: *mut SSL, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn SSL_load_client_CA_file(file: *const ::core::ffi::c_char)
            -> *mut stack_st_X509_NAME;
        
        pub fn SSL_CTX_set_session_id_context(
            ctx: *mut SSL_CTX,
            sid_ctx: *const ::core::ffi::c_uchar,
            sid_ctx_len: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        
        pub fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
        
        pub fn SSL_accept(ssl: *mut SSL) -> ::core::ffi::c_int;
        
        pub fn SSL_CTX_ctrl(
            ctx: *mut SSL_CTX,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;
        
        pub fn SSL_CTX_callback_ctrl(
            _: *mut SSL_CTX,
            _: ::core::ffi::c_int,
            _: Option<unsafe extern "C" fn() -> ()>,
        ) -> ::core::ffi::c_long;
        
        pub fn TLS_server_method() -> *const SSL_METHOD;
        
        pub fn SSL_CTX_set_client_CA_list(ctx: *mut SSL_CTX, name_list: *mut stack_st_X509_NAME);
        
        pub fn SSL_set_ex_data(
            ssl: *mut SSL,
            idx: ::core::ffi::c_int,
            data: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod tls_h {
    use super::tls_internal_h::tls;
    extern "C" {
        
        pub fn usual_tls_free(_ctx: *mut tls);
    }
}

pub mod ec_h {
    use super::types_h::EC_KEY;
    extern "C" {
        
        pub fn EC_KEY_new_by_curve_name(nid: ::core::ffi::c_int) -> *mut EC_KEY;
        
        pub fn EC_KEY_free(key: *mut EC_KEY);
    }
}

pub mod tls_compat_h {
    use super::types_h::SSL_CTX;
    extern "C" {
        
        pub fn SSL_CTX_set_dh_auto(
            ctx: *mut SSL_CTX,
            onoff: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        
        pub fn SSL_CTX_set_ecdh_auto(
            ctx: *mut SSL_CTX,
            onoff: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
    }
}

pub mod sys__types_h {
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod obj_mac_h {
    
    pub const NID_undef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod tls1_h {
    
    pub const SSL_TLSEXT_ERR_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const SSL_TLSEXT_ERR_ALERT_FATAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const SSL_TLSEXT_ERR_NOACK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
}

pub mod err_h {
    extern "C" {
        
        pub fn ERR_clear_error();
    }
}

pub mod rand_h {
    extern "C" {
        
        pub fn RAND_bytes(
            buf: *mut ::core::ffi::c_uchar,
            num: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
use self::ec_h::{EC_KEY_free, EC_KEY_new_by_curve_name};
use self::err_h::ERR_clear_error;
pub use self::obj_mac_h::NID_undef;
use self::rand_h::RAND_bytes;
pub use self::ssl_h::{
    ssl_method_st, SSL_CTX_alpn_select_cb_func, SSL_CTX_callback_ctrl, SSL_CTX_ctrl, SSL_CTX_new,
    SSL_CTX_set_alpn_select_cb, SSL_CTX_set_client_CA_list, SSL_CTX_set_options,
    SSL_CTX_set_session_id_context, SSL_accept, SSL_load_client_CA_file, SSL_new,
    SSL_select_next_proto, SSL_set_ex_data, SSL_set_rfd, SSL_set_wfd, TLS_server_method,
    OPENSSL_NPN_NEGOTIATED, SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB, SSL_CTRL_SET_TMP_ECDH, SSL_METHOD,
    SSL_OP_CIPHER_SERVER_PREFERENCE, SSL_OP_SERVER_PREFERENCE, SSL_OP_SINGLE_ECDH_USE,
    SSL_VERIFY_FAIL_IF_NO_PEER_CERT, SSL_VERIFY_PEER,
};
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls1_h::{SSL_TLSEXT_ERR_ALERT_FATAL, SSL_TLSEXT_ERR_NOACK, SSL_TLSEXT_ERR_OK};
use self::tls_compat_h::{SSL_CTX_set_dh_auto, SSL_CTX_set_ecdh_auto};
use self::tls_h::usual_tls_free;
pub use self::tls_internal_h::{
    tls, tls_config, tls_configure_keypair, tls_configure_ssl, tls_configure_ssl_verify,
    tls_conninfo, tls_error, tls_keypair, tls_new, tls_ocsp_info, tls_ocsp_query,
    tls_ocsp_stapling_callback, tls_set_errorx, tls_ssl_error, TLS_HANDSHAKE_COMPLETE, TLS_SERVER,
    TLS_SERVER_CONN,
};
pub use self::types_h::{ec_key_st, ssl_ctx_st, ssl_st, x509_st, EC_KEY, SSL, SSL_CTX, X509};
use self::x509_h::stack_st_X509_NAME;
#[no_mangle]

pub unsafe extern "C" fn tls_server() -> *mut tls {
    let mut ctx = ::core::ptr::null_mut::<tls>();
    ctx = tls_new();
    if ctx.is_null() {
        return ::core::ptr::null_mut::<tls>();
    }
    (*ctx).flags |= TLS_SERVER as uint32_t;
    ctx
}
#[no_mangle]

pub unsafe extern "C" fn tls_server_conn(mut _ctx: *mut tls) -> *mut tls {
    let mut conn_ctx = ::core::ptr::null_mut::<tls>();
    conn_ctx = tls_new();
    if conn_ctx.is_null() {
        return ::core::ptr::null_mut::<tls>();
    }
    (*conn_ctx).flags |= TLS_SERVER_CONN as uint32_t;
    conn_ctx
}

static mut alpn_protos: [::core::ffi::c_uchar; 11] = [
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    'p' as i32 as ::core::ffi::c_uchar,
    'o' as i32 as ::core::ffi::c_uchar,
    's' as i32 as ::core::ffi::c_uchar,
    't' as i32 as ::core::ffi::c_uchar,
    'g' as i32 as ::core::ffi::c_uchar,
    'r' as i32 as ::core::ffi::c_uchar,
    'e' as i32 as ::core::ffi::c_uchar,
    's' as i32 as ::core::ffi::c_uchar,
    'q' as i32 as ::core::ffi::c_uchar,
    'l' as i32 as ::core::ffi::c_uchar,
];

unsafe extern "C" fn alpn_cb(
    mut _ssl: *mut SSL,
    mut out: *mut *const ::core::ffi::c_uchar,
    mut outlen: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
    mut _userdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    retval = SSL_select_next_proto(
        out as *mut *mut ::core::ffi::c_uchar,
        outlen,
        &raw const alpn_protos as *const ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 11]>() as ::core::ffi::c_uint,
        in_0,
        inlen,
    );
    if (*out).is_null()
        || *outlen as usize > ::core::mem::size_of::<[::core::ffi::c_uchar; 11]>()
        || *outlen as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
    {
        return SSL_TLSEXT_ERR_NOACK;
    }
    if retval == OPENSSL_NPN_NEGOTIATED {
        SSL_TLSEXT_ERR_OK
    } else {
        SSL_TLSEXT_ERR_ALERT_FATAL
    }
}
#[no_mangle]

pub unsafe extern "C" fn tls_configure_server(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ecdh_key = ::core::ptr::null_mut::<EC_KEY>();
    let mut cert_stack = ::core::ptr::null_mut::<stack_st_X509_NAME>();
    let mut sid: [::core::ffi::c_uchar; 32] = [0; 32];
    (*ctx).ssl_ctx = SSL_CTX_new(TLS_server_method());
    if (*ctx).ssl_ctx.is_null() {
        tls_set_errorx(
            ctx,
            b"ssl context failure\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        SSL_CTX_set_alpn_select_cb(
            (*ctx).ssl_ctx,
            Some(
                alpn_cb
                    as unsafe extern "C" fn(
                        *mut SSL,
                        *mut *const ::core::ffi::c_uchar,
                        *mut ::core::ffi::c_uchar,
                        *const ::core::ffi::c_uchar,
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
        );
        if (tls_configure_ssl(ctx) == 0 as ::core::ffi::c_int)
            && (tls_configure_keypair(
                ctx,
                (*ctx).ssl_ctx,
                (*(*ctx).config).keypair,
                1 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int)
        {
            if (*(*ctx).config).verify_client != 0 as ::core::ffi::c_int {
                let mut verify = SSL_VERIFY_PEER;
                if (*(*ctx).config).verify_client == 1 as ::core::ffi::c_int {
                    verify |= SSL_VERIFY_FAIL_IF_NO_PEER_CERT;
                }
                if tls_configure_ssl_verify(ctx, verify) == -(1 as ::core::ffi::c_int) {
                    current_block = 10176218473768484732;
                } else {
                    current_block = 11812396948646013369;
                }
            } else {
                current_block = 11812396948646013369;
            }
            match current_block {
                10176218473768484732 => {}
                _ => {
                    if (*(*ctx).config).dheparams == -(1 as ::core::ffi::c_int) {
                        SSL_CTX_set_dh_auto((*ctx).ssl_ctx, 1 as ::core::ffi::c_int);
                    }
                    if (*(*ctx).config).ecdhecurve == -(1 as ::core::ffi::c_int) {
                        SSL_CTX_set_ecdh_auto((*ctx).ssl_ctx, 1 as ::core::ffi::c_int);
                        current_block = 13242334135786603907;
                    } else if (*(*ctx).config).ecdhecurve != NID_undef {
                        ecdh_key = EC_KEY_new_by_curve_name((*(*ctx).config).ecdhecurve);
                        if ecdh_key.is_null() {
                            tls_set_errorx(
                                ctx,
                                b"failed to set ECDHE curve\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 10176218473768484732;
                        } else {
                            SSL_CTX_set_options((*ctx).ssl_ctx, SSL_OP_SINGLE_ECDH_USE as uint64_t);
                            SSL_CTX_ctrl(
                                (*ctx).ssl_ctx,
                                SSL_CTRL_SET_TMP_ECDH,
                                0 as ::core::ffi::c_long,
                                ecdh_key as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                            );
                            EC_KEY_free(ecdh_key);
                            current_block = 13242334135786603907;
                        }
                    } else {
                        current_block = 13242334135786603907;
                    }
                    match current_block {
                        10176218473768484732 => {}
                        _ => {
                            if (*(*ctx).config).ciphers_server == 1 as ::core::ffi::c_int {
                                SSL_CTX_set_options(
                                    (*ctx).ssl_ctx,
                                    SSL_OP_CIPHER_SERVER_PREFERENCE,
                                );
                            }
                            if SSL_CTX_callback_ctrl(
                                (*ctx).ssl_ctx,
                                SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB,
                                ::core::mem::transmute::<
                                    Option<
                                        unsafe extern "C" fn(
                                            *mut SSL,
                                            *mut ::core::ffi::c_void,
                                        )
                                            -> ::core::ffi::c_int,
                                    >,
                                    Option<unsafe extern "C" fn() -> ()>,
                                >(Some(
                                    tls_ocsp_stapling_callback
                                        as unsafe extern "C" fn(
                                            *mut SSL,
                                            *mut ::core::ffi::c_void,
                                        )
                                            -> ::core::ffi::c_int,
                                )),
                            ) != 1 as ::core::ffi::c_long
                            {
                                tls_set_errorx(
                                    ctx,
                                    b"ssl OCSP stapling setup failure\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            } else if RAND_bytes(
                                &raw mut sid as *mut ::core::ffi::c_uchar,
                                ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>()
                                    as ::core::ffi::c_int,
                            ) == 0
                            {
                                tls_set_errorx(
                                    ctx,
                                    b"failed to generate session id\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            } else if SSL_CTX_set_session_id_context(
                                (*ctx).ssl_ctx,
                                &raw mut sid as *mut ::core::ffi::c_uchar,
                                ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>()
                                    as ::core::ffi::c_uint,
                            ) == 0
                            {
                                tls_set_errorx(
                                    ctx,
                                    b"failed to set session id context\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                cert_stack = SSL_load_client_CA_file((*(*ctx).config).ca_file);
                                SSL_CTX_set_client_CA_list((*ctx).ssl_ctx, cert_stack);
                                return 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]

pub unsafe extern "C" fn tls_accept_socket(
    mut ctx: *mut tls,
    mut cctx: *mut *mut tls,
    mut socket: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    tls_accept_fds(ctx, cctx, socket, socket)
}
#[no_mangle]

pub unsafe extern "C" fn tls_accept_fds(
    mut ctx: *mut tls,
    mut cctx: *mut *mut tls,
    mut fd_read: ::core::ffi::c_int,
    mut fd_write: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut conn_ctx = ::core::ptr::null_mut::<tls>();
    if (*ctx).flags & TLS_SERVER as uint32_t == 0 as uint32_t {
        tls_set_errorx(
            ctx,
            b"not a server context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        conn_ctx = tls_server_conn(ctx);
        if conn_ctx.is_null() {
            tls_set_errorx(
                ctx,
                b"connection context failure\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            (*conn_ctx).ssl_conn = SSL_new((*ctx).ssl_ctx);
            if (*conn_ctx).ssl_conn.is_null() {
                tls_set_errorx(
                    ctx,
                    b"ssl failure\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if SSL_set_ex_data(
                (*conn_ctx).ssl_conn,
                0 as ::core::ffi::c_int,
                conn_ctx as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ) != 1 as ::core::ffi::c_int
            {
                tls_set_errorx(
                    ctx,
                    b"ssl application data failure\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if SSL_set_rfd((*conn_ctx).ssl_conn, fd_read) != 1 as ::core::ffi::c_int
                || SSL_set_wfd((*conn_ctx).ssl_conn, fd_write) != 1 as ::core::ffi::c_int
            {
                tls_set_errorx(
                    ctx,
                    b"ssl file descriptor failure\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                *cctx = conn_ctx;
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    usual_tls_free(conn_ctx);
    *cctx = ::core::ptr::null_mut::<tls>();
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]

pub unsafe extern "C" fn tls_handshake_server(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut ssl_ret: ::core::ffi::c_int = 0;
    let mut rv = -(1 as ::core::ffi::c_int);
    if (*ctx).flags & TLS_SERVER_CONN as uint32_t == 0 as uint32_t {
        tls_set_errorx(
            ctx,
            b"not a server connection context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        ERR_clear_error();
        ssl_ret = SSL_accept((*ctx).ssl_conn);
        if ssl_ret != 1 as ::core::ffi::c_int {
            rv = tls_ssl_error(
                ctx,
                (*ctx).ssl_conn,
                ssl_ret,
                b"handshake\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            (*ctx).state |= TLS_HANDSHAKE_COMPLETE as uint32_t;
            rv = 0 as ::core::ffi::c_int;
        }
    }
    rv
}
