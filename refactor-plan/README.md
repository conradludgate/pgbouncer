# PgBouncer Rust Refactoring Plan

This document outlines the strategy for refactoring the c2rust-translated PgBouncer codebase into idiomatic, safe Rust.

## Goals

1. **Make business logic safe first** — prioritize higher-level modules
2. **Keep behavior equivalent** — no functional changes during refactoring
3. **Enable future swaps** — lower-level details (crypto, TLS) can be replaced with Rust crates later

## Approach: Top-Down

We start from the entry point (`main.rs`) and core connection handling (`client.rs`, `server.rs`), working down through the dependency graph. This ensures:

- Business logic becomes safe before implementation details
- We understand the full flow before touching utilities
- Lower-level modules become swappable implementation details

## Progress Summary

| Milestone | Status |
|-----------|--------|
| Remove c2rust attributes from src/ | ✅ Complete |
| Remove c2rust attributes from lib/usual/ | ✅ Complete |
| Consolidate PgStats type | ✅ Complete |
| Consolidate List/StatList types | ✅ Complete |
| Consolidate AATree types | ✅ Complete |
| Consolidate MBuf type | ✅ Complete |
| Consolidate proto_h (PktHdr) type | ✅ Complete |
| Consolidate prepare_h (prepared statement) types | ✅ Complete |
| Consolidate strpool_h types | ✅ Complete |
| Consolidate statlist_h types | ✅ Complete |
| Consolidate aatree_h types | ✅ Complete |
| Consolidate time_h types | ✅ Complete |
| Consolidate uthash_h types | ✅ Complete |
| Consolidate cryptohash_h types | ✅ Complete |
| Consolidate stdbool_h types | ✅ Complete |
| Consolidate varcache_h types | ✅ Complete |
| Remove list_h re-export modules | ✅ Complete |
| Consolidate C type alias modules | ✅ Complete |
| **Consolidate _types_h module** | ✅ Complete (23 files, -470 lines) |
| **Consolidate sys__types_h module** | ✅ Complete (26 files, -350 lines) |
| **Consolidate type wrapper modules** | ✅ Complete (_gid_t_h, _socklen_t_h, etc., 13 files, -241 lines) |
| **Consolidate bouncer_h types** | 🔴 **Complex** (22 files, needs manual approach) |
| **Consolidate iobuf_h types** | 🔴 **Complex** (20 files, has extern static deps) |
| **Consolidate sbuf_h types** | 🔴 **Not Started** (20 files) |
| Remove primitive type modules | 🟡 **Partial** (530 remaining) |
| Apply function cleanup patterns | ✅ **Complete** (11,000+ changes) |
| Convert static mut to RefCell | 🔴 **Blocked** (requires C→Rust module migration) |

**Lines saved from refactoring: ~10,400+** (type consolidation + cleanup patterns)

### Recent Session (2026-01-31): Variable Content Module Consolidation

**Created `scripts/merge_module.py`** — a smart consolidation script that:
- Collects ALL items from ALL instances of a module across all files
- Merges types, constants, functions into a single definition
- Detects existing symbols to avoid duplicates
- Handles `super::module_name::` patterns inside nested modules

**Successfully consolidated variable content modules:**
| Module | Files | Lines Saved |
|--------|-------|-------------|
| `tls_h` | 20 | ~200 |
| `logging_h` | 23 | ~760 |
| `socket_h` | 22 | ~410 |
| `in_h` | 21 | ~480 |
| `_string_h` | 23 | ~610 |
| `_stdlib_h` | 21 | ~180 |
| `_malloc_h` | 17 | ~160 |
| `safeio_h` | 6 | ~70 |
| `usual_socket_h` | 7 | ~65 |

**Total lines saved this session: ~2,935 lines** (net: -2,235 lines after adding to types.rs)

**Modules needing manual migration:**
- `in6_h` — ✅ **DONE** - Replaced Darwin `__u6_addr` with portable `s6_addr`
- `event_h` / `event_struct_h` — See "C2RustUnnamed Type Migration" below
- `_stdio_h` — See "Portable stderr Migration" below
- `dnslookup_h` — Use `libc::addrinfo`
- `protocol_h` — Deduplicate auth constants

### C2RustUnnamed Type Migration

