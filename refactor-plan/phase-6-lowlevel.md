# Phase 6: Low-Level Modules

Crypto, string utilities, and libusual infrastructure.

## Modules

### src/common/

| Module | Description |
|--------|-------------|
| types.rs | Shared type definitions (already cleaned up) |
| ascii.rs | ASCII validation (partially cleaned) |
| base64.rs | Base64 encoding/decoding |
| cryptohash.rs | Cryptographic hash abstraction |
| hmac.rs | HMAC implementation |
| sha2.rs | SHA-2 implementation |
| scram_common.rs | Shared SCRAM functions |
| string.rs | String utilities |
| bool.rs | Boolean parsing |
| pgstrcasecmp.rs | Case-insensitive comparison |
| saslprep.rs | SASL string preparation |
| unicode_norm.rs | Unicode normalization |
| wchar.rs | Wide character handling |

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
| scram_common.rs | Custom safe impl or crate |
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

### Option A: Gradual Replacement
Replace one module at a time with crate equivalents, updating callers.

### Option B: Abstraction Layer
Create safe Rust trait interfaces, implement with current unsafe code, then swap implementations.

### Option C: Full Rewrite
Once business logic is safe, rewrite low-level modules from scratch with modern Rust.

## Checklist

- [ ] Audit which low-level functions are actually used
- [ ] Document external dependencies (OpenSSL, libevent)
- [ ] Identify crate replacements for each module
- [ ] Create abstraction traits where beneficial
- [ ] (Later) Replace implementations one at a time
