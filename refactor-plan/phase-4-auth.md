# Phase 4: Authentication

Authentication mechanisms: SCRAM, HBA, PAM, LDAP.

## Current Metrics (2026-01-30)

| Module | Lines | Unsafe Fns | #[no_mangle] | Static Mut | Duplicate Modules |
|--------|-------|------------|--------------|------------|-------------------|
| scram.rs | 2,700 | 33 | 14 | 4 | 27 |
| hba.rs | 2,200 | 59 | 9 | 1 | 22 |
| pam.rs | 1,800 | 12 | 3 | 0 | 15 |
| ldapauth.rs | 774 | 12 | 3 | 0 | 15 |

## scram.rs

### Overview
SCRAM-SHA-256 authentication implementation:
- Client-side SCRAM (authenticating to backends)
- Server-side SCRAM (authenticating clients)
- Nonce generation, proof calculation

### Dependencies
- `common/scram_common.rs` — shared SCRAM functions
- `common/cryptohash.rs`, `common/hmac.rs` — crypto primitives

### Key Functions
1. `scram_client_first()` — initiate client auth
2. `scram_server_first()` — handle server challenge
3. `scram_verify()` — verify authentication
4. `scram_build_verifier()` — create password verifier

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules
3. SCRAM is mostly pure computation — good for safe Rust
4. Define proper types for SCRAM state machine
5. Eventually replace with a SCRAM crate or clean implementation

## hba.rs

### Overview
Host-Based Authentication (pg_hba.conf) parsing:
- Rule parsing
- Address matching
- Authentication method selection

### Key Functions
1. `parse_hba_file()` — parse HBA config
2. `check_hba()` — check connection against rules
3. Address/hostname matching functions

### Strategy
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules
3. Define proper types for HBA rules
4. Parsing logic can be made safe
5. Address matching needs care (network/IP handling)

## pam.rs & ldapauth.rs

### Overview
PAM and LDAP authentication — external auth systems.

### Strategy
These are FFI-heavy (calling external C libraries). Strategy:
1. ~~Clean up c2rust artifacts~~ ✅ Done
2. Consolidate type modules
3. Keep FFI boundary unsafe
4. Make wrapper logic safe
5. Lower priority than SCRAM/HBA

## Checklist

### Completed ✅
- [x] scram.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] scram.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs
- [x] hba.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] hba.rs: Consolidate List, StatList to common/types.rs
- [x] pam.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] pam.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs
- [x] ldapauth.rs: Remove c2rust::src_loc and c2rust::header_src attributes
- [x] ldapauth.rs: Consolidate PgStats, List, StatList, AATree to common/types.rs

### Blocked 🚧 (Waiting on Type Consolidation)
- [ ] **Consolidate bouncer_h** first (Phase 1 blocker)

### Pending 📋 (After Type Consolidation)
- [ ] scram.rs: Remove remaining type modules (~27 modules)
- [ ] scram.rs: Import all types from crate::types::*
- [ ] scram.rs: Define SCRAM state machine types
- [ ] scram.rs: Make pure computation safe
- [ ] hba.rs: Remove remaining type modules (~22 modules)
- [ ] hba.rs: Define HBA rule types
- [ ] hba.rs: Safe parsing logic
- [ ] pam.rs: Remove remaining type modules (~15 modules)
- [ ] ldapauth.rs: Remove remaining type modules (~15 modules)
- [ ] All: Run test_auth.py for auth-specific tests

### Notes

**scram.rs is a good candidate for safe Rust:**
- Mostly pure computation (hashing, base64)
- Clear state machine
- Could eventually use `scram` crate

**Testing:**
```bash
cd test && pytest test_auth.py -v --timeout=120
```
