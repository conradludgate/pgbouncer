# Phase 2: Connection Pooling & I/O

Core infrastructure for connection management and socket I/O.

## Current Metrics (2026-01-30)

| Module | Lines | Unsafe Fns | #[no_mangle] | Static Mut | Duplicate Modules |
|--------|-------|------------|--------------|------------|-------------------|
| pooler.rs | 3,300 | 38 | 9 | 22 | 39 |
| sbuf.rs | 4,000 | 90 | 22 | 33 | 25 |

## pooler.rs

### Overview
Connection pool management:
- Pool creation and destruction
- Connection allocation/release
- Load balancing
- Event loop integration

### Key Data Structures
- `PgPool` — connection pool for a database/user pair
- Pool lists: `pool_list`, `database_list`, `user_list`

### Priority Functions
1. `pooler_loop()` — main event loop
2. `get_server()` — acquire server connection from pool
3. `release_server()` — return connection to pool
4. `pool_*()` functions — pool lifecycle

### Dependencies
- Heavily uses `objects.rs` data structures
- Integrates with libevent

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules (depends on bouncer_h)
3. Convert pool lists to safe Rust collections (eventually)
4. Replace pointer-based iteration with iterators
5. Keep libevent integration as-is initially

## sbuf.rs

### Overview
Socket buffer abstraction — the core I/O layer:
- Buffered reading/writing
- TLS integration
- Packet parsing callbacks
- Flow control

### Key Data Structures
- `SBuf` — socket buffer with event handling
- `IOBuf` — raw I/O buffer
- `SBufIO` — I/O operations vtable

### This is Complex
sbuf.rs has 90 unsafe functions and is performance-critical. It interfaces with:
- libevent for async I/O
- OpenSSL/TLS for encryption
- Raw socket syscalls

### Priority Functions
1. `sbuf_recv_cb()` — receive callback
2. `sbuf_send_pending()` — send buffered data
3. `sbuf_continue()` — resume processing
4. `sbuf_connect()` — initiate connection

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules (depends on bouncer_h, sbuf_h, iobuf_h)
3. Keep core I/O unsafe initially — this is low-level
4. Focus on making the interface safer (better types)
5. TLS integration can be swapped later (rustls)

## Checklist

### Completed ✅
- [x] pooler.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] pooler.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs
- [x] sbuf.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] sbuf.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs

### Blocked 🚧 (Waiting on Type Consolidation)
- [ ] **Consolidate bouncer_h** first (Phase 1 blocker)
- [ ] Consolidate sbuf_h (20 files) — SBuf, SBufIO types
- [ ] Consolidate iobuf_h (20 files) — IOBuf type

### Pending 📋 (After Type Consolidation)
- [ ] pooler.rs: Remove remaining type modules (~39 modules)
- [ ] pooler.rs: Import all types from crate::types::*
- [ ] pooler.rs: Convert static mut to thread-local (22 occurrences)
- [ ] pooler.rs: Identify safe vs unsafe boundaries
- [ ] pooler.rs: Apply function cleanup patterns
- [ ] sbuf.rs: Remove remaining type modules (~25 modules)
- [ ] sbuf.rs: Import all types from crate::types::*
- [ ] sbuf.rs: Document unsafe invariants
- [ ] sbuf.rs: Make SBuf interface safer (not internals)
- [ ] All: Run integration tests after changes

### Notes

**sbuf.rs is the most complex module** — don't try to make it safe all at once. Focus on:
1. Type consolidation first
2. Document what invariants the unsafe code relies on
3. Make the public interface safer
4. Keep internals unsafe until we understand them fully

**Dependencies:**
- pooler.rs depends on: bouncer_h (PgPool, PgSocket), objects.rs
- sbuf.rs depends on: bouncer_h, iobuf_h, sbuf_h, TLS/OpenSSL
