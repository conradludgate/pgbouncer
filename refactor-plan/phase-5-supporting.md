# Phase 5: Supporting Modules

Core data structures and supporting functionality.

## Modules

| Module | Unsafe Fns | #[no_mangle] | Static Mut | Lines |
|--------|------------|--------------|------------|-------|
| objects.rs | 149 | 76 | 46 | ~6,200 |
| loader.rs | 34 | 7 | 9 | ~2,900 |
| janitor.rs | 57 | 10 | 40 | ~3,300 |
| varcache.rs | 34 | 10 | 4 | ~2,500 |
| prepare.rs | 32 | 9 | 6 | ~2,600 |
| stats.rs | 25 | 5 | 7 | ~2,100 |
| pktbuf.rs | 38 | 20 | 3 | ~2,500 |
| dnslookup.rs | 61 | 10 | 6 | ~1,800 |
| takeover.rs | 32 | 4 | 6 | ~3,000 |
| system.rs | 3 | 3 | - | ~600 |
| util.rs | 43 | 20 | 13 | ~2,900 |

## objects.rs — The Core

### Overview
Central data structures used by everything:
- `PgSocket` — client or server connection
- `PgDatabase` — database configuration
- `PgPool` — connection pool
- `PgUser` — user credentials
- Global lists: `database_list`, `pool_list`, `user_list`

### This is Critical
149 unsafe functions, 76 `#[no_mangle]`, 46 `static mut`. Almost every other module depends on this.

### Strategy
1. Clean up c2rust artifacts first
2. Keep struct layouts compatible with C initially
3. Convert global lists to thread-local RefCell
4. Eventually redesign with proper Rust ownership

### Caution
Changes here affect everything. Move slowly, test frequently.

## loader.rs

### Overview
Configuration file parsing:
- INI file parsing
- Database/user configuration
- Config validation

### Strategy
1. Clean up c2rust artifacts
2. Define proper config types
3. Parsing can be made safe
4. Consider using a config parsing crate eventually

## janitor.rs

### Overview
Background maintenance:
- Connection cleanup
- Statistics aggregation
- Health checks
- Idle connection management

### Strategy
1. Clean up c2rust artifacts
2. Many globals (40 static mut) — careful conversion
3. Timer-based logic can be made safer

## Other Modules

### varcache.rs
PostgreSQL parameter caching. Relatively self-contained.

### prepare.rs
Prepared statement tracking. Important for statement pooling mode.

### stats.rs
Statistics collection. Mostly counters — easy to make safe.

### pktbuf.rs
Packet buffer utilities. Low-level but bounded scope.

### dnslookup.rs
DNS resolution. FFI to system resolver.

### takeover.rs
Online restart / socket migration. Complex but rarely touched.

### system.rs
System utilities. Small module.

### util.rs
Misc utilities. Contains logging helpers.

## Checklist

### Completed ✅
- [x] All modules: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] All modules: Consolidate PgStats type to common/types.rs
- [x] All modules: Consolidate List, StatList types to common/types.rs
- [x] All modules: Consolidate AATree types to common/types.rs
- [x] stats.rs: Clean up function implementations (removed wrapping arithmetic, simplified code)

### Pending 📋
- [ ] objects.rs: Remove remaining c2rust type modules
- [ ] objects.rs: Import all types from common/types.rs
- [ ] objects.rs: Document struct invariants
- [ ] objects.rs: Convert global lists to thread-local
- [ ] loader.rs: Remove remaining c2rust type modules, safe parsing
- [ ] janitor.rs: Convert globals, safe logic
- [ ] varcache.rs: Remove remaining c2rust type modules
- [ ] prepare.rs: Remove remaining c2rust type modules
- [ ] stats.rs: Remove remaining c2rust type modules
- [ ] pktbuf.rs: Remove remaining c2rust type modules
- [ ] dnslookup.rs: Remove remaining c2rust type modules
- [ ] takeover.rs: Remove remaining c2rust type modules
- [ ] system.rs: Remove remaining c2rust type modules
- [ ] util.rs: Remove remaining c2rust type modules

### Notes
- stats.rs was used as proof-of-concept for function cleanup patterns
- See `findings-stats-rs.md` for detailed refactoring learnings
