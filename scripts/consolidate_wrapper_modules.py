#!/usr/bin/env python3
"""
Consolidate simple type wrapper modules (_gid_t_h, _mode_t_h, etc.) by:
1. Removing pub mod _*_h { } blocks that just define type aliases
2. Updating imports to use crate::types:: directly
"""

import re
import sys
from pathlib import Path

# Wrapper modules that just define type aliases from darwin types
# Format: module_name -> type_name
SIMPLE_WRAPPERS = {
    '_gid_t_h': 'gid_t',
    '_mode_t_h': 'mode_t',
    '_off_t_h': 'off_t',
    '_dev_t_h': 'dev_t',
    '_blkcnt_t_h': 'blkcnt_t',
    '_blksize_t_h': 'blksize_t',
    '_useconds_t_h': 'useconds_t',
    '_nlink_t_h': 'nlink_t',
    '_time_t_h': 'time_t',
    '_socklen_t_h': 'socklen_t',
    '_va_list_h': 'va_list',
    '_u_int32_t_h': 'u_int32_t',
    # Primitive integer types (already in types.rs)
    '_int32_t_h': 'int32_t',
    '_int64_t_h': 'int64_t',
    '_sigset_t_h': 'sigset_t',
    # Struct types (have libc equivalents in types.rs)
    '_timespec_h': 'timespec',
    '_iovec_t_h': 'iovec',
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

def remove_module(content: str, module_name: str) -> str:
    """Remove a pub mod X_h { ... } block."""
    pattern = rf'^pub mod {re.escape(module_name)} \{{'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        return content
    
    start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        print(f"Warning: Could not find matching brace for {module_name}")
        return content
    
    # Include trailing newlines
    end = brace_end + 1
    while end < len(content) and content[end] == '\n':
        end += 1
    
    return content[:start] + content[end:]

def process_file(filepath: Path, dry_run: bool = False) -> tuple[bool, list[str]]:
    """Process a single file. Returns (changed, messages)."""
    content = filepath.read_text()
    original = content
    messages = []
    
    is_main = filepath.name == 'main.rs'
    types_prefix = 'pgbouncer::types' if is_main else 'crate::types'
    
    for module_name, type_name in SIMPLE_WRAPPERS.items():
        # Check if this module exists in the file
        if f'pub mod {module_name}' not in content:
            continue
        
        # Remove the module
        new_content = remove_module(content, module_name)
        if new_content != content:
            messages.append(f"  Removed {module_name}")
            content = new_content
        
        # Update imports: self::_X_h::type -> crate::types::type
        pattern = rf'self::{re.escape(module_name)}::{re.escape(type_name)}'
        replacement = f'{types_prefix}::{type_name}'
        new_content = re.sub(pattern, replacement, content)
        if new_content != content:
            content = new_content
        
        # Update imports: super::_X_h::type -> crate::types::type
        pattern = rf'super::{re.escape(module_name)}::{re.escape(type_name)}'
        new_content = re.sub(pattern, replacement, content)
        if new_content != content:
            content = new_content
    
    if content == original:
        return False, []
    
    if not dry_run:
        filepath.write_text(content)
    
    return True, messages

def main():
    dry_run = '--dry-run' in sys.argv
    src_dir = Path(__file__).parent.parent / 'src'
    
    files = list(src_dir.glob('*.rs')) + list(src_dir.glob('common/*.rs'))
    
    changed = 0
    for filepath in sorted(files):
        was_changed, messages = process_file(filepath, dry_run)
        if was_changed:
            action = "Would update" if dry_run else "Updated"
            print(f"{action}: {filepath}")
            for msg in messages:
                print(msg)
            changed += 1
    
    print(f"\n{'Would change' if dry_run else 'Changed'}: {changed} files")

if __name__ == '__main__':
    main()
