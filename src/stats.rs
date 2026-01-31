pub mod bouncer_h {
    pub use crate::types::*;
    pub use c2rust_bitfields::BitfieldStruct;
    extern "C" {

    pub static mut pgb_event_base: *mut event_base;

    pub static mut cf_stats_period: ::core::ffi::c_int;

    pub static mut cf_log_stats: ::core::ffi::c_int;
    }
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
    extern "C" {

        pub fn pktbuf_dynamic(start_len: ::core::ffi::c_int) -> *mut PktBuf;

        pub fn pktbuf_write_RowDescription(
            buf: *mut PktBuf,
            tupdesc: *const ::core::ffi::c_char,
            ...
        );

        pub fn pktbuf_write_DataRow(buf: *mut PktBuf, tupdesc: *const ::core::ffi::c_char, ...);
    }






}

pub mod dnslookup_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;
    extern "C" {
    }






}

pub mod admin_h {

    pub use crate::types::*;
    extern "C" {

        pub fn admin_error(console: *mut PgSocket, fmt: *const ::core::ffi::c_char, ...) -> bool;

        pub fn admin_flush(
            admin: *mut PgSocket,
            buf: *mut PktBuf,
            desc: *const ::core::ffi::c_char,
        ) -> bool;
    }


}

pub mod objects_h {

    pub use crate::types::*;
    extern "C" {

        pub static mut pool_list: StatList;
    }


}

use crate::types::strerror;
use self::admin_h::{admin_error, admin_flush};
pub use self::bouncer_h::{
    cf_log_stats, cf_stats_period, pgb_event_base, sockaddr_ucreds, C2RustUnnamed_9, CallbackState,
    LoadBalanceHosts, PacketCallbackFlag, PgAddr, PgCredentials, PgDatabase, PgGlobalUser, PgPool,
    PgSocket, PgStats, ReplicationType, ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET,
    CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN,
    CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL,
    SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN,
    SV_TESTED, SV_USED,
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
pub use crate::types::NULL;
pub use crate::types::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint16_t,
    __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

use crate::types::__error;
pub use crate::types::{event_add, event_assign, event_base, event_callback_fn, EV_PERSIST};
pub use crate::types::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::{
    log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
use self::objects_h::pool_list;
pub use self::pktbuf_h::{
    pktbuf_dynamic, pktbuf_write_DataRow, pktbuf_write_RowDescription, PktBuf,
};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use crate::types::sockaddr;
pub use crate::types::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::true_0;
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::StatList;
pub use crate::types::{usec_t, USEC};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};

static mut ev_stats: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};

static mut old_stamp: usec_t = 0;

static mut new_stamp: usec_t = 0;

unsafe fn reset_stats(stat: *mut PgStats) {
    (*stat).server_bytes = 0;
    (*stat).client_bytes = 0;
    (*stat).server_assignment_count = 0;
    (*stat).query_count = 0;
    (*stat).query_time = 0;
    (*stat).xact_count = 0;
    (*stat).xact_time = 0;
    (*stat).wait_time = 0;
    (*stat).ps_client_parse_count = 0;
    (*stat).ps_server_parse_count = 0;
    (*stat).ps_bind_count = 0;
}

unsafe fn stat_add(total: *mut PgStats, stat: *mut PgStats) {
    (*total).server_bytes += (*stat).server_bytes;
    (*total).client_bytes += (*stat).client_bytes;
    (*total).server_assignment_count += (*stat).server_assignment_count;
    (*total).query_count += (*stat).query_count;
    (*total).query_time += (*stat).query_time;
    (*total).xact_count += (*stat).xact_count;
    (*total).xact_time += (*stat).xact_time;
    (*total).wait_time += (*stat).wait_time;
    (*total).ps_client_parse_count += (*stat).ps_client_parse_count;
    (*total).ps_server_parse_count += (*stat).ps_server_parse_count;
    (*total).ps_bind_count += (*stat).ps_bind_count;
}

