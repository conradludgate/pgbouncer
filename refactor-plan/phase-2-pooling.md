# Phase 2: Connection Pooling & I/O

Core infrastructure for connection management and socket I/O.

## Modules

| Module | Unsafe Fns | #[no_mangle] | Static Mut | Lines |
|--------|------------|--------------|------------|-------|
| pooler.rs | 46 | 9 | 22 | ~3,300 |
| sbuf.rs | 97 | 22 | 33 | ~4,000 |

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
1. Clean up c2rust artifacts
2. Convert pool lists to safe Rust collections (eventually)
3. Replace pointer-based iteration with iterators
4. Keep libevent integration as-is initially

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
sbuf.rs has the most unsafe functions (97) and is performance-critical. It interfaces with:
- libevent for async I/O
- OpenSSL/TLS for encryption
- Raw socket syscalls

### Priority Functions
1. `sbuf_recv_cb()` — receive callback
2. `sbuf_send_pending()` — send buffered data
3. `sbuf_continue()` — resume processing
4. `sbuf_connect()` — initiate connection

### Strategy
1. Clean up c2rust artifacts (careful, lots of them)
2. Keep core I/O unsafe initially — this is low-level
3. Focus on making the interface safer (better types)
4. TLS integration can be swapped later (rustls)

## Checklist

- [ ] pooler.rs: Remove c2rust type modules
- [ ] pooler.rs: Import from common/types.rs
- [ ] pooler.rs: Convert static mut to thread-local
- [ ] pooler.rs: Identify safe vs unsafe boundaries
- [ ] sbuf.rs: Remove c2rust type modules
- [ ] sbuf.rs: Import from common/types.rs
- [ ] sbuf.rs: Document unsafe invariants
- [ ] sbuf.rs: Make SBuf interface safer (not internals)
- [ ] All: Run integration tests after changes
