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
| Consolidate MBuf type | 📋 Pending |
| Remove duplicate type modules | 📋 In Progress |

**Lines saved from type consolidation: ~1,382**

## Current State Metrics

| Metric | Original | Current |
|--------|----------|---------|
| Total Rust lines (src/) | ~126,000 | ~124,600 |
| Total Rust lines (lib/usual/) | ~26,000 | ~26,000 |
| `static mut` occurrences (src/) | 430 | 430 |
| `unsafe extern "C" fn` (src/) | 1,787 | ~1,787 |
| `#[no_mangle]` (src/) | 450 | 450 |
| `#[c2rust::...]` attributes (src/) | 7,150 | **0** ✅ |
| `#[c2rust::...]` attributes (lib/usual/) | 2,761 | **0** ✅ |

## Phase Overview

| Phase | Modules | Focus |
|-------|---------|-------|
| 1 | main.rs, client.rs, server.rs | Entry point & core connection handling |
| 2 | pooler.rs, sbuf.rs | Connection pooling & I/O |
| 3 | proto.rs, admin.rs, messages.rs | Protocol & admin console |
| 4 | scram.rs, hba.rs | Authentication |
| 5 | objects.rs, loader.rs, janitor.rs, etc. | Supporting modules |
| 6 | src/common/, lib/usual/ | Low-level (defer or replace with crates) |

## Per-Module Refactoring Steps

For each module, follow these steps in order:

### Step 1: Remove c2rust Artifacts
- Delete `#[c2rust::src_loc = "..."]` attributes
- Delete `#[c2rust::header_src = "..."]` attributes
- Remove duplicated type modules (e.g., `mod _types_h { ... }`)
- Remove `extern "C"` from internal function signatures

### Step 2: Consolidate Types
- Import from `crate::src::common::types` instead of local redefinitions
- Replace C type aliases with Rust types:
  - `c_int` → `i32`
  - `c_uint` → `u32`
  - `size_t` → `usize`
  - etc.

### Step 3: Convert Static Mutable State
- Replace `static mut` with thread-local RefCell:
  ```rust
  // Before
  static mut cf_verbose: c_int = 0;
  
  // After
  thread_local! {
      static CF_VERBOSE: RefCell<i32> = RefCell::new(0);
  }
  ```

### Step 4: Convert Function Signatures
- Raw pointers → references where safe
- Return codes → `Result<T, Error>`
- Boolean ints → `bool`

### Step 5: Simplify Patterns
- Remove cast chains (`x as c_int as i32` → `x as i32`)
- Replace `!ptr.is_null()` checks with `Option<&T>`
- Use idiomatic Rust control flow

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
