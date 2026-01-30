# Phase 3: Protocol & Admin

PostgreSQL protocol handling and admin console.

## Modules

| Module | Unsafe Fns | #[no_mangle] | Static Mut | Lines |
|--------|------------|--------------|------------|-------|
| proto.rs | 42 | 11 | 5 | ~2,900 |
| admin.rs | 148 | 11 | 34 | ~6,000 |
| messages.rs | 26 | 8 | 1 | ~2,000 |

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
1. Clean up c2rust artifacts
2. Consider defining proper message types (enums with data)
3. Replace raw buffer manipulation with structured parsing
4. This is a good candidate for safe Rust (pure parsing logic)

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
148 unsafe functions — mostly command handlers. Many are relatively simple and can be made safe.

### Priority Functions
1. `admin_cmd()` — command dispatcher
2. `admin_show_*()` — various SHOW implementations
3. `admin_reload()`, `admin_pause()`, etc.

### Strategy
1. Clean up c2rust artifacts
2. Replace function pointer dispatch with match/enum
3. Many SHOW commands are just formatting — easy to make safe
4. Good module for incremental wins

## messages.rs

### Overview
Protocol message definitions and helpers:
- Error/notice message formatting
- Standard PostgreSQL messages
- Message constants

### Strategy
1. Clean up c2rust artifacts
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

### Pending 📋
- [ ] proto.rs: Remove remaining c2rust type modules
- [ ] proto.rs: Import all types from common/types.rs
- [ ] proto.rs: Define packet type enum
- [ ] proto.rs: Safe parsing for simple messages
- [ ] admin.rs: Remove remaining c2rust type modules
- [ ] admin.rs: Import all types from common/types.rs
- [ ] admin.rs: Replace dispatch table with match
- [ ] admin.rs: Convert SHOW handlers to safe Rust
- [ ] messages.rs: Remove remaining c2rust type modules
- [ ] messages.rs: Use Rust string formatting
- [ ] All: Run test_admin.py (specifically tests admin functionality)
