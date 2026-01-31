# Refactoring Changelog

Session-by-session progress log for the PgBouncer Rust refactoring effort.

## 2026-01-31: Module Consolidation (Session 3)

### Modules Consolidated

| Module | Files | Notes |
|--------|-------|-------|
| `errno_h` | 14 | Added ECONNABORTED, EINVAL, EIO, ENOENT, ENOSYS, ESRCH |

### Metrics Change
- Duplicate modules: ~290 → ~278 (down 12)

### Attempted Consolidations (Failed/Reverted)
- `util_h` — Extern functions reference types not in types.rs (CfValue, addrinfo, PgSocket)
- `protocol_h` — Duplicate constants with different types (c_int vs uint32_t)
- `unistd_h` — Type mismatches in getpeereid signature
- `ctype_h` — Script didn't detect inline functions

### Findings
The `merge_module.py` script works well for simple modules with only constants/type aliases,
but has issues with:
1. Constants defined with different types across files
2. Extern functions referencing types not yet in types.rs
3. Inline function detection

The `bouncer_h` consolidation remains the critical blocker. Types like PgSocket, PgPool,
PgDatabase need to be in types.rs before other modules can reference them from types.rs.

---

## 2026-01-31: Module Consolidation (Session 2)

### Scripts Created
- `scripts/merge_module.py` — Collects and merges all items from all instances of a module
- `scripts/remove_module.py` — Removes modules when types already exist in types.rs

### Modules Consolidated

| Module | Files | Notes |
|--------|-------|-------|
| `tls_h` | 20 | 44 symbols merged |
| `logging_h` | 23 | |
| `socket_h` | 22 | |
| `in_h` | 21 | |
| `in6_h` | 21 | Fixed `__u6_addr` → `s6_addr` |
| `_string_h` | 23 | |
| `_stdlib_h` | 21 | |
| `_malloc_h` | 17 | |
| `safeio_h` | 6 | |
| `usual_socket_h` | 7 | |
| `event_struct_h` | 21 | Added C2RustUnnamed type aliases |
| `event_h` | 21 | |
| `_stdio_h` | 12 | Changed FILE to libc::FILE |

### Platform-Specific Migrations
- ✅ Replaced Darwin `__u6_addr.__u6_addr8` → portable `s6_addr`
- ✅ Replaced Darwin `__stderrp` → portable `libc::write(libc::STDERR_FILENO, ...)`
- ✅ Changed `FILE` from custom `__sFILE` → `libc::FILE`
- ✅ Removed `__stderrp`, `__stdoutp`, `__stdinp` from types.rs

### C2RustUnnamed Type Aliases Added

| C2RustUnnamed | Named Type |
|---|---|
| `C2RustUnnamed_0` | `event_union` |
| `C2RustUnnamed_1` | `event_signal` |
| `C2RustUnnamed_2` | `event_signal_next` |
| `C2RustUnnamed_3` | `event_io` |
| `C2RustUnnamed_4` | `event_io_next` |
| `C2RustUnnamed_5` | `event_timeout_pos` |
| `C2RustUnnamed_6` | `event_next_with_common_timeout` |
| `C2RustUnnamed_7` | `event_callback_union` |
| `C2RustUnnamed_8` | `event_callback_active_next` |

### Metrics Change
- Lines: 56,580 → ~49,850 (down ~6,700)
- Duplicate modules: 521 → ~290 (down ~230)

---

## 2026-01-30: Cleanup Patterns (Session 1)

### Commits
8 commits applying automated and manual cleanup patterns.

### Transformations Applied
1. Byte string casts: `b"...\0" as *const...` → `c"...".as_ptr()` (~2,000 changes)
2. Boolean patterns: `true_0 != 0` → `true` (1,200+ changes)
3. Verbose bool checks: `(expr) as c_int as c_long != 0` → `expr` (200+ changes)
4. Offset removal: `.offset(0 as c_int as isize)` → removed (150+ changes)
5. Manual bool simplification: `bool_fn() as c_int != 0` → `bool_fn()` (60+ changes)

### Scripts Created
- `scripts/cleanup_functions.py` — Automated cleanup transformations
- `scripts/convert_static_mut.py` — Analysis tool for static mut conversion

### Key Finding
`static mut` conversion is blocked because all variables are shared between C and Rust code. Must migrate modules fully to Rust before converting.

---

## Earlier: c2rust Attribute Removal

### Completed
- Removed all `#[c2rust::src_loc = "..."]` attributes (7,150 instances)
- Removed all `#[c2rust::header_src = "..."]` attributes

### Consolidated Types (Early Sessions)
- PgStats, List/StatList, AATree, MBuf
- proto_h (PktHdr), prepare_h, strpool_h
- statlist_h, aatree_h, time_h, uthash_h
- cryptohash_h, stdbool_h, varcache_h
- _types_h, sys__types_h, type wrapper modules
