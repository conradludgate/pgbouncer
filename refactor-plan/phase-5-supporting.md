# Phase 5: Supporting Modules

Core data structures and supporting functionality.

## Current Metrics (2026-01-30)

| Module | Lines | Unsafe Fns | #[no_mangle] | Static Mut | Duplicate Modules |
|--------|-------|------------|--------------|------------|-------------------|
| objects.rs | 6,200 | 129 | 76 | 46 | 39 |
| loader.rs | 2,436 | 32 | 7 | 9 | 35 |
| janitor.rs | 3,300 | 48 | 10 | 40 | 24 |
| varcache.rs | 2,500 | 32 | 10 | 4 | 25 |
| prepare.rs | 2,600 | 26 | 9 | 6 | 21 |
| stats.rs | 1,656 | 19 | 5 | 7 | 18 |
| pktbuf.rs | 2,500 | 36 | 20 | 3 | 23 |
| dnslookup.rs | 1,800 | 49 | 10 | 6 | 19 |
| takeover.rs | 3,000 | 24 | 4 | 6 | 38 |
| system.rs | 354 | 3 | 3 | 0 | 12 |
| util.rs | 2,900 | 41 | 20 | 13 | 35 |

## objects.rs — The Core

### Overview
Central data structures used by everything:
- `PgSocket` — client or server connection
- `PgDatabase` — database configuration
- `PgPool` — connection pool
- `PgUser` — user credentials
- Global lists: `database_list`, `pool_list`, `user_list`

### This is Critical
129 unsafe functions, 76 `#[no_mangle]`, 46 `static mut`. Almost every other module depends on this.

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules (depends on bouncer_h)
3. Keep struct layouts compatible with C initially
4. Convert global lists to thread-local RefCell
5. Eventually redesign with proper Rust ownership

### Caution
Changes here affect everything. Move slowly, test frequently.

## loader.rs

### Overview
Configuration file parsing:
- INI file parsing
- Database/user configuration
- Config validation

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules
3. Define proper config types
4. Parsing can be made safe
5. Consider using a config parsing crate eventually

## janitor.rs

### Overview
Background maintenance:
- Connection cleanup
- Statistics aggregation
- Health checks
- Idle connection management

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules
3. Many globals (40 static mut) — careful conversion
4. Timer-based logic can be made safer

## stats.rs ✅ CLEANUP DONE

### Overview
Statistics collection. Mostly counters.

### Status
Function cleanup patterns applied and documented in `findings-stats-rs.md`.
This is the **reference module** for cleanup patterns.

## Other Modules

### varcache.rs
PostgreSQL parameter caching. Relatively self-contained.

### prepare.rs
Prepared statement tracking. Important for statement pooling mode.

### pktbuf.rs
Packet buffer utilities. Low-level but bounded scope.

### dnslookup.rs
DNS resolution. FFI to system resolver.

### takeover.rs
Online restart / socket migration. Complex but rarely touched.

### system.rs
System utilities. Small module (354 lines).

### util.rs
Misc utilities. Contains logging helpers.

## Checklist

### Completed ✅
- [x] All modules: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] All modules: Consolidate PgStats type to common/types.rs
- [x] All modules: Consolidate List, StatList types to common/types.rs
- [x] All modules: Consolidate AATree types to common/types.rs
- [x] stats.rs: Clean up function implementations (removed wrapping arithmetic, simplified code)
- [x] stats.rs: Document cleanup patterns in findings-stats-rs.md

### Blocked 🚧 (Waiting on Type Consolidation)
- [ ] **Consolidate bouncer_h** first (Phase 1 blocker)
- [ ] objects.rs has 39 duplicate modules and defines core types

### Pending 📋 (After Type Consolidation)
- [ ] objects.rs: Remove remaining type modules (~39 modules)
- [ ] objects.rs: Import all types from crate::types::*
- [ ] objects.rs: Document struct invariants
- [ ] objects.rs: Convert global lists to thread-local (46 static mut)
- [ ] loader.rs: Remove remaining type modules (~35 modules), safe parsing
- [ ] janitor.rs: Remove type modules, convert globals (40 static mut)
- [ ] varcache.rs: Remove remaining type modules (~25 modules)
- [ ] prepare.rs: Remove remaining type modules (~21 modules)
- [ ] pktbuf.rs: Remove remaining type modules (~23 modules)
- [ ] dnslookup.rs: Remove remaining type modules (~19 modules)
- [ ] takeover.rs: Remove remaining type modules (~38 modules)
- [ ] system.rs: Remove remaining type modules (~12 modules)
- [ ] util.rs: Remove remaining type modules (~35 modules)
- [ ] All: Apply function cleanup patterns from stats.rs

### Notes

**stats.rs is the reference implementation:**
- Cleanup patterns documented in `findings-stats-rs.md`
- Apply same patterns to other modules

**objects.rs is critical path:**
- Defines types used everywhere
- Changes affect all other modules
- Move very carefully, test after every change

**Function Cleanup Patterns (from stats.rs):**
```rust
// Wrapping arithmetic
.wrapping_add(x)  →  += x

// String literals  
b"str\0" as *const u8 as *const c_char  →  c"str".as_ptr()

// Redundant mut
mut param: *mut T  →  param: *mut T  (when not reassigned)

// Type annotations
0 as usec_t  →  0
```