The c2rust translation created anonymous types (`C2RustUnnamed*`) for C's anonymous unions/structs. These need to be mapped to named types already defined in `types.rs`:

| C2RustUnnamed | Named Type | Source (libevent) |
|---|---|---|
| `C2RustUnnamed` (in event) | `event_union` | union of ev_io/ev_signal |
| `C2RustUnnamed_0` | `event_signal` | signal event data |
| `C2RustUnnamed_1` | `event_signal_next` | `LIST_ENTRY(event)` |
| `C2RustUnnamed_2` | `event_io` | io event data |
| `C2RustUnnamed_3` | `event_io_next` | `LIST_ENTRY(event)` |
| `C2RustUnnamed_4` | `event_io_next` | `LIST_ENTRY(event)` (alias) |
| `C2RustUnnamed_5` | `event_timeout_pos` | union with TAILQ_ENTRY or int |
| `C2RustUnnamed_6` | `event_next_with_common_timeout` | `TAILQ_ENTRY(event)` |
| `C2RustUnnamed_7` | `event_callback_union` | callback function union |
| `C2RustUnnamed_8` | `event_callback_struct` | callback struct |

**Migration steps:**
1. Add type aliases in `types.rs`: `pub type C2RustUnnamed_4 = event_io_next;`
2. Gradually update struct definitions to use named types
3. Remove the aliases once all usages are updated

### Portable stderr Migration

The Darwin-specific `__stderrp` should be replaced with portable file descriptor writes:

**Current (Darwin-specific):**
```rust
extern "C" {
    #[link_name = "__stderrp"]
    pub static mut __stderrp: *mut FILE;
}
fprintf(__stderrp, "error: %s\n", msg);
```

**Target (portable):**
```rust
use libc::{write, STDERR_FILENO};

// Direct write to stderr file descriptor
unsafe {
    libc::write(libc::STDERR_FILENO, msg.as_ptr() as *const c_void, msg.len());
}
```

**Migration steps:**
1. Create a portable `write_stderr()` helper function
2. Replace `fprintf(__stderrp, ...)` calls with the helper or direct `libc::write()`
3. Remove `__stderrp` from types.rs
4. Consolidate `_stdio_h` modules

### Type Migration Guidelines

1. **Replace Darwin-specific types with portable definitions** — All Darwin-specific types (`__darwin_*`, `__u6_addr`, etc.) are temporary from c2rust and must be replaced with POSIX/portable equivalents.

2. **Prefer specific struct definitions over opaque types** — Use full struct definitions when available, not just `extern { pub type X; }`.

3. **Use libc definitions when available** — Prefer `libc::in6_addr`, `libc::addrinfo`, etc. over custom definitions. Remove any duplicates.

4. **Update code that uses platform-specific fields** — When switching to libc types, update field accesses (e.g., `.__u6_addr.__u6_addr8` → `.s6_addr`).

**Key findings:**
- `iobuf_h` has inline functions depending on extern statics (`cf_sbuf_len`), making it complex to consolidate. Same issue as `bouncer_h`.
- **Module content varies across files** — SOLVED with `merge_module.py` which collects ALL items from all files and merges them.

### Remaining Duplicate Modules Analysis

After consolidation, remaining complex modules:

**2. Complex struct modules (need careful migration):**
- `bouncer_h` (22 files) — Core types, mixes types with extern statics
- `event_struct_h` (21 files) — libevent structures with C2RustUnnamed unions
- `sbuf_h` (20 files), `iobuf_h` (20 files), `pktbuf_h` (20 files) — Have inline functions

**3. Extern function modules (could consolidate if content was uniform):**
- `_string_h` (21), `_stdlib_h` (19), `_malloc_h` (15) — Declare libc functions
- But different files import different subsets of functions

**Recommended approach:** Manual per-file migration or use the existing consolidate_module.py with `--analyze` to understand each module's content before attempting consolidation.

### Previous Session (2026-01-30): Cleanup Patterns Applied

**8 commits** applying automated and manual cleanup:
1. Byte string casts: `b"...\0" as *const...` → `c"...".as_ptr()` (~2,000 changes)
2. Boolean patterns: `true_0 != 0` → `true` (1,200+ changes)
3. Verbose bool checks: `(expr) as c_int as c_long != 0` → `expr` (200+ changes)
4. Offset removal: `.offset(0 as c_int as isize)` → removed (150+ changes)
5. Manual bool simplification: `bool_fn() as c_int != 0` → `bool_fn()` (60+ changes)

