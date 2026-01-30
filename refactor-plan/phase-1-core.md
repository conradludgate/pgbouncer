# Phase 1: Core Modules

Entry point and core connection handling.

## Current Metrics (2026-01-30)

| Module | Lines | Unsafe Fns | #[no_mangle] | Static Mut | Duplicate Modules |
|--------|-------|------------|--------------|------------|-------------------|
| main.rs | 5,950 | 258 | 109 | 134 | 48 |
| client.rs | 4,500 | 41 | 7 | 17 | 33 |
| server.rs | 3,200 | 29 | 14 | 14 | 22 |

## main.rs

### Overview
Entry point for pgbouncer. Contains:
- Command-line parsing
- Configuration initialization
- Signal handling
- Main event loop startup
- Many global config variables (`cf_*`)

### Key Globals to Convert (134 total)
```
cf_verbose, cf_daemon, cf_pause_mode, cf_shutdown
cf_listen_addr, cf_listen_port, cf_unix_socket_dir
cf_pool_mode, cf_max_client_conn, cf_default_pool_size
... and 124 more
```

### Priority Functions
1. `main()` — entry point
2. `load_config()` — configuration loading
3. Signal handlers

### Strategy
1. ~~Remove c2rust type modules~~ → Consolidate type modules first
2. Import types from `crate::types::*` (after bouncer_h consolidation)
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
1. Consolidate type modules (depends on bouncer_h)
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
1. Consolidate type modules (depends on bouncer_h)
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
- [x] All: Consolidate MBuf type and inline functions to lib/usual/mbuf.rs
- [x] All: Remove mbuf_h modules from all 22 files
- [x] All: Consolidate proto_h (PktHdr) type and inline functions to types.rs
- [x] All: Remove proto_h modules from all 20 files
- [x] All: Consolidate prepare_h types to types.rs
- [x] All: Remove prepare_h modules from all 20 files

### Blocked 🚧 (Complex Consolidation Required)
- [ ] **Consolidate bouncer_h** (22 files) — PgSocket, PgPool, PgDatabase
  - ⚠️ The consolidation script doesn't work directly because bouncer_h mixes type definitions with extern static declarations
  - **Manual approach required:** Add types to types.rs, keep externs in each file
  - See README.md for detailed approach
- [ ] Consolidate iobuf_h (20 files) — IOBuf (simpler, script may work)
- [ ] Consolidate sbuf_h (20 files) — SBuf (depends on bouncer_h types)
- [ ] Remove primitive type modules (_types_h, sys__types_h, etc.)

### Pending 📋 (After Type Consolidation)
- [ ] main.rs: Import all types from crate::types::*
- [ ] main.rs: Convert cf_* globals to config struct
- [ ] main.rs: Wrap config in thread-local RefCell
- [ ] main.rs: Remove extern "C" from internal functions
- [ ] main.rs: Apply function cleanup patterns (see stats.rs findings)
- [ ] client.rs: Import all types from crate::types::*
- [ ] client.rs: Convert SocketState to proper enum
- [ ] client.rs: Convert static mut to thread-local
- [ ] server.rs: Import all types from crate::types::*
- [ ] server.rs: Mirror client.rs patterns
- [ ] All: Run full test suite

### Notes

**Type Consolidation Progress:**
- PgStats, List, StatList, AATree: consolidated to `src/common/types.rs`
- MBuf + inline functions: consolidated to `lib/usual/mbuf.rs`
- PktHdr: consolidated to `src/common/types.rs`
- **bouncer_h NOT YET DONE** — this is the biggest remaining blocker

**Remaining Duplicate Modules to Consolidate (Priority Order):**

1. `bouncer_h` — PgSocket, PgPool, PgDatabase, PgCredentials (~22 files)
   - Complex, many dependencies, has bitfields
   - This is the critical path blocker
   
2. `iobuf_h` — IOBuf struct (~20 files)

3. `sbuf_h` — SBuf, SBufEvent, SBufIO (~20 files)

4. Primitive modules — `_types_h`, `socket_h`, `in_h`, etc.
   - These are just re-exports of libc types
   - Can be deleted once imports are updated

**Key Patterns:**
- Inner modules like `sbuf_h` depend on `bouncer_h` types
- `src/main.rs` is the binary, uses `pgbouncer::` instead of `crate::`
- Some inline functions call extern functions — keep those declarations

**Testing:**
- Tests require PostgreSQL `initdb` + `postgres` in same directory
- If tests fail with "postgres not found", ensure full PostgreSQL is installed