unsafe fn calc_average(avg: *mut PgStats, cur: *mut PgStats, old: *mut PgStats) {
    let dur: usec_t = get_cached_time() - old_stamp;
    reset_stats(avg);
    if dur == 0 {
        return;
    }

    let query_count = (*cur).query_count - (*old).query_count;
    let xact_count = (*cur).xact_count - (*old).xact_count;
    let server_assignment_count = (*cur).server_assignment_count - (*old).server_assignment_count;

    (*avg).query_count = (USEC * query_count) / dur;
    (*avg).xact_count = (USEC * xact_count) / dur;
    (*avg).server_assignment_count = (USEC * server_assignment_count) / dur;
    (*avg).client_bytes = (USEC * ((*cur).client_bytes - (*old).client_bytes)) / dur;
    (*avg).server_bytes = (USEC * ((*cur).server_bytes - (*old).server_bytes)) / dur;

    if query_count > 0 {
        (*avg).query_time = ((*cur).query_time - (*old).query_time) / query_count;
    }
    if xact_count > 0 {
        (*avg).xact_time = ((*cur).xact_time - (*old).xact_time) / xact_count;
    }
    if server_assignment_count > 0 {
        (*avg).wait_time = ((*cur).wait_time - (*old).wait_time) / server_assignment_count;
    }

    let ps_client_parse_count = (*cur).ps_client_parse_count - (*old).ps_client_parse_count;
    let ps_server_parse_count = (*cur).ps_server_parse_count - (*old).ps_server_parse_count;
    let ps_bind_count = (*cur).ps_bind_count - (*old).ps_bind_count;

    (*avg).ps_client_parse_count = (USEC * ps_client_parse_count) / dur;
    (*avg).ps_server_parse_count = (USEC * ps_server_parse_count) / dur;
    (*avg).ps_bind_count = (USEC * ps_bind_count) / dur;
}

