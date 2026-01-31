#!/usr/bin/env python3
"""
Remove a module from all files and update imports to use crate::types.

Unlike merge_module.py, this does NOT add anything to types.rs - it assumes
the types are already defined there (or as aliases).
"""

import re
import sys
from pathlib import Path

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

def extract_module_content(content: str, module_name: str) -> tuple[str, int, int] | None:
    """Extract the content of a module. Returns (content, start, end) or None."""
    pattern = rf'^pub mod {re.escape(module_name)} \{{'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        return None
    
    start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        return None
    
    inner_content = content[brace_start + 1:brace_end].strip()
    return inner_content, start, brace_end + 1

def get_exported_symbols(content: str) -> list[str]:
    """Get all pub symbols from module content."""
    symbols = []
    
    # Type aliases
    for match in re.finditer(r'pub\s+type\s+(\w+)', content):
        symbols.append(match.group(1))
    
    # Structs
    for match in re.finditer(r'pub\s+struct\s+(\w+)', content):
        symbols.append(match.group(1))
    
    # Unions
    for match in re.finditer(r'pub\s+union\s+(\w+)', content):
        symbols.append(match.group(1))
    
    # Constants
    for match in re.finditer(r'pub\s+const\s+(\w+)', content):
        symbols.append(match.group(1))
    
    # Functions
    for match in re.finditer(r'pub\s+fn\s+(\w+)', content):
        symbols.append(match.group(1))
    
    return list(set(symbols))

def main():
    if len(sys.argv) < 2:
        print("Usage: python remove_module.py <module_name> [--apply]")
        print("Example: python remove_module.py event_struct_h --apply")
        return
    
    module_name = sys.argv[1]
    apply_changes = '--apply' in sys.argv
    
    src_dir = Path(__file__).parent.parent / 'src'
    files = list(src_dir.glob('*.rs')) + list(src_dir.glob('common/*.rs'))
    
    print(f"Removing module: {module_name}")
    print("=" * 60)
    
    # Collect all symbols from all instances
    all_symbols = set()
    file_locations = []
    
    for filepath in sorted(files):
        content = filepath.read_text()
        result = extract_module_content(content, module_name)
        if result:
            inner_content, start, end = result
            symbols = get_exported_symbols(inner_content)
            all_symbols.update(symbols)
            file_locations.append((filepath, start, end))
            print(f"  {filepath.name}: {len(symbols)} symbols")
    
    if not file_locations:
        print(f"No instances of {module_name} found.")
        return
    
    print(f"\nFound {len(file_locations)} instances")
    print(f"Symbols to migrate: {sorted(all_symbols)}")
    
    if apply_changes:
        print("\nApplying changes...")
        
        is_main = lambda p: p.name == 'main.rs'
        
        for filepath, start, end in file_locations:
            content = filepath.read_text()
            types_prefix = 'pgbouncer::types' if is_main(filepath) else 'crate::types'
            
            # Update imports
            for sym in all_symbols:
                content = re.sub(
                    rf'self::{re.escape(module_name)}::{re.escape(sym)}',
                    f'{types_prefix}::{sym}',
                    content
                )
                content = re.sub(
                    rf'super::{re.escape(module_name)}::{re.escape(sym)}',
                    f'{types_prefix}::{sym}',
                    content
                )
            
            # Update use statements
            content = re.sub(
                rf'pub use self::{re.escape(module_name)}::',
                f'pub use {types_prefix}::',
                content
            )
            content = re.sub(
                rf'use self::{re.escape(module_name)}::',
                f'use {types_prefix}::',
                content
            )
            content = re.sub(
                rf'use super::{re.escape(module_name)}::',
                f'use {types_prefix}::',
                content
            )
            
            # Remove the module
            result = extract_module_content(content, module_name)
            if result:
                _, mod_start, mod_end = result
                while mod_end < len(content) and content[mod_end] == '\n':
                    mod_end += 1
                content = content[:mod_start] + content[mod_end:]
            
            filepath.write_text(content)
            print(f"  Updated {filepath.name}")
        
        print(f"\nRemoved {module_name} from {len(file_locations)} files")
        print("Run 'cargo build' to verify")
    else:
        print("\nTo apply, run with --apply flag")

if __name__ == '__main__':
    main()
