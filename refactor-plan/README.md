# PgBouncer Rust Refactoring Plan

Refactoring the c2rust-translated PgBouncer codebase into idiomatic, safe Rust.

## Goals

1. **Make business logic safe first** — prioritize higher-level modules
2. **Keep behavior equivalent** — no functional changes during refactoring
3. **Enable future swaps** — lower-level details (crypto, TLS) can be replaced with Rust crates later

## Current State (2026-01-31)

| Metric | Original | Current | Target |
|--------|----------|---------|--------|
| Total Rust lines (src/*.rs) | ~126,000 | **~42,500** | <30,000 |
| Duplicate `pub mod *_h` modules | ~800 | **~248** | 0 |
| `#[c2rust::...]` attributes | 7,150 | **0** ✅ | 0 |
| `static mut` occurrences | 430 | 430 | 0 |

**Lines saved so far: ~22,000+**

### Latest Changes (Session 5)
- ✅ **Consolidated additional modules**:
  - `objects_h` (15 files) - Added Slab opaque type to types.rs
  - `takeover_h`, `admin_h`, `janitor_h`, `pooler_h` (16 files total)
  - `server_h`, `client_h`, `stats_h`, `loader_h`, `system_h` (16 files total)
  - `unistd_h` (6 files), `scram_h` (4 files), `un_h` (3 files)
  - `stdbool_h`, `postgres_compat_h`, `_size_t_h`, `_types_h`, etc.

- ✅ Added to `types.rs`:
  - `Slab` opaque type
  - `slab_init_fn`, `slab_stat_fn`, `pooler_cb`, `str_cb` function pointer types
  - `PasswordType` and `PASSWORD_TYPE_*` constants
  - `HIGHBIT`, `MaxAllocSize`, `INT_MAX` constants

### Previous Changes (Session 4)
- ✅ **Consolidated major modules** (~7,200 lines removed):
  - `bouncer_h` (22 files) - Core types (PgSocket, PgPool, PgDatabase, etc.)
  - `sbuf_h` (20 files) - SBuf, SBufIO, sbuf_cb_t, SBufEvent + inline functions
  - `iobuf_h` (20 files) - iobuf/IOBuf + inline functions
  - `pktbuf_h` (20 files) - PktBuf, PktHdr types
  - `dnslookup_h` (21 files) - DNSContext, DNSToken, adns_* types
  - `netdb_h` (3 files) - addrinfo struct

- ✅ Created rewrite scripts for module consolidation:
  - `scripts/rewrite_bouncer_h.py` - Specialized for bouncer_h
  - `scripts/rewrite_sbuf_h.py` - Specialized for sbuf_h with inline functions
  - `scripts/rewrite_iobuf_h.py` - Specialized for iobuf_h with inline functions
  - `scripts/rewrite_h_module.py` - General purpose for any module

- ✅ Added to `types.rs`:
  - iobuf_empty, iobuf_amount_pending, iobuf_amount_parse (inline functions)
  - sbuf_op_send (inline function)
  - addrinfo, AI_PASSIVE, freeaddrinfo, gai_strerror, getaddrinfo
  - adns_callback_f, adns_walk_name_f, adns_walk_zone_f
  - CfLookup, HBA, DEFAULT_UNIX_SOCKET_DIR

### Previous Changes (Session 3)
- ✅ Consolidated `errno_h` from 14 files

## Priority: Remaining Duplicate Modules

| Module | Files | Complexity | Notes |
|--------|-------|------------|-------|
| `bouncer_h` | 22 | ✅ Done | Now uses pub use crate::types::* |
| `dnslookup_h` | 21 | ✅ Done | Now uses pub use crate::types::* |
| `sbuf_h` | 20 | ✅ Done | Now uses pub use crate::types::* |
| `pktbuf_h` | 20 | ✅ Done | Now uses pub use crate::types::* |
| `iobuf_h` | 20 | ✅ Done | Now uses pub use crate::types::* |
| `objects_h` | 15 | ✅ Done | Now uses pub use crate::types::* |
| `util_h` | 11 | 🔴 Blocked | References cfparser_h::CfValue |
| `protocol_h` | 10 | 🔴 Blocked | Constants have conflicting types |
| Others | ~120 | 🟢 Low | Various smaller modules |

## Quick Commands

```bash
# Rewrite module to use types.rs (preserves inline functions and externs)
python3 scripts/rewrite_h_module.py MODULE_NAME

# Specific module scripts
python3 scripts/rewrite_bouncer_h.py
python3 scripts/rewrite_sbuf_h.py
python3 scripts/rewrite_iobuf_h.py

# Build and test
cargo build && cd test && pytest --timeout=120
```

## Rewrite Script Strategy

The rewrite scripts:
1. Add `pub use crate::types::*;` at the beginning of the module
2. Remove struct/type definitions (moved to types.rs)
3. Remove const definitions (moved to types.rs)
4. Remove use statements for types.rs (now handled by pub use)
5. **Preserve inline functions** (they use the types from types.rs)
6. **Preserve extern "C" blocks** (but strip `pub type Name;` declarations)

This approach maintains backward compatibility - code using `super::bouncer_h::PgSocket` still works because the module re-exports `crate::types::PgSocket`.

## Next Steps

1. **Consolidate protocol_h** — 🔴 Blocked: Constants have conflicting types (`c_int` vs `c_uint` vs `uint32_t`)
2. **Consolidate util_h** — 🔴 Blocked: References `cfparser_h::CfValue` creating type conflicts
3. **Consolidate cfparser_h** — 🔴 Blocked: Complex types (`CfContext`, `CfKey`, `CfOps`, `CfSect`)
4. **Consolidate objects_h** — 🟡 Possible: Mostly extern declarations, needs `Slab` opaque type
5. **Convert static mut** — Blocked until modules fully migrated to Rust

### Blockers Found

The rewrite approach works well for modules with:
- Uniform type definitions across files
- No conflicting types (same name, different underlying type)
- Inline functions that can reference `types.rs`

It does **not** work for:
- `protocol_h`: `PqMsg_*` constants defined as both `c_int` and `c_uint`
- `util_h`/`cfparser_h`: Cross-module type dependencies

## Refactoring Phases

| Phase | Modules | Status |
|-------|---------|--------|
| 1 | Core types consolidated | ✅ Done |
| 2 | main.rs, client.rs, server.rs | 🟡 Types consolidated |
| 3 | pooler.rs, sbuf.rs | 🟡 Types consolidated |
| 4 | proto.rs, admin.rs, messages.rs | 🟡 Types consolidated |
| 5 | scram.rs, hba.rs | 🟡 Types consolidated |
| 6 | objects.rs, loader.rs, janitor.rs | 🟡 Types consolidated |
| 7 | static mut conversion | 🔴 Pending |

## Documentation

| File | Contents |
|------|----------|
| [CHANGELOG.md](./CHANGELOG.md) | Session-by-session progress |
| [guidelines.md](./guidelines.md) | Type migration rules and patterns |
| [scripts.md](./scripts.md) | Script documentation |
| [phase-*.md](./phase-1-core.md) | Per-phase detailed notes |

## References

- `.cursor/rules/codebase-structure.mdc` — Module layout
- `.cursor/rules/unsafe-patterns.mdc` — Refactoring patterns
- `.cursor/rules/testing.mdc` — Testing guide
