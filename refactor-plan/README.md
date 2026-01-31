# PgBouncer Rust Refactoring Plan

Refactoring the c2rust-translated PgBouncer codebase into idiomatic, safe Rust.

## Goals

1. **Make business logic safe first** — prioritize higher-level modules
2. **Keep behavior equivalent** — no functional changes during refactoring
3. **Enable future swaps** — lower-level details (crypto, TLS) can be replaced with Rust crates later

## Current State (2026-01-31)

| Metric | Original | Current | Target |
|--------|----------|---------|--------|
| Total Rust lines (src/*.rs) | ~126,000 | **~49,850** | <30,000 |
| Duplicate `pub mod *_h` modules | ~800 | **~278** | 0 |
| `#[c2rust::...]` attributes | 7,150 | **0** ✅ | 0 |
| `static mut` occurrences | 430 | 430 | 0 |

**Lines saved so far: ~15,000+**

### Latest Changes
- ✅ Consolidated `errno_h` from 14 files (EAGAIN, EINTR, etc. use libc; added ECONNABORTED, EINVAL, EIO, ENOENT, ENOSYS, ESRCH)

## Priority: Remaining Duplicate Modules

| Module | Files | Complexity | Notes |
|--------|-------|------------|-------|
| `bouncer_h` | 22 | 🔴 High | Core types + extern statics |
| `dnslookup_h` | 21 | 🟡 Medium | addrinfo varies |
| `sbuf_h` | 20 | 🔴 High | Inline functions |
| `pktbuf_h` | 20 | 🔴 High | Inline functions |
| `iobuf_h` | 20 | 🔴 High | Extern static deps |
| Others | ~187 | 🟢 Low | Various smaller modules |

## Quick Commands

```bash
# Merge module (collects all items, adds to types.rs)
python3 scripts/merge_module.py MODULE_NAME --apply

# Remove module (when types already in types.rs)
python3 scripts/remove_module.py MODULE_NAME --apply

# Build and test
cargo build && cd test && pytest --timeout=120
```

## Next Steps

1. **Consolidate bouncer_h** — Requires manual approach: add types to types.rs, keep extern statics in each file
2. **Fix merge_module.py** — Script has issues with duplicate constants and cross-file type references
3. **Consolidate simpler modules** — Only modules with uniform definitions across files work well
4. **Convert static mut** — Blocked until modules fully migrated to Rust

## Script Limitations Found

The `merge_module.py` script has limitations:
- Doesn't deduplicate constants with same name but different types (e.g., `c_int` vs `uint32_t`)
- Adds extern function declarations that reference types not yet in types.rs
- Doesn't handle modules that mix types with extern statics well

**Recommended approach for complex modules**: Manual migration of types to types.rs, then script removal of local type definitions.

## Refactoring Phases

| Phase | Modules | Status |
|-------|---------|--------|
| 1 | main.rs, client.rs, server.rs | 🟡 Types pending |
| 2 | pooler.rs, sbuf.rs | 🟡 Types pending |
| 3 | proto.rs, admin.rs, messages.rs | 🟡 Types pending |
| 4 | scram.rs, hba.rs | 🟡 Types pending |
| 5 | objects.rs, loader.rs, janitor.rs | 🟡 Types pending |
| 6 | src/common/, lib/usual/ | 🟡 Types pending |

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
