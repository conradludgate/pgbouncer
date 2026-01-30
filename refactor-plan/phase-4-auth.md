# Phase 4: Authentication

Authentication mechanisms: SCRAM, HBA, PAM, LDAP.

## Modules

| Module | Unsafe Fns | #[no_mangle] | Static Mut | Lines |
|--------|------------|--------------|------------|-------|
| scram.rs | 35 | 14 | 4 | ~2,700 |
| hba.rs | 62 | 9 | 1 | ~2,200 |
| pam.rs | 14 | 3 | - | ~1,800 |
| ldapauth.rs | 14 | 3 | - | ~1,800 |

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
1. Clean up c2rust artifacts
2. SCRAM is mostly pure computation — good for safe Rust
3. Define proper types for SCRAM state machine
4. Eventually replace with a SCRAM crate or clean implementation

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
1. Clean up c2rust artifacts
2. Define proper types for HBA rules
3. Parsing logic can be made safe
4. Address matching needs care (network/IP handling)

## pam.rs & ldapauth.rs

### Overview
PAM and LDAP authentication — external auth systems.

### Strategy
These are FFI-heavy (calling external C libraries). Strategy:
1. Clean up c2rust artifacts
2. Keep FFI boundary unsafe
3. Make wrapper logic safe
4. Lower priority than SCRAM/HBA

## Checklist

- [ ] scram.rs: Remove c2rust type modules
- [ ] scram.rs: Import from common/types.rs
- [ ] scram.rs: Define SCRAM state machine types
- [ ] scram.rs: Make pure computation safe
- [ ] hba.rs: Remove c2rust type modules
- [ ] hba.rs: Define HBA rule types
- [ ] hba.rs: Safe parsing logic
- [ ] pam.rs: Basic cleanup
- [ ] ldapauth.rs: Basic cleanup
- [ ] All: Run test_auth.py for auth-specific tests
