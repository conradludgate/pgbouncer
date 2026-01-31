pub mod bouncer_h {
    pub use crate::types::*;
    pub use c2rust_bitfields::BitfieldStruct;
    extern "C" {}
}

pub mod sbuf_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;






}

pub mod iobuf_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;






}

pub mod pktbuf_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;






}

pub mod dnslookup_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;
    extern "C" {
    }






}

pub mod messages_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgParsePacket {
        pub len: ::core::ffi::c_uint,
        pub name: *const ::core::ffi::c_char,
        pub query_and_parameters_len: size_t,
        pub query_and_parameters: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgBindPacket {
        pub len: ::core::ffi::c_uint,
        pub portal: *const ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgDescribePacket {
        pub type_0: ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgClosePacket {
        pub type_0: ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }

    pub type PreparedStatementAction = ::core::ffi::c_uint;

    pub const PS_INSPECT_FAILED: PreparedStatementAction = 3;

    pub const PS_HANDLE_FULL_PACKET: PreparedStatementAction = 2;

    pub const PS_HANDLE: PreparedStatementAction = 1;

    pub const PS_IGNORE: PreparedStatementAction = 0;
    use crate::types::size_t;
}

pub mod objects_h {

    pub use crate::types::*;
    extern "C" {

        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
    }


}

use crate::types::strlen;
pub use self::bouncer_h::{
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use crate::types::in_addr_t;
pub use crate::types::in_port_t;
pub use crate::types::pid_t;
pub use crate::types::ptrdiff_t;
pub use crate::types::sa_family_t;
pub use crate::types::size_t;
pub use crate::types::ssize_t;
pub use crate::types::timeval;
pub use crate::types::uid_t;
pub use crate::types::uint16_t;
pub use crate::types::uint32_t;
pub use crate::types::uint64_t;
pub use crate::types::uint8_t;
pub use crate::types::uintptr_t;
pub use crate::types::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint16_t,
    __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use crate::types::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::messages_h::{
    PgBindPacket, PgClosePacket, PgDescribePacket, PgParsePacket, PreparedStatementAction,
    PS_HANDLE, PS_HANDLE_FULL_PACKET, PS_IGNORE, PS_INSPECT_FAILED,
};
use self::objects_h::disconnect_client;
pub use self::pktbuf_h::PktBuf;
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use crate::types::sockaddr;
pub use crate::types::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t};
pub use crate::lib::usual::mbuf::{
    mbuf_avail_for_read, mbuf_get_bytes, mbuf_get_char, mbuf_get_string, mbuf_get_uint16be,
    mbuf_written,
};
pub use crate::types::usec_t;
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{incomplete_pkt, PktHdr};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[no_mangle]

pub unsafe extern "C" fn inspect_parse_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> PreparedStatementAction {
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut statement) {
        return PS_INSPECT_FAILED;
    }
    if *statement == 0 {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"inspect_parse_packet: type=%c, len=%d, statement=<empty>".as_ptr(),
                (*pkt).type_0,
                (*pkt).len,
            );
        }
        return PS_IGNORE;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            c"inspect_parse_packet: type=%c, len=%d, statement=%s".as_ptr(),
            (*pkt).type_0,
            (*pkt).len,
            statement,
        );
    }
    PS_HANDLE_FULL_PACKET
}
#[no_mangle]

pub unsafe extern "C" fn inspect_bind_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> PreparedStatementAction {
    let mut portal = ::core::ptr::null::<::core::ffi::c_char>();
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut portal) {
        return PS_INSPECT_FAILED;
    }
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut statement) {
        return PS_INSPECT_FAILED;
    }
    if *statement == 0 {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"inspect_bind_packet: type=%c, len=%d, statement=<empty>".as_ptr(),
                (*pkt).type_0,
                (*pkt).len,
            );
        }
        return PS_IGNORE;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            c"inspect_bind_packet: type=%c, len=%d, statement=%s".as_ptr(),
            (*pkt).type_0,
            (*pkt).len,
            statement,
        );
    }
    PS_HANDLE
}
#[no_mangle]

