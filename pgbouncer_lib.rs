#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod lib {
    pub mod usual {
        pub mod aatree;
        pub mod base;
        pub mod cbtree;
        pub mod cfparser;
        pub mod crypto {
            pub mod chacha;
            pub mod csrandom;
            pub mod digest;
            pub mod entropy;
            pub mod keccak;
            pub mod keccak_prng;
            pub mod md5;
        } // mod crypto
        pub mod cxalloc;
        pub mod cxextra;
        pub mod err;
        pub mod fileutil;
        pub mod getopt;
        pub mod list;
        pub mod logging;
        pub mod mbuf;
        pub mod mempool;
        pub mod netdb;
        pub mod pgutil;
        pub mod regex;
        pub mod safeio;
        pub mod signal;
        pub mod slab;
        pub mod socket;
        pub mod socket_ntop;
        pub mod socket_pton;
        pub mod string;
        pub mod strpool;
        pub mod time;
        pub mod tls {
            pub mod tls;
            pub mod tls_cert;
            pub mod tls_client;
            pub mod tls_compat;
            pub mod tls_config;
            pub mod tls_conninfo;
            pub mod tls_ocsp;
            pub mod tls_peer;
            pub mod tls_server;
            pub mod tls_util;
            pub mod tls_verify;
        } // mod tls
    } // mod usual
} // mod lib
pub mod src {
    pub mod admin;
    pub mod client;
    pub mod common {
        pub mod ascii;
        pub mod base64;
        pub mod r#bool;
        pub mod cryptohash;
        pub mod hmac;
        pub mod pgstrcasecmp;
        pub mod saslprep;
        pub mod scram_common;
        pub mod sha2;
        pub mod string;
        pub mod unicode_norm;
        pub mod wchar;
    } // mod common
    pub mod dnslookup;
    pub mod hba;
    pub mod janitor;
    pub mod ldapauth;
    pub mod loader;
    pub mod messages;
    pub mod objects;
    pub mod pam;
    pub mod pktbuf;
    pub mod pooler;
    pub mod prepare;
    pub mod proto;
    pub mod sbuf;
    pub mod scram;
    pub mod server;
    pub mod stats;
    pub mod system;
    pub mod takeover;
    pub mod util;
    pub mod varcache;
} // mod src