**Scripts created:**
- `scripts/cleanup_functions.py` - Automated cleanup transformations
- `scripts/convert_static_mut.py` - Analysis tool for static mut conversion

**Key finding:** `static mut` conversion is blocked because all variables are shared between C and Rust code. Must migrate modules fully to Rust before converting.

## Current State Metrics (as of 2026-01-31)

| Metric | Original | Current | Target |
|--------|----------|---------|--------|
| Total Rust lines (src/*.rs) | ~126,000 | **56,580** | <30,000 |
| Total Rust lines (src/common/) | - | **58,376** | - |
| `static mut` occurrences (src/) | 430 | **430** | 0 |
| `unsafe extern "C" fn` (src/) | 1,787 | **1,632** | <500 |
| `#[no_mangle]` (src/) | 450 | **450** | <100 |
| `#[c2rust::...]` attributes | 7,150 | **0** ✅ | 0 |
| Duplicate `pub mod *_h` modules | ~800 | **521** | 0 |
| `as c_int != 0` patterns | ~100+ | **40** ✅ | ~40 (char comparisons) |

### Biggest Blocker: Duplicate Type Modules

Each file has 20-40+ `pub mod *_h { }` blocks duplicating types. This is ~5x code bloat:

| Module | Files Affected | Types Defined |
|--------|---------------|---------------|
| `bouncer_h` | 22 files | PgSocket, PgPool, PgDatabase, PgCredentials, etc. |
| `iobuf_h` | 20 files | IOBuf |
| `sbuf_h` | 20 files | SBuf, SBufIO |
| `_types_h` | 27 files | u8, u16, i32 aliases (unnecessary) |
| `socket_h` | 20+ files | sockaddr, msghdr |

**Consolidating these is the highest-impact next step.**

## Recommended Next Actions

### Priority Order

1. **Type Consolidation** - Remove duplicate `*_h` modules (highest code reduction impact)
2. **C→Rust Module Migration** - Pick one small module to fully migrate (enables `static mut` conversion)
3. **Continue Cleanup Patterns** - Run `cleanup_functions.py` on new files if added

### 1. Consolidate `bouncer_h` (HIGH IMPACT) — **COMPLEX**

This is the biggest win — PgSocket, PgPool, PgDatabase are the core types.

**⚠️ The consolidation script doesn't work directly for bouncer_h** because:
- The module contains BOTH type definitions AND extern static declarations (cf_* config variables)
- The script tries to import `cf_admin_users`, `adns`, etc. from types.rs, but those are extern declarations pointing to variables defined in main.rs
- Extern statics should stay in individual modules, not be moved to types.rs

**Correct approach:**
1. Add type definitions (structs, enums, type aliases) to `types.rs`
2. Leave extern declarations (`static mut cf_*`, `fn load_config()`, etc.) in each file
3. Update imports file-by-file to use `crate::types::PgSocket` etc.
4. Delete just the struct/enum definitions from bouncer_h, keep the extern block

**Manual process per file:**
```rust
// Before: everything in bouncer_h module
pub mod bouncer_h {
    pub struct PgSocket { ... }  // DELETE - use from types.rs
    pub struct PgPool { ... }    // DELETE - use from types.rs
    extern "C" {
        pub static mut cf_admin_users: *mut c_char;  // KEEP
        pub fn load_config() -> bool;                // KEEP
    }
}

// After: types from crate::types, externs at file level
use crate::types::{PgSocket, PgPool, PgDatabase, ...};
extern "C" {
    pub static mut cf_admin_users: *mut c_char;
    pub fn load_config() -> bool;
}
```

**Alternative**: Focus on smaller modules first (iobuf_h, pktbuf_h) to build momentum.

### 2. Consolidation Script Limitation (IMPORTANT FINDING)

**The consolidation script has a fundamental limitation**: It can't handle modules where inline functions depend on extern static variables.

Examples:
- `bouncer_h`: `first_socket()` uses statlist_empty, types from many modules, extern statics like `cf_*`
- `iobuf_h`: `iobuf_amount_recv()` depends on `cf_sbuf_len` (extern static from main.rs)
- `sbuf_h`: Similar dependencies

**Alternative approaches:**
1. **Focus on function cleanup first** (from stats.rs patterns) - this works reliably
2. **Remove primitive type modules** (`_types_h`, etc.) which have no dependencies
3. **Manual consolidation** for complex modules - move types but keep externs in place

### 3. Remove Primitive Type Modules (SIMPLER)

The `_types_h`, `sys__types_h`, `_socklen_t_h` modules just alias `u8`, `i32`, etc. Delete them and use `crate::types::*` instead. These have no function dependencies.

### 4. Apply Function Cleanup Patterns (RECOMMENDED NEXT STEP)

This is the safest and most reliable refactoring to do now. From `stats.rs` findings, apply these patterns across all files:

```rust
// Before → After
.wrapping_add(x)  →  += x
b"str\0" as *const u8 as *const c_char  →  c"str".as_ptr()
mut param: *mut T (when not mutated)  →  param: *mut T
0 as usec_t  →  0
::core::mem::zeroed() for struct initialization
```

This doesn't change module structure, so there's no risk of import mismatches.

### 5. Convert `static mut` to Thread-Local

Start with config variables in `main.rs` (134 `static mut`):

```rust
// Before
static mut cf_verbose: c_int = 0;

// After
thread_local! {
    static CF_VERBOSE: RefCell<i32> = RefCell::new(0);
}
```

## Scripts

Scripts can be created or improved to help with refactoring. See `scripts/` directory.

### consolidate_sys_types.py ✅

Removes `sys__types_h` modules and updates imports to `crate::types`:

```bash
python3 scripts/consolidate_sys_types.py --dry-run  # Preview
python3 scripts/consolidate_sys_types.py            # Apply
```

### consolidate_wrapper_modules.py ✅

Removes simple type wrapper modules (`_gid_t_h`, `_socklen_t_h`, etc.):

```bash
python3 scripts/consolidate_wrapper_modules.py --dry-run  # Preview
python3 scripts/consolidate_wrapper_modules.py            # Apply
```

### consolidate_module.py (Original)

Use `scripts/consolidate_module.py` to automate module consolidation:

```bash
# Analyze what's in a module across all files
python3 scripts/consolidate_module.py bouncer_h --analyze

# Extract canonical definitions to add to types.rs
python3 scripts/consolidate_module.py bouncer_h --extract

# Preview changes without modifying files
python3 scripts/consolidate_module.py bouncer_h --dry-run

# Apply changes (after adding missing types to types.rs)
python3 scripts/consolidate_module.py bouncer_h --force
```

**Prerequisites before running the script:**
1. Ensure all types exported via `pub use` exist in `src/common/types.rs`
2. The script handles extern "C" declarations by moving them to file end
3. Run `cargo build` after to verify compilation

**Known limitations (needs fix):**
- Tries to import extern statics (`cf_*`, `adns`, etc.) from types.rs
- Doesn't distinguish between type definitions and extern declarations
- Fails for modules like `bouncer_h`, `iobuf_h` that mix types with extern statics

### Script Improvements Needed

**consolidate_module.py enhancements:**
1. Distinguish symbol types:
   - Type definitions (struct, enum, type alias, const) → import from types.rs
   - Extern static declarations (`pub static mut`) → keep in file or move to file-level
   - Inline functions → check for extern dependencies before moving

2. Add `--types-only` flag to only consolidate type definitions, leaving externs in place

3. Add `--skip-statics` flag to not try importing `static mut` declarations

### Proposed New Scripts

**1. `scripts/cleanup_functions.py`** - Apply function cleanup patterns:
```bash
python scripts/cleanup_functions.py src/admin.rs --dry-run
```
Transformations:
- `.wrapping_add(x)` → `+= x`
- `.wrapping_sub(x)` → `-= x`
- `b"str\0" as *const u8 as *const c_char` → `c"str".as_ptr()`
- Remove redundant `mut` on pointer parameters
- `0 as usec_t` → `0`

**2. `scripts/remove_primitive_modules.py`** - Remove `_types_h`, `sys__types_h`, etc:
```bash
python scripts/remove_primitive_modules.py --dry-run
```
These modules just re-export `u8`, `i32`, etc. and have no dependencies.

**3. `scripts/analyze_module_deps.py`** - Categorize modules by complexity:
```bash
python scripts/analyze_module_deps.py bouncer_h
```
Output: Lists extern dependencies, flags as "simple" (can use script) or "complex" (needs manual work)

### Manual Consolidation Pattern

When consolidating duplicated type modules (e.g., `mbuf_h`), follow this order:
1. Add all inline functions to the consolidated module (`lib/usual/mbuf.rs`)
2. Update the top-level `pub use self::X_h::...` to use new paths
3. Update inner module `use super::X_h::...` statements to `use crate::types::...` or `use crate::lib::usual::X::...`
4. Delete the local `pub mod X_h { ... }` module
5. Build and fix any type mismatches

Key insight: Inner modules like `proto_h` define `PktHdr` which contains `MBuf`. These need their imports updated BEFORE removing the local `mbuf_h` module, otherwise you get type mismatches.

### Binary vs Library Imports

- `src/main.rs` is the **binary** entry point, not part of the library
- It uses `pgbouncer::types::MBuf` instead of `crate::types::MBuf`
- All other `src/*.rs` files are part of the library and use `crate::types::...`

### Type Migration Must Be Coordinated

If stats.rs uses shared types but admin.rs has local types:
- `stats::PgPool` ≠ `admin::bouncer_h::PgPool` 
- Rust sees these as different types!

**Solution**: Migrate all files sharing a type together.

## Phase Overview

| Phase | Modules | Focus | Status |
|-------|---------|-------|--------|
| 1 | main.rs, client.rs, server.rs | Entry point & core connection handling | 🟡 Attributes done |
| 2 | pooler.rs, sbuf.rs | Connection pooling & I/O | 🟡 Attributes done |
| 3 | proto.rs, admin.rs, messages.rs | Protocol & admin console | 🟡 Attributes done |
| 4 | scram.rs, hba.rs | Authentication | 🟡 Attributes done |
| 5 | objects.rs, loader.rs, janitor.rs, etc. | Supporting modules | 🟡 Attributes done |
| 6 | src/common/, lib/usual/ | Low-level (defer or replace with crates) | 🟡 Attributes done |

**All phases**: c2rust attributes removed. Next step is type module consolidation.

## Per-Module Refactoring Steps

For each module, follow these steps in order:

### Step 1: Remove c2rust Artifacts ✅ DONE
- ~~Delete `#[c2rust::src_loc = "..."]` attributes~~
- ~~Delete `#[c2rust::header_src = "..."]` attributes~~

### Step 2: Consolidate Type Modules 🔴 CURRENT PRIORITY
- Remove `pub mod *_h { }` blocks
- Import from `crate::types::*` instead
- Use consolidation script for bulk changes

### Step 3: Clean Up Function Code 🟡 IN PROGRESS
- Remove `extern "C"` from internal functions
- Simplify wrapping arithmetic
- Use C string literals
- Remove redundant casts

### Step 4: Convert Static Mutable State
- Replace `static mut` with thread-local RefCell
- Group related config into structs

### Step 5: Convert Function Signatures
- Raw pointers → references where safe
- Return codes → `Result<T, Error>`
- Boolean ints → `bool`

### Step 6: Handle FFI Boundary
- Keep `#[no_mangle]` on functions called from C
- These are converted last, after their C callers are also in Rust

## Testing Strategy

Run integration tests after each significant change:

```bash
cargo build && cp target/debug/pgbouncer . && \
  cd test && pytest test_admin.py -v --timeout=120
```

Expand to full test suite periodically:
```bash
cd test && pytest --timeout=120
```

If a test fails, the refactoring likely changed behavior (unintended).

## Files

- [Phase 1: Core Modules](./phase-1-core.md)
- [Phase 2: Pooling & I/O](./phase-2-pooling.md)
- [Phase 3: Protocol & Admin](./phase-3-protocol.md)
- [Phase 4: Authentication](./phase-4-auth.md)
- [Phase 5: Supporting Modules](./phase-5-supporting.md)
- [Phase 6: Low-Level](./phase-6-lowlevel.md)

## References

- `.cursor/rules/codebase-structure.mdc` — Module layout and dependencies
- `.cursor/rules/unsafe-patterns.mdc` — Common patterns and how to refactor them
- `.cursor/rules/testing.mdc` — How to run tests