unsafe extern "C" fn write_stats(
    mut buf: *mut PktBuf,
    mut stat: *mut PgStats,
    mut old: *mut PgStats,
    mut dbname: *mut ::core::ffi::c_char,
) {
    let mut avg = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    calc_average(&raw mut avg, stat, old);
    pktbuf_write_DataRow(
        buf,
        c"sNNNNNNNNNNNNNNNNNNNNNN".as_ptr(),
        dbname,
        (*stat).server_assignment_count,
        (*stat).xact_count,
        (*stat).query_count,
        (*stat).client_bytes,
        (*stat).server_bytes,
        (*stat).xact_time,
        (*stat).query_time,
        (*stat).wait_time,
        (*stat).ps_client_parse_count,
        (*stat).ps_server_parse_count,
        (*stat).ps_bind_count,
        avg.server_assignment_count,
        avg.xact_count,
        avg.query_count,
        avg.client_bytes,
        avg.server_bytes,
        avg.xact_time,
        avg.query_time,
        avg.wait_time,
        avg.ps_client_parse_count,
        avg.ps_server_parse_count,
        avg.ps_bind_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn admin_database_stats(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut cur_db = ::core::ptr::null_mut::<PgDatabase>();
    let mut st_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_db);
    reset_stats(&raw mut old_db);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(client, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"sNNNNNNNNNNNNNNNNNNNNNN".as_ptr(),
        c"database".as_ptr(),
        c"total_server_assignment_count".as_ptr(),
        c"total_xact_count".as_ptr(),
        c"total_query_count".as_ptr(),
        c"total_received".as_ptr(),
        c"total_sent".as_ptr(),
        c"total_xact_time".as_ptr(),
        c"total_query_time".as_ptr(),
        c"total_wait_time".as_ptr(),
        c"total_client_parse_count".as_ptr(),
        c"total_server_parse_count".as_ptr(),
        c"total_bind_count".as_ptr(),
        c"avg_server_assignment_count".as_ptr(),
        c"avg_xact_count".as_ptr(),
        c"avg_query_count".as_ptr(),
        c"avg_recv".as_ptr(),
        c"avg_sent".as_ptr(),
        c"avg_xact_time".as_ptr(),
        c"avg_query_time".as_ptr(),
        c"avg_wait_time".as_ptr(),
        c"avg_client_parse_count".as_ptr(),
        c"avg_server_parse_count".as_ptr(),
        c"avg_bind_count".as_ptr(),
    );
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if cur_db.is_null() {
            cur_db = (*pool).db;
        }
        if (*pool).db != cur_db {
            write_stats(
                buf,
                &raw mut st_db,
                &raw mut old_db,
                &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
            );
            cur_db = (*pool).db;
            reset_stats(&raw mut st_db);
            reset_stats(&raw mut old_db);
        }
        stat_add(&raw mut st_db, &raw mut (*pool).stats);
        stat_add(&raw mut old_db, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    if !cur_db.is_null() {
        write_stats(
            buf,
            &raw mut st_db,
            &raw mut old_db,
            &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
        );
    }
    admin_flush(client, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn write_stats_totals(
    mut buf: *mut PktBuf,
    mut stat: *mut PgStats,
    mut _old: *mut PgStats,
    mut dbname: *mut ::core::ffi::c_char,
) {
    pktbuf_write_DataRow(
        buf,
        c"sNNNNNNNNNNN".as_ptr(),
        dbname,
        (*stat).server_assignment_count,
        (*stat).xact_count,
        (*stat).query_count,
        (*stat).client_bytes,
        (*stat).server_bytes,
        (*stat).xact_time,
        (*stat).query_time,
        (*stat).wait_time,
        (*stat).ps_client_parse_count,
        (*stat).ps_server_parse_count,
        (*stat).ps_bind_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn admin_database_stats_totals(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut cur_db = ::core::ptr::null_mut::<PgDatabase>();
    let mut st_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_db);
    reset_stats(&raw mut old_db);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(client, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"sNNNNNNNNNNN".as_ptr(),
        c"database".as_ptr(),
        c"server_assignment_count".as_ptr(),
        c"xact_count".as_ptr(),
        c"query_count".as_ptr(),
        c"bytes_received".as_ptr(),
        c"bytes_sent".as_ptr(),
        c"xact_time".as_ptr(),
        c"query_time".as_ptr(),
        c"wait_time".as_ptr(),
        c"client_parse_count".as_ptr(),
        c"server_parse_count".as_ptr(),
        c"bind_count".as_ptr(),
    );
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if cur_db.is_null() {
            cur_db = (*pool).db;
        }
        if (*pool).db != cur_db {
            write_stats_totals(
                buf,
                &raw mut st_db,
                &raw mut old_db,
                &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
            );
            cur_db = (*pool).db;
            reset_stats(&raw mut st_db);
            reset_stats(&raw mut old_db);
        }
        stat_add(&raw mut st_db, &raw mut (*pool).stats);
        stat_add(&raw mut old_db, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    if !cur_db.is_null() {
        write_stats_totals(
            buf,
            &raw mut st_db,
            &raw mut old_db,
            &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
        );
    }
    admin_flush(client, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn write_stats_averages(
    mut buf: *mut PktBuf,
    mut stat: *mut PgStats,
    mut old: *mut PgStats,
    mut dbname: *mut ::core::ffi::c_char,
) {
    let mut avg = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    calc_average(&raw mut avg, stat, old);
    pktbuf_write_DataRow(
        buf,
        c"sNNNNNNNNNNN".as_ptr(),
        dbname,
        avg.server_assignment_count,
        avg.xact_count,
        avg.query_count,
        avg.client_bytes,
        avg.server_bytes,
        avg.xact_time,
        avg.query_time,
        avg.wait_time,
        avg.ps_client_parse_count,
        avg.ps_server_parse_count,
        avg.ps_bind_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn admin_database_stats_averages(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut cur_db = ::core::ptr::null_mut::<PgDatabase>();
    let mut st_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_db);
    reset_stats(&raw mut old_db);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(client, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"sNNNNNNNNNNN".as_ptr(),
        c"database".as_ptr(),
        c"server_assignment_count".as_ptr(),
        c"xact_count".as_ptr(),
        c"query_count".as_ptr(),
        c"bytes_received".as_ptr(),
        c"bytes_sent".as_ptr(),
        c"xact_time".as_ptr(),
        c"query_time".as_ptr(),
        c"wait_time".as_ptr(),
        c"avg_client_parse_count".as_ptr(),
        c"avg_server_parse_count".as_ptr(),
        c"avg_bind_count".as_ptr(),
    );
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if cur_db.is_null() {
            cur_db = (*pool).db;
        }
        if (*pool).db != cur_db {
            write_stats_averages(
                buf,
                &raw mut st_db,
                &raw mut old_db,
                &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
            );
            cur_db = (*pool).db;
            reset_stats(&raw mut st_db);
            reset_stats(&raw mut old_db);
        }
        stat_add(&raw mut st_db, &raw mut (*pool).stats);
        stat_add(&raw mut old_db, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    if !cur_db.is_null() {
        write_stats_averages(
            buf,
            &raw mut st_db,
            &raw mut old_db,
            &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
        );
    }
    admin_flush(client, buf, c"SHOW".as_ptr());
    true
}
#[no_mangle]

pub unsafe extern "C" fn show_stat_totals(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut st_total = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_total = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut avg = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_total);
    reset_stats(&raw mut old_total);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(client, c"no mem".as_ptr());
        return true;
    }
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        stat_add(&raw mut st_total, &raw mut (*pool).stats);
        stat_add(&raw mut old_total, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    calc_average(&raw mut avg, &raw mut st_total, &raw mut old_total);
    pktbuf_write_RowDescription(buf, c"sN".as_ptr(), c"name".as_ptr(), c"value".as_ptr());
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_server_assignment_count".as_ptr(),
        st_total.server_assignment_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_xact_count".as_ptr(),
        st_total.xact_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_query_count".as_ptr(),
        st_total.query_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_client_bytes".as_ptr(),
        st_total.client_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_server_bytes".as_ptr(),
        st_total.server_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_xact_time".as_ptr(),
        st_total.xact_time,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_query_time".as_ptr(),
        st_total.query_time,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_wait_time".as_ptr(),
        st_total.wait_time,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_ps_client_parse_count".as_ptr(),
        st_total.ps_client_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_ps_server_parse_count".as_ptr(),
        st_total.ps_server_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"total_ps_bind_count".as_ptr(),
        st_total.ps_bind_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_server_assignment_count".as_ptr(),
        avg.server_assignment_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_xact_count".as_ptr(),
        avg.xact_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_query_count".as_ptr(),
        avg.query_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_client_bytes".as_ptr(),
        avg.client_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_server_bytes".as_ptr(),
        avg.server_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_xact_time".as_ptr(),
        avg.xact_time,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_query_time".as_ptr(),
        avg.query_time,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_wait_time".as_ptr(),
        avg.wait_time,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_ps_client_parse_count".as_ptr(),
        avg.ps_client_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_ps_server_parse_count".as_ptr(),
        avg.ps_server_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        c"sN".as_ptr(),
        c"avg_ps_bind_count".as_ptr(),
        avg.ps_bind_count,
    );
    admin_flush(client, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn refresh_stats(
    _s: ::core::ffi::c_int,
    _flags: ::core::ffi::c_short,
    _arg: *mut ::core::ffi::c_void,
) {
    let mut item: *mut List;
    let mut pool: *mut PgPool;
    let mut old_total: PgStats = ::core::mem::zeroed();
    let mut cur_total: PgStats = ::core::mem::zeroed();
    let mut avg: PgStats = ::core::mem::zeroed();

    reset_stats(&raw mut old_total);
    reset_stats(&raw mut cur_total);
    old_stamp = new_stamp;
    new_stamp = get_cached_time();

    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = item as *mut PgPool;
        (*pool).older_stats = (*pool).newer_stats;
        (*pool).newer_stats = (*pool).stats;
        if cf_log_stats != 0 {
            stat_add(&raw mut cur_total, &raw mut (*pool).stats);
            stat_add(&raw mut old_total, &raw mut (*pool).older_stats);
        }
        item = (*item).next;
    }

    calc_average(&raw mut avg, &raw mut cur_total, &raw mut old_total);

    if cf_log_stats != 0 {
        log_generic(
            LG_INFO,
            NULL,
            c"stats: %llu xacts/s, %llu queries/s, %llu client parses/s, %llu server parses/s, %llu binds/s, in %llu B/s, out %llu B/s, xact %llu us, query %llu us, wait %llu us".as_ptr(),
            avg.xact_count,
            avg.query_count,
            avg.ps_client_parse_count,
            avg.ps_server_parse_count,
            avg.ps_bind_count,
            avg.client_bytes,
            avg.server_bytes,
            avg.xact_time,
            avg.query_time,
            avg.wait_time,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_setup() {
    let mut period = timeval {
        tv_sec: cf_stats_period as __darwin_time_t,
        tv_usec: 0,
    };
    new_stamp = get_cached_time();
    old_stamp = new_stamp - USEC;

    event_assign(
        &raw mut ev_stats,
        pgb_event_base,
        -1,
        EV_PERSIST as ::core::ffi::c_short,
        Some(refresh_stats),
        NULL,
    );

    if event_add(&raw mut ev_stats, &raw mut period) < 0 {
        log_generic(
            LG_WARNING,
            NULL,
            c"event_add failed: %s".as_ptr(),
            strerror(*__error()),
        );
    }
}

extern "C" {
    pub fn get_cached_time() -> usec_t;
}
