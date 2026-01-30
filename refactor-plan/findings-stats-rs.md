# stats.rs Refactoring Findings

## Overview

Successfully refactored `stats.rs` function implementations while preserving the type module structure.

## Results

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Lines  | 2,020  | 1,942 | -78 (-4%) |
| Insertions | - | 79 | - |
| Deletions | - | 157 | - |

**All tests pass** including `test_show_stats` and `test_show`.

## File Structure

```
stats.rs: 2,020 lines → 1,942 lines
stats.c:    433 lines (original)

Breakdown:
- Lines 1-1126:  Type module boilerplate (~58% - unchanged)
- Lines 1127-1172: Static variables  
- Lines 1173-1942: Actual functions (~42% - cleaned up)
```

The Rust file is ~5x larger than the C original, almost entirely due to type duplication.

## Module Structure Discovery

Each `.rs` file in `src/` is **self-contained** with its own complete set of type definitions:

```rust
// Every file starts with these modules:
pub mod _types_h { ... }      // u8, u16, i32, etc.
pub mod _uintptr_t_h { ... }
pub mod sys__types_h { ... }
pub mod _in_addr_t_h { ... }
// ... 50+ more type modules ...
pub mod bouncer_h { ... }     // PgPool, PgSocket, etc.
```

This is a **bug** from c2rust translation:
1. Types are duplicated in every file (wasteful)
2. Types with same name/layout **should be** the same Rust type
3. Current state prevents proper cross-module function calls

## Type Consolidation Strategy

**Goal**: All files should import from a single `src/common/types.rs` instead of defining their own type modules. The duplicate types are a c2rust artifact to fix.

### Approach

1. **Extend `src/common/types.rs`** with all needed types (PgStats, PgPool, PgSocket, etc.)
2. **For each src/*.rs file**:
   - Delete the `pub mod *_h { }` type modules at the top
   - Add `use crate::src::common::types::*;` 
3. **Test after each file** to catch breakage early

### Why This Works

- All types with the same name/layout **should be** the same Rust type
- The current duplication is a bug, not a feature
- Once consolidated, cross-module function calls will use consistent types

### Type Consolidation Results

**Multiple types consolidated successfully!**

| Type | Before | After | Lines Saved |
|------|--------|-------|-------------|
| PgStats | 20 defs | 1 def | ~337 |
| List + StatList | 22 defs | 1 def | ~523 |
| AATree + AANode | 21 defs | 1 def | ~522 |
| **Total** | - | - | **~1,382 lines** |

All tests pass.

Next targets for consolidation:
- `MBuf` - 20 definitions (has inline functions)
- `PgPool` - ~20 definitions (complex, has bitfields)
- `PgSocket` - ~20 definitions (complex, has bitfields)
- Other `bouncer_h` types

---

## Technical Notes: Type Module Structure

### Current State (Bug to Fix)

```
pgbouncer_lib.rs (library crate)
├── pub mod lib { pub mod usual { ... } }
├── pub mod src { pub mod admin; pub mod stats; ... }
└── pub use src::common::types;

src/main.rs (binary crate)
├── Includes full type modules inline
├── Compiles independently
└── Uses: use ::pgbouncer; (imports library)
```

When compiling `main.rs`:
- It has its own type modules
- It imports `::pgbouncer` from the library
- Types from library ≠ types defined locally (different Rust paths)

### Problem 2: Incremental Migration Challenge

If we migrate stats.rs to use shared types but admin.rs still has local types:
- `stats::PgPool` (from shared types) ≠ `admin::bouncer_h::PgPool` (local)
- Rust sees these as different types

**Solution**: Migrate all files that share a type together, or migrate all at once.

### Problem 3: extern "C" Functions

Functions like `admin_database_stats()` are `#[no_mangle] extern "C"`:
- They're called from C code via FFI
- Parameter types must match C expectations
- Changing type paths could break FFI

## What Would Work

### Option A: Gradual Migration (Recommended)

1. Keep type modules in each file for now
2. Clean up just the function implementations
3. Eventually create a shared `types.rs` that ALL files import
4. Do a bulk migration once types.rs is complete

### Option B: Use Type Aliases

In each file, add at the top:
```rust
// Type aliases pointing to a canonical location
use crate::types::PgPool;  // instead of local bouncer_h::PgPool
```

But this requires `main.rs` to also use the same canonical path.

### Option C: Remove Type Modules, Keep extern Blocks

Remove the `pub mod *_h {}` modules but keep:
```rust
// Keep extern blocks for C functions
extern "C" {
    fn admin_error(...);
    fn pktbuf_dynamic(...);
}

// Keep struct definitions that are used
#[repr(C)]
struct PgStats { ... }
```

This reduces duplication but maintains compatibility.

## Changes Made (Successful)

The following refactoring was applied to stats.rs and all tests pass:

### 1. Removed `extern "C"` from internal functions
```rust
// Before:
unsafe extern "C" fn reset_stats(mut stat: *mut PgStats) { ... }

// After:
unsafe fn reset_stats(stat: *mut PgStats) { ... }
```

### 2. Simplified wrapping arithmetic
```rust
// Before:
(*total).server_bytes = (*total).server_bytes.wrapping_add((*stat).server_bytes);

// After:
(*total).server_bytes += (*stat).server_bytes;
```

### 3. Removed redundant `mut` qualifiers
```rust
// Before:
unsafe extern "C" fn calc_average(mut avg: *mut PgStats, mut cur: *mut PgStats, mut old: *mut PgStats)

// After:
unsafe fn calc_average(avg: *mut PgStats, cur: *mut PgStats, old: *mut PgStats)
```

### 4. Replaced verbose struct initialization with `zeroed()`
```rust
// Before:
let mut avg = PgStats {
    server_assignment_count: 0,
    xact_count: 0,
    query_count: 0,
    server_bytes: 0,
    // ... 8 more fields ...
};

// After:
let mut avg: PgStats = ::core::mem::zeroed();
```

### 5. Replaced byte string casts with C string literals
```rust
// Before:
b"event_add failed: %s\0" as *const u8 as *const ::core::ffi::c_char

// After:
c"event_add failed: %s".as_ptr()
```

### 6. Simplified pointer casting
```rust
// Before:
pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut PgPool;

// After:
pool = item as *mut PgPool;
```

### 7. Removed unnecessary type annotations
```rust
// Before:
if dur <= 0 as usec_t { return; }
if query_count > 0 as uint64_t { ... }

// After:
if dur == 0 { return; }
if query_count > 0 { ... }
```

## Code Section Improvements (Safe to Do)

Even without removing type modules, the function code can be cleaned:

### Before (c2rust style):
```rust
(*total).server_bytes = (*total).server_bytes.wrapping_add((*stat).server_bytes);
```

### After (idiomatic):
```rust
(*total).server_bytes += (*stat).server_bytes;
```

### Before:
```rust
if dur <= 0 as usec_t { return; }
```

### After:
```rust
if dur == 0 { return; }
```

### Before:
```rust
b"no mem\0" as *const u8 as *const ::core::ffi::c_char
```

### After:
```rust
c"no mem".as_ptr()
```

## Recommendations

1. **Don't attempt bulk type module removal** - too risky, breaks compilation
2. **Do simplify function implementations** - safe, doesn't affect types
3. **Consider a shared types.rs approach** - but needs coordinated migration
4. **Document type dependencies** - map which types come from where

## Next Steps

1. Clean up the function code in stats.rs (wrapping_add → +=, etc.)
2. Apply same cleanup to other files
3. Create comprehensive types.rs in common/
4. Plan coordinated migration of all files to shared types
