#!/usr/bin/env python3
"""
Slim down bouncer_h modules by:
1. Removing type definitions (structs, type aliases, unions, constants that are now in types.rs)
2. Keeping extern static declarations and extern function declarations
3. Adding appropriate imports from types.rs
"""

import re
import sys
from pathlib import Path

# Types that are now in types.rs and should be imported, not defined locally
TYPES_IN_TYPES_RS = {
    # Type aliases
    'SocketState', 'PauseMode', 'ShutDownMode', 'PacketCallbackFlag', 
    'LoadBalanceHosts', 'ReplicationType', 'SSLMode', 'auth_type',
    'ResponseAction',
    
    # Structs
    'PgSocket', 'PgPool', 'PgDatabase', 'PgCredentials', 'PgGlobalUser',
    'PgAddr', 'sockaddr_ucreds', 'ScramState', 'CallbackState',
    'C2RustUnnamed_9', 'OutstandingRequest',
    
    # Constants (SocketState)
    'CL_FREE', 'CL_JUSTFREE', 'CL_LOGIN', 'CL_WAITING', 'CL_WAITING_LOGIN',
    'CL_ACTIVE', 'CL_WAITING_CANCEL', 'CL_ACTIVE_CANCEL',
    'SV_FREE', 'SV_JUSTFREE', 'SV_LOGIN', 'SV_BEING_CANCELED', 'SV_IDLE',
    'SV_ACTIVE', 'SV_ACTIVE_CANCEL', 'SV_USED', 'SV_TESTED',
    
    # Constants (PauseMode)
    'P_NONE', 'P_PAUSE', 'P_SUSPEND',
    
    # Constants (ShutDownMode)
    'SHUTDOWN_NONE', 'SHUTDOWN_WAIT_FOR_SERVERS', 'SHUTDOWN_WAIT_FOR_CLIENTS', 'SHUTDOWN_IMMEDIATE',
    
    # Constants (PacketCallbackFlag)
    'CB_NONE', 'CB_WANT_COMPLETE_PACKET', 'CB_HANDLE_COMPLETE_PACKET',
    
    # Constants (LoadBalanceHosts)
    'LOAD_BALANCE_HOSTS_DISABLE', 'LOAD_BALANCE_HOSTS_ROUND_ROBIN',
    
    # Constants (ReplicationType)
    'REPLICATION_NONE', 'REPLICATION_LOGICAL', 'REPLICATION_PHYSICAL',
    
    # Constants (SSLMode)
    'SSLMODE_DISABLED', 'SSLMODE_ALLOW', 'SSLMODE_PREFER', 'SSLMODE_REQUIRE',
    'SSLMODE_VERIFY_CA', 'SSLMODE_VERIFY_FULL',
    
    # Constants (auth_type)
    'AUTH_TYPE_ANY', 'AUTH_TYPE_TRUST', 'AUTH_TYPE_PLAIN', 'AUTH_TYPE_MD5',
    'AUTH_TYPE_CERT', 'AUTH_TYPE_HBA', 'AUTH_TYPE_LDAP', 'AUTH_TYPE_PAM',
    'AUTH_TYPE_SCRAM_SHA_256', 'AUTH_TYPE_PEER', 'AUTH_TYPE_REJECT',
    
    # Constants (ResponseAction)
    'RA_FORWARD', 'RA_SKIP', 'RA_FAKE',
    
    # Constants (misc)
    'BACKENDKEY_LEN', 'MAX_USERNAME', 'MAX_PASSWORD', 'CANCELLATION_TTL_MASK',
    'PKT_STARTUP_V2', 'PKT_STARTUP_V3', 'PKT_STARTUP_V3_UNSUPPORTED', 'PKT_STARTUP_V4',
    'PKT_CANCEL', 'PKT_SSLREQ', 'PKT_GSSENCREQ', 'RAW_IOBUF_SIZE', 'SD_LISTEN_FDS_START',
    
    # Constants (pool modes - already in types.rs) 
    'POOL_SESSION', 'POOL_TX', 'POOL_STMT', 'POOL_INHERIT',
    
    # Inline functions now in types.rs
    'first_socket', 'last_socket', 'pga_is_unix', 'pga_family', 'cstr_skip_ws',
    
    # Extern functions now in types.rs
    'pga_port', 'pga_set', 'pga_copy', 'pga_cmp_addr', 'pga_ntop', 'pga_str', 'pga_details',
}

