#!/usr/bin/env python3
"""
Slim down a module by removing type definitions that are now in types.rs.
Keeps extern static declarations in the module.
Updates cross-module references to use crate::types::.
"""

import re
import sys
from pathlib import Path

# Types that are now in types.rs
TYPES_IN_TYPES_RS = {
    # Structs
    'PgSocket', 'PgPool', 'PgDatabase', 'PgCredentials', 'PgGlobalUser',
    'ScramState', 'CallbackState', 'OutstandingRequest', 'PktBuf',
    'sockaddr_ucreds',
    
    # Unions
    'PgAddr', 'C2RustUnnamed_9',
    
    # Type aliases (already in types.rs)
    'SocketState', 'PauseMode', 'ShutDownMode', 'PacketCallbackFlag',
    'LoadBalanceHosts', 'ReplicationType', 'SSLMode', 'auth_type',
    'ResponseAction',
    
    # SBuf types already in types.rs
    'SBuf', 'SBufIO', 'SBufEvent', 'sbuf_cb_t', 'TLSState', 'WaitType',
    
    # IOBuf types already in types.rs
    'iobuf', 'IOBuf',
}

# Constants that are now in types.rs
CONSTANTS_IN_TYPES_RS = {
    # SocketState
    'CL_FREE', 'CL_JUSTFREE', 'CL_LOGIN', 'CL_WAITING', 'CL_WAITING_LOGIN',
    'CL_ACTIVE', 'CL_WAITING_CANCEL', 'CL_ACTIVE_CANCEL',
    'SV_FREE', 'SV_JUSTFREE', 'SV_LOGIN', 'SV_BEING_CANCELED', 'SV_IDLE',
    'SV_ACTIVE', 'SV_ACTIVE_CANCEL', 'SV_USED', 'SV_TESTED',
    
    # PauseMode
    'P_NONE', 'P_PAUSE', 'P_SUSPEND',
    
    # ShutDownMode
    'SHUTDOWN_NONE', 'SHUTDOWN_WAIT_FOR_SERVERS', 'SHUTDOWN_WAIT_FOR_CLIENTS', 'SHUTDOWN_IMMEDIATE',
    
    # PacketCallbackFlag
    'CB_NONE', 'CB_WANT_COMPLETE_PACKET', 'CB_HANDLE_COMPLETE_PACKET',
    
    # LoadBalanceHosts
    'LOAD_BALANCE_HOSTS_DISABLE', 'LOAD_BALANCE_HOSTS_ROUND_ROBIN',
    
    # ReplicationType
    'REPLICATION_NONE', 'REPLICATION_LOGICAL', 'REPLICATION_PHYSICAL',
    
    # SSLMode
    'SSLMODE_DISABLED', 'SSLMODE_ALLOW', 'SSLMODE_PREFER', 'SSLMODE_REQUIRE',
    'SSLMODE_VERIFY_CA', 'SSLMODE_VERIFY_FULL',
    
    # auth_type
    'AUTH_TYPE_ANY', 'AUTH_TYPE_TRUST', 'AUTH_TYPE_PLAIN', 'AUTH_TYPE_MD5',
    'AUTH_TYPE_CERT', 'AUTH_TYPE_HBA', 'AUTH_TYPE_LDAP', 'AUTH_TYPE_PAM',
    'AUTH_TYPE_SCRAM_SHA_256', 'AUTH_TYPE_PEER', 'AUTH_TYPE_REJECT',
    
    # ResponseAction
    'RA_FORWARD', 'RA_SKIP', 'RA_FAKE',
    
    # SBufEvent
    'SBUF_EV_READ', 'SBUF_EV_RECV_FAILED', 'SBUF_EV_SEND_FAILED',
    'SBUF_EV_CONNECT_FAILED', 'SBUF_EV_CONNECT_OK', 'SBUF_EV_FLUSH',
    'SBUF_EV_PKT_CALLBACK', 'SBUF_EV_TLS_READY',
    
    # TLSState
    'SBUF_TLS_NONE', 'SBUF_TLS_DO_HANDSHAKE', 'SBUF_TLS_OK',
    
    # WaitType
    'W_NONE', 'W_CONNECT', 'W_RECV', 'W_SEND',
    
    # Pool modes
    'POOL_SESSION', 'POOL_TX', 'POOL_STMT', 'POOL_INHERIT',
    
    # Misc constants
    'BACKENDKEY_LEN', 'MAX_USERNAME', 'MAX_PASSWORD', 'CANCELLATION_TTL_MASK',
    'PKT_STARTUP_V2', 'PKT_STARTUP_V3', 'PKT_STARTUP_V3_UNSUPPORTED', 'PKT_STARTUP_V4',
    'PKT_CANCEL', 'PKT_SSLREQ', 'PKT_GSSENCREQ', 'RAW_IOBUF_SIZE', 'SD_LISTEN_FDS_START',
    'SBUF_SMALL_PKT',
}

# Inline functions that are now in types.rs (should be removed from modules)
FUNCTIONS_IN_TYPES_RS = {
    'first_socket', 'last_socket', 'pga_is_unix', 'pga_family', 'cstr_skip_ws',
    'sbuf_is_empty', 'sbuf_is_closed',
}


def find_matching_brace(content: str, start: int) -> int:
    """Find the closing brace matching the opening brace at start."""
    depth = 0
    i = start
    while i < len(content):
        if content[i] == '{':
            depth += 1
        elif content[i] == '}':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1


def process_file(filepath: Path, module_name: str, apply: bool) -> bool:
    """Process a single file. Returns True if changes needed."""
    content = filepath.read_text()
    original = content
    
    # Find the module
    pattern = rf'^pub mod {re.escape(module_name)} \{{'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        return False
    
    # Update super::bouncer_h::X to crate::types::X for types
    for type_name in TYPES_IN_TYPES_RS:
        content = re.sub(
            rf'\bsuper::{re.escape(module_name)}::{re.escape(type_name)}\b',
            f'crate::types::{type_name}',
            content
        )
        # Also handle direct module references without super
        content = re.sub(
            rf'\b{re.escape(module_name)}::{re.escape(type_name)}\b',
            f'crate::types::{type_name}',
            content
        )
    
    # Update grouped imports like: use super::bouncer_h::{PgSocket, PgPool};
    # This is harder - we need to parse the import list
    
    if content != original:
        if apply:
            filepath.write_text(content)
            print(f"  Updated {filepath.name}")
        else:
            print(f"  Would update {filepath.name}")
        return True
    
    return False


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 slim_module.py MODULE_NAME [--apply]")
        sys.exit(1)
    
    module_name = sys.argv[1]
    apply = '--apply' in sys.argv
    
    src_dir = Path('src')
    if not src_dir.exists():
        print("Run from project root directory")
        sys.exit(1)
    
    print(f"Processing {module_name} modules...")
    print("=" * 60)
    
    changed = 0
    for rs_file in sorted(src_dir.glob('*.rs')):
        if process_file(rs_file, module_name, apply):
            changed += 1
    
    # Also check src/common/
    for rs_file in sorted((src_dir / 'common').glob('*.rs')):
        if process_file(rs_file, module_name, apply):
            changed += 1
    
    print("=" * 60)
    if apply:
        print(f"Updated {changed} files")
    else:
        print(f"Would update {changed} files")
        print("Run with --apply to make changes")


if __name__ == '__main__':
    main()
