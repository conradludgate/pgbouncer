# Phase 3: Protocol & Admin

PostgreSQL protocol handling and admin console.

## Current Metrics (2026-01-30)

| Module | Lines | Unsafe Fns | #[no_mangle] | Static Mut | Duplicate Modules |
|--------|-------|------------|--------------|------------|-------------------|
| proto.rs | 2,900 | 28 | 11 | 5 | 26 |
| admin.rs | 5,663 | 135 | 11 | 34 | 41 |
| messages.rs | 2,000 | 17 | 8 | 1 | 17 |

## proto.rs

### Overview
PostgreSQL protocol parsing and generation:
- Packet header parsing
- Message type handling
- Parameter encoding/decoding

### Key Functions
1. `parse_packet()` — parse incoming packet header
2. `send_*()` functions — send protocol messages
3. `handle_*_packet()` — protocol message handlers

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules (depends on bouncer_h)
3. Consider defining proper message types (enums with data)
4. Replace raw buffer manipulation with structured parsing
5. This is a good candidate for safe Rust (pure parsing logic)

## admin.rs

### Overview
Admin console implementation:
- SHOW commands (databases, pools, stats, etc.)
- RELOAD, PAUSE, RESUME, SHUTDOWN
- Command parsing and dispatch

### Key Data Structures
- Command dispatch table
- Admin connection state

### This Module Has Many Functions
135 unsafe functions — mostly command handlers. Many are relatively simple and can be made safe.

### Priority Functions
1. `admin_cmd()` — command dispatcher
2. `admin_show_*()` — various SHOW implementations
3. `admin_reload()`, `admin_pause()`, etc.

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules (41 modules to remove!)
3. Replace function pointer dispatch with match/enum
4. Many SHOW commands are just formatting — easy to make safe
5. Good module for incremental wins

## messages.rs

### Overview
Protocol message definitions and helpers:
- Error/notice message formatting
- Standard PostgreSQL messages
- Message constants

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Define proper message types
3. Use Rust string formatting instead of C-style sprintf
4. Relatively small module — good quick win

## Checklist

### Completed ✅
- [x] proto.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] proto.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs
- [x] admin.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] admin.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs
- [x] messages.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] messages.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs

### Blocked 🚧 (Waiting on Type Consolidation)
- [ ] **Consolidate bouncer_h** first (Phase 1 blocker)
- [ ] admin.rs has 41 duplicate modules — largest in codebase

### Pending 📋 (After Type Consolidation)
- [ ] proto.rs: Remove remaining type modules (~26 modules)
- [ ] proto.rs: Import all types from crate::types::*
- [ ] proto.rs: Define packet type enum
- [ ] proto.rs: Safe parsing for simple messages
- [ ] admin.rs: Remove remaining type modules (~41 modules)
- [ ] admin.rs: Import all types from crate::types::*
- [ ] admin.rs: Replace dispatch table with match
- [ ] admin.rs: Convert SHOW handlers to safe Rust
- [ ] admin.rs: Apply function cleanup patterns
- [ ] messages.rs: Remove remaining type modules (~17 modules)
- [ ] messages.rs: Use Rust string formatting
- [ ] All: Run test_admin.py (specifically tests admin functionality)

### Notes

**admin.rs is a good candidate for incremental cleanup:**
- Many SHOW handlers are pure formatting
- Command dispatch can become a match statement
- Relatively isolated from I/O complexity

**Testing:**
```bash
cd test && pytest test_admin.py -v --timeout=120
```
