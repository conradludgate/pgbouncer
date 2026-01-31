#!/usr/bin/env python3
"""
Comprehensive module consolidation script.
Removes duplicate pub mod X_h { } blocks and updates imports.
"""

import re
import sys
from pathlib import Path

# Modules and their exported symbols
# Format: module_name -> [list of symbols to import from crate::types]
MODULES = {
    # TLS (simple extern type)
    'tls_h': ['tls', 'tls_config'],
    
    # Logging
    'logging_h': ['LogLevel', 'LG_NOISE', 'LG_DEBUG', 'LG_INFO', 'LG_STATS', 
                  'LG_WARNING', 'LG_ERROR', 'LG_FATAL', 'log_generic', 
                  'log_warning', 'log_error', 'log_debug', 'log_noise', 'fatal'],
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

def remove_module(content: str, module_name: str) -> tuple[str, bool]:
    """Remove a pub mod X_h { ... } block. Returns (new_content, was_removed)."""
    pattern = rf'^pub mod {re.escape(module_name)} \{{'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        return content, False
    
    start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        print(f"Warning: Could not find matching brace for {module_name}")
        return content, False
    
    # Include trailing newlines
    end = brace_end + 1
    while end < len(content) and content[end] == '\n':
        end += 1
    
    return content[:start] + content[end:], True

def update_imports(content: str, module_name: str, symbols: list, types_prefix: str) -> str:
    """Update imports from self::module_name::X to types_prefix::X"""
    for symbol in symbols:
        # self::module_name::symbol -> types_prefix::symbol
        pattern = rf'self::{re.escape(module_name)}::{re.escape(symbol)}'
        content = re.sub(pattern, f'{types_prefix}::{symbol}', content)
        
        # super::module_name::symbol -> types_prefix::symbol
        pattern = rf'super::{re.escape(module_name)}::{re.escape(symbol)}'
        content = re.sub(pattern, f'{types_prefix}::{symbol}', content)
    
    return content

def process_file(filepath: Path, modules_to_process: dict, dry_run: bool = False) -> tuple[bool, list[str]]:
    """Process a single file. Returns (changed, messages)."""
    content = filepath.read_text()
    original = content
    messages = []
    
    is_main = filepath.name == 'main.rs'
    types_prefix = 'pgbouncer::types' if is_main else 'crate::types'
    
    for module_name, symbols in modules_to_process.items():
        # Check if this module exists in the file
        if f'pub mod {module_name}' not in content:
            continue
        
        # First update imports (before removing module)
        content = update_imports(content, module_name, symbols, types_prefix)
        
        # Remove the module
        content, was_removed = remove_module(content, module_name)
        if was_removed:
            messages.append(f"  Removed {module_name}")
    
    if content == original:
        return False, []
    
    if not dry_run:
        filepath.write_text(content)
    
    return True, messages

def main():
    dry_run = '--dry-run' in sys.argv
    args = [a for a in sys.argv[1:] if a != '--dry-run']
    
    # Determine which modules to process
    if not args or args[0] == 'all':
        modules_to_process = MODULES
    else:
        modules_to_process = {m: MODULES[m] for m in args if m in MODULES}
        if not modules_to_process:
            print(f"Unknown modules: {args}")
            print(f"Available: {', '.join(MODULES.keys())}")
            return
    
    print(f"Processing modules: {', '.join(modules_to_process.keys())}")
    
    src_dir = Path(__file__).parent.parent / 'src'
    files = list(src_dir.glob('*.rs')) + list(src_dir.glob('common/*.rs'))
    
    changed = 0
    for filepath in sorted(files):
        was_changed, messages = process_file(filepath, modules_to_process, dry_run)
        if was_changed:
            action = "Would update" if dry_run else "Updated"
            print(f"{action}: {filepath}")
            for msg in messages:
                print(msg)
            changed += 1
    
    print(f"\n{'Would change' if dry_run else 'Changed'}: {changed} files")

if __name__ == '__main__':
    main()