pub unsafe extern "C" fn inspect_describe_or_close_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> PreparedStatementAction {
    let mut describe: ::core::ffi::c_char = 0;
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if !mbuf_get_char(&raw mut (*pkt).data, &raw mut describe) {
        return PS_INSPECT_FAILED;
    }
    if describe as ::core::ffi::c_int != 'S' as i32 {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"inspect_describe_or_close_packet: type=%c, len=%d, P/S=%c".as_ptr(),
                (*pkt).type_0,
                (*pkt).len,
                describe as ::core::ffi::c_int,
            );
        }
        return PS_IGNORE;
    }
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut statement) {
        return PS_INSPECT_FAILED;
    }
    if *statement == 0 {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"inspect_describe_or_close_packet: type=%c, len=%d, P/S=%c, statement=<empty>"
                    .as_ptr(),
                (*pkt).type_0,
                (*pkt).len,
                describe as ::core::ffi::c_int,
            );
        }
        return PS_IGNORE;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            c"inspect_describe_or_close_packet: type=%c, len=%d, P/S=%c, statement=%s".as_ptr(),
            (*pkt).type_0,
            (*pkt).len,
            describe as ::core::ffi::c_int,
            statement,
        );
    }
    PS_HANDLE
}
#[no_mangle]

pub unsafe extern "C" fn unmarshall_parse_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut parse_packet: *mut PgParsePacket,
) -> bool {
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    let mut query = ::core::ptr::null::<::core::ffi::c_char>();
    let mut num_parameters: uint16_t = 0;
    let mut parameter_types_bytes = ::core::ptr::null::<uint8_t>();
    let mut parameters_length: size_t = 0;
    if mbuf_get_string(&raw mut (*pkt).data, &raw mut statement)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut query)
        && mbuf_get_uint16be(&raw mut (*pkt).data, &raw mut num_parameters)
    {
        parameters_length = (num_parameters as size_t).wrapping_mul(4 as size_t);
        if mbuf_get_bytes(
            &raw mut (*pkt).data,
            parameters_length as ::core::ffi::c_uint,
            &raw mut parameter_types_bytes,
        ) {
            (*parse_packet).len = (*pkt).len;
            (*parse_packet).name = statement;
            (*parse_packet).query_and_parameters_len = strlen(query)
                .wrapping_add(1 as size_t)
                .wrapping_add(::core::mem::size_of::<uint16_t>() as size_t)
                .wrapping_add(parameters_length);
            (*parse_packet).query_and_parameters = query;
            return true;
        }
    }
    disconnect_client(client, true, c"broken Parse packet".as_ptr());
    false
}
#[no_mangle]

pub unsafe extern "C" fn unmarshall_bind_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut bind_packet: *mut PgBindPacket,
) -> bool {
    let mut portal = ::core::ptr::null::<::core::ffi::c_char>();
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if mbuf_get_string(&raw mut (*pkt).data, &raw mut portal)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut statement)
    {
        (*bind_packet).len = (*pkt).len;
        (*bind_packet).portal = portal;
        (*bind_packet).name = statement;
        return true;
    }
    disconnect_client(client, true, c"broken Bind packet".as_ptr());
    false
}
#[no_mangle]

pub unsafe extern "C" fn unmarshall_describe_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut describe_packet: *mut PgDescribePacket,
) -> bool {
    let mut describe: ::core::ffi::c_char = 0;
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if incomplete_pkt(pkt) {
        return false;
    }
    if mbuf_get_char(&raw mut (*pkt).data, &raw mut describe)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut statement)
    {
        (*describe_packet).type_0 = describe;
        (*describe_packet).name = statement;
        return true;
    }
    disconnect_client(client, true, c"broken Describe packet".as_ptr());
    false
}
#[no_mangle]

pub unsafe extern "C" fn unmarshall_close_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut close_packet: *mut PgClosePacket,
) -> bool {
    let mut type_0: ::core::ffi::c_char = 0;
    let mut name = ::core::ptr::null::<::core::ffi::c_char>();
    if incomplete_pkt(pkt) {
        return false;
    }
    if mbuf_get_char(&raw mut (*pkt).data, &raw mut type_0)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut name)
    {
        (*close_packet).type_0 = type_0;
        (*close_packet).name = name;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"unmarshall_close_packet: type=%c, len=%d, S/P=%c, name=%s".as_ptr(),
                (*pkt).type_0,
                (*pkt).len,
                type_0 as ::core::ffi::c_int,
                name,
            );
        }
        return true;
    }
    disconnect_client(client, true, c"broken Close packet".as_ptr());
    false
}
#[no_mangle]

pub unsafe extern "C" fn is_close_named_statement_packet(
    mut close_packet: *mut PgClosePacket,
) -> bool {
    (*close_packet).type_0 as ::core::ffi::c_int == 'S' as i32
        && *(*close_packet).name as ::core::ffi::c_int != 0
}
