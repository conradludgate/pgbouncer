# Phase 6: Low-Level Modules

Crypto, string utilities, and libusual infrastructure.

## Current Metrics (2026-01-30)

| Location | Lines | Description |
|----------|-------|-------------|
| src/common/*.rs | 58,377 | Shared types, crypto, string utilities |
| lib/usual/*.rs | ~25,900 | Low-level infrastructure library |

## Modules

### src/common/

| Module | Description | Strategy |
|--------|-------------|----------|
| types.rs | Shared type definitions | ✅ Consolidated, keep expanding |
| ascii.rs | ASCII validation | Keep for now |
| base64.rs | Base64 encoding/decoding | Replace with `base64` crate |
| cryptohash.rs | Cryptographic hash abstraction | Replace with RustCrypto |
| hmac.rs | HMAC implementation | Replace with `hmac` crate |
| sha2.rs | SHA-2 implementation | Replace with `sha2` crate |
| scram_common.rs | Shared SCRAM functions | Keep or use `scram` crate |
| string.rs | String utilities | Keep for now |
| bool.rs | Boolean parsing | Keep for now |
| pgstrcasecmp.rs | Case-insensitive comparison | Keep for now |
| saslprep.rs | SASL string preparation | Keep or use `stringprep` crate |
| unicode_norm.rs | Unicode normalization (46k lines!) | Replace with `unicode-normalization` |
| wchar.rs | Wide character handling (6k lines) | Replace with std or crate |

### lib/usual/ (~26,000 lines)

Low-level infrastructure library:
- Event loop primitives
- TLS/OpenSSL wrappers
- Socket utilities
- Memory pools
- Logging
- Various C utility functions

## Strategy: Replace Rather Than Refactor

For many of these modules, the best approach is **replacement with Rust crates**:

| Module | Potential Replacement |
|--------|----------------------|
| base64.rs | `base64` crate |
| sha2.rs | `sha2` crate |
| hmac.rs | `hmac` crate |
| unicode_norm.rs | `unicode-normalization` crate |
| wchar.rs | `unicode-width` + std |
| scram_common.rs | Custom safe impl or `scram` crate |
| lib/usual/tls/ | `rustls` + `tokio-rustls` |
| lib/usual/crypto/ | `ring` or RustCrypto crates |

## When to Do This

**After** the higher-level modules are refactored:
1. Business logic is safe and uses clean interfaces
2. We understand exactly what functionality is needed
3. We can swap implementations without touching business logic

## Minimal Cleanup Now

For now, just:
1. Ensure `common/types.rs` exports what other modules need
2. Keep FFI interfaces stable
3. Document what each module provides

## Future Work

### Option A: Gradual Replacement (Recommended)
Replace one module at a time with crate equivalents, updating callers.

**Priority order:**
1. `base64.rs` → `base64` crate (simple, well-defined interface)
2. `sha2.rs` + `hmac.rs` → RustCrypto crates
3. `unicode_norm.rs` → `unicode-normalization` (saves 46k lines!)
4. TLS → `rustls` (bigger change, do later)

### Option B: Abstraction Layer
Create safe Rust trait interfaces, implement with current unsafe code, then swap implementations.

### Option C: Full Rewrite
Once business logic is safe, rewrite low-level modules from scratch with modern Rust.

## Checklist

### Completed ✅
- [x] types.rs: Consolidated shared types (PgStats, List, StatList, AATree, etc.)
- [x] lib/usual/*: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] src/common/*: Remove c2rust::src_loc and c2rust::header_src attributes

### Blocked 🚧 (Waiting on Higher-Level Refactoring)
- [ ] Need stable interfaces before replacing implementations
- [ ] Need to understand what functionality is actually used

### Pending 📋 (Long-term)
- [ ] Audit which low-level functions are actually used
- [ ] Document external dependencies (OpenSSL, libevent)
- [ ] Identify crate replacements for each module
- [ ] Create abstraction traits where beneficial
- [ ] Replace implementations one at a time
- [ ] Remove unicode_norm.rs (46k lines!) with crate

### Types Consolidated to common/types.rs
- `PgStats` - Statistics counters
- `List`, `list_*` functions - Linked list
- `StatList`, `statlist_*` functions - Statistics list
- `AATree`, `AANode`, `aatree_*` functions - AA tree
- `MBuf` - Memory buffer (moved to lib/usual/mbuf.rs)
- `PktHdr` - Packet header
- Various primitive type aliases (`usec_t`, `uint64_t`, etc.)
- Socket structures (`sockaddr`, `sockaddr_in`, etc.)

### Types NOT YET Consolidated (Priority)
- `PgSocket`, `PgPool`, `PgDatabase`, `PgCredentials` (bouncer_h) — **do this next**
- `SBuf`, `SBufIO` (sbuf_h)
- `IOBuf` (iobuf_h)

## Notes

**unicode_norm.rs is 46,000 lines!**
- Generated Unicode tables
- Replace with `unicode-normalization` crate to save massive LOC
- Low priority but huge impact when done

**wchar.rs is 6,000 lines**
- Wide character handling
- Much of this may be unused
- Audit before replacing