def find_matching_brace(content: str, start: int) -> int:
    """Find the index of the closing brace matching the opening brace at start."""
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

def extract_module(content: str, module_name: str):
    """Extract module boundaries. Returns (start, end, inner_content) or None."""
    pattern = rf'^pub mod {re.escape(module_name)} \{{'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        return None
    
    start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        return None
    
    inner_content = content[brace_start + 1:brace_end]
    return start, brace_end + 1, inner_content

def is_extern_declaration(item: str) -> bool:
    """Check if an item is an extern static or function that should be kept."""
    # Keep extern function declarations (but not inline function definitions)
    # Keep extern static declarations
    return ('extern "C"' in item or 
            'pub static mut cf_' in item or
            'pub static mut adns' in item or
            'pub static mut g_' in item or
            'pub static mut parsed_hba' in item or
            'pub static pool_mode_map' in item or
            'pub static load_balance_hosts_map' in item or
            'pub static mut replication_type_parameters' in item or
            'pub static mut pgb_event_base' in item)

def should_keep_item(item: str) -> bool:
    """Determine if an item should be kept in the module."""
    item_stripped = item.strip()
    
    # Always keep extern "C" blocks with static declarations
    if 'extern "C"' in item and ('static' in item or 'pub fn' in item):
        # But not if it only contains pga_* functions we've moved
        if re.search(r'pub fn pga_(port|set|copy|cmp_addr|ntop|str|details)', item):
            return False
        return True
    
    # Keep use statements that import from super or crate (dependencies)
    if item_stripped.startswith('use '):
        return True
    
    # Don't keep type definitions, structs, unions, constants that are in types.rs
    for type_name in TYPES_IN_TYPES_RS:
        if re.search(rf'\bpub\s+(type|const|struct|union)\s+{re.escape(type_name)}\b', item):
            return False
        # Also catch constants
        if re.search(rf'\bpub\s+const\s+{re.escape(type_name)}\s*:', item):
            return False
    
    # Don't keep inline function definitions that are now in types.rs
    for fn_name in ['first_socket', 'last_socket', 'pga_is_unix', 'pga_family', 'cstr_skip_ws']:
        if re.search(rf'\bpub\s+(unsafe\s+)?(extern\s+"C"\s+)?fn\s+{fn_name}\s*\(', item):
            return False
    
    return True

def process_file(filepath: Path, apply: bool) -> bool:
    """Process a single file. Returns True if changes were made."""
    content = filepath.read_text()
    
    result = extract_module(content, 'bouncer_h')
    if not result:
        return False
    
    start, end, inner = result
    
    # Parse items in the module
    # Split on item boundaries (rough parsing)
    # For now, let's take a simpler approach: remove the entire module 
    # and just keep the extern declarations in a simplified form
    
    # Find all extern "C" blocks with static declarations
    extern_blocks = []
    for match in re.finditer(r'extern\s+"C"\s*\{[^}]+\}', inner, re.DOTALL):
        block = match.group()
        # Only keep blocks that have static declarations (not just pga_* functions)
        if 'static' in block:
            extern_blocks.append(block)
    
    # If there are no extern statics to keep, we can remove the module entirely
    if not extern_blocks:
        # Remove the module and add nothing
        new_content = content[:start] + content[end:]
    else:
        # Keep a slimmed down module with just the externs
        new_module = "pub mod bouncer_h {\n"
        for block in extern_blocks:
            new_module += "    " + block.replace('\n', '\n    ') + "\n"
        new_module += "}"
        new_content = content[:start] + new_module + content[end:]
    
    if apply:
        filepath.write_text(new_content)
        print(f"  Updated {filepath.name}")
        return True
    else:
        print(f"  Would update {filepath.name}")
        return False

def main():
    apply = '--apply' in sys.argv
    
    src_dir = Path('src')
    if not src_dir.exists():
        print("Run from project root directory")
        sys.exit(1)
    
    print("Processing bouncer_h modules...")
    print("=" * 60)
    
    changed = 0
    for rs_file in sorted(src_dir.glob('*.rs')):
        if process_file(rs_file, apply):
            changed += 1
    
    print("=" * 60)
    if apply:
        print(f"Updated {changed} files")
    else:
        print(f"Would update {changed} files")
        print("Run with --apply to make changes")

if __name__ == '__main__':
    main()
