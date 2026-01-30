# Phase 1: Core Modules

Entry point and core connection handling.

## Modules

| Module | Unsafe Fns | #[no_mangle] | Static Mut | Lines |
|--------|------------|--------------|------------|-------|
| main.rs | 261 | 109 | 134 | ~6,200 |
| client.rs | 64 | 7 | 17 | ~4,500 |
| server.rs | 46 | 14 | 14 | ~3,200 |

## main.rs

### Overview
Entry point for pgbouncer. Contains:
- Command-line parsing
- Configuration initialization
- Signal handling
- Main event loop startup
- Many global config variables (`cf_*`)

### Key Globals to Convert
```
cf_verbose, cf_daemon, cf_pause_mode, cf_shutdown
cf_listen_addr, cf_listen_port, cf_unix_socket_dir
cf_pool_mode, cf_max_client_conn, cf_default_pool_size
... (134 total static mut variables)
```

### Priority Functions
1. `main()` — entry point
2. `load_config()` — configuration loading
3. Signal handlers

### Strategy
1. Remove c2rust type modules (hundreds of lines of noise)
2. Import types from `common/types.rs`
3. Group related `static mut` into config structs
4. Convert config struct to thread-local RefCell
5. Identify internal vs FFI functions

## client.rs

### Overview
Handles client connections:
- Connection acceptance
- Authentication flow
- Query routing
- State machine for client lifecycle

### Key States (from types.rs)
```rust
CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, 
CL_WAITING_LOGIN, CL_ACTIVE, CL_WAITING_CANCEL, CL_ACTIVE_CANCEL
```

### Priority Functions
1. `client_proto()` — main protocol handler callback
2. `accept_client()` — new connection handling
3. `client_auth_*()` — authentication functions
4. `disconnect_client()` — cleanup

### Strategy
1. Clean up c2rust artifacts
2. Convert state constants to proper enum
3. Replace raw PgSocket pointers with references where possible
4. Convert callback-based flow to more idiomatic patterns

## server.rs

### Overview
Handles PostgreSQL backend connections:
- Connection establishment
- Authentication with backend
- Query forwarding
- Connection health checks

### Key States
```rust
SV_FREE, SV_JUSTFREE, SV_LOGIN, SV_BEING_CANCELED,
SV_IDLE, SV_ACTIVE, SV_ACTIVE_CANCEL, SV_USED, SV_TESTED
```

### Priority Functions
1. `server_proto()` — main protocol handler
2. `connect_server()` — establish backend connection
3. `server_auth_*()` — backend authentication
4. `release_server()` — return to pool

### Strategy
1. Clean up c2rust artifacts
2. Mirror client.rs refactoring patterns
3. Ensure client/server state machines are consistent

## Checklist

### Completed ✅
- [x] main.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] client.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] server.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] All: Consolidate PgStats type to common/types.rs
- [x] All: Consolidate List/StatList types to common/types.rs
- [x] All: Consolidate AATree types to common/types.rs
- [x] All: Run test_admin.py after each major change

### In Progress 🔄
- [ ] main.rs: Remove remaining c2rust type modules
- [ ] client.rs: Remove remaining c2rust type modules
- [ ] server.rs: Remove remaining c2rust type modules

### Pending 📋
- [ ] main.rs: Import all types from common/types.rs
- [ ] main.rs: Convert cf_* globals to config struct
- [ ] main.rs: Wrap config in thread-local RefCell
- [ ] main.rs: Remove extern "C" from internal functions (careful: some are C callbacks)
- [ ] client.rs: Import all types from common/types.rs
- [ ] client.rs: Convert SocketState to proper enum
- [ ] client.rs: Convert static mut to thread-local
- [ ] server.rs: Import all types from common/types.rs
- [ ] server.rs: Mirror client.rs patterns
- [ ] All: Run full test suite before completing phase

### Notes
- Removing `extern "C"` requires care: some internal functions are used as libevent callbacks
- Type consolidation is ongoing: PgStats, List, StatList, AATree done; MBuf pending
- Each type consolidated saves ~15-25 lines per file (20+ files)
