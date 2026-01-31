# Type Migration Guidelines

Rules and patterns for refactoring types in the PgBouncer codebase.

## Core Principles

1. **Replace Darwin-specific types with portable definitions**
   - All `__darwin_*` types are c2rust artifacts
   - Replace with POSIX/libc equivalents

2. **Prefer specific struct definitions over opaque types**
   - Use full struct definitions when available
   - Avoid `extern { pub type X; }` unless truly opaque

3. **Use libc definitions when available**
   - Prefer `libc::in6_addr`, `libc::addrinfo`, `libc::FILE`
   - Remove custom definitions that duplicate libc

4. **Update platform-specific field accesses**
   - Example: `.__u6_addr.__u6_addr8` → `.s6_addr`

## Module Consolidation Patterns

### Simple Modules (use scripts)

Modules with only type definitions, constants, or extern functions:

```bash
python3 scripts/merge_module.py MODULE_NAME --apply
```

### Complex Modules (manual approach)

Modules mixing types with extern statics (bouncer_h, iobuf_h):

```rust
// Before: everything in bouncer_h module
pub mod bouncer_h {
    pub struct PgSocket { ... }  // Type - move to types.rs
    extern "C" {
        pub static mut cf_admin_users: *mut c_char;  // Extern - keep in file
    }
}

// After: types from crate::types, externs at file level
use crate::types::PgSocket;
extern "C" {
    pub static mut cf_admin_users: *mut c_char;
}
```

### Binary vs Library Imports

- `src/main.rs` uses `pgbouncer::types::X`
- All other files use `crate::types::X`

## Function Cleanup Patterns

```rust
// Wrapping arithmetic
x.wrapping_add(y)  →  x += y
x.wrapping_sub(y)  →  x -= y

// C strings
b"str\0" as *const u8 as *const c_char  →  c"str".as_ptr()

// Boolean patterns
true_0 != 0           →  true
(expr) as c_int != 0  →  expr
bool_fn() as c_int    →  bool_fn()

// Unnecessary casts
0 as usec_t           →  0
ptr.offset(0)         →  ptr
```

## Type Coordination

When migrating types shared across files:

```
stats::PgPool ≠ admin::bouncer_h::PgPool  // Rust sees as different types!
```

**Solution**: Migrate all files sharing a type together, or add type aliases.

## C2RustUnnamed Types

Anonymous C unions/structs become `C2RustUnnamed_N`. Map to named types:

| C2RustUnnamed | Named Type | Origin |
|---|---|---|
| `C2RustUnnamed_0` | `event_union` | libevent |
| `C2RustUnnamed_1` | `event_signal` | libevent |
| `C2RustUnnamed_4` | `event_io_next` | LIST_ENTRY |
| `C2RustUnnamed_5` | `event_timeout_pos` | TAILQ_ENTRY |

Add aliases in types.rs: `pub type C2RustUnnamed_4 = event_io_next;`

## Static Mut Conversion

**Currently blocked** — variables shared between C and Rust code.

When unblocked:
```rust
// Before
static mut cf_verbose: c_int = 0;

// After
thread_local! {
    static CF_VERBOSE: RefCell<i32> = RefCell::new(0);
}
```

## Refactoring Order (Per Module)

1. ✅ Remove c2rust attributes
2. 🔴 Consolidate type modules
3. 🟡 Clean up function code
4. ⬜ Convert static mut
5. ⬜ Convert function signatures
6. ⬜ Handle FFI boundary
