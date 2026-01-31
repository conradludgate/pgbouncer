#!/usr/bin/env python3
"""
Consolidate sys__types_h modules by:
1. Removing pub mod sys__types_h { ... } blocks
2. Updating pub use self::sys__types_h::... to pub use crate::types::...
3. Updating use super::sys__types_h::... to use crate::types::...
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

def remove_sys_types_module(content: str) -> str:
    """Remove the pub mod sys__types_h { ... } block."""
    pattern = r'^pub mod sys__types_h \{'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        return content
    
    start = match.start()
    brace_start = match.end() - 1  # Position of opening brace
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        print("Warning: Could not find matching brace for sys__types_h")
        return content
    
    # Include trailing newlines
    end = brace_end + 1
    while end < len(content) and content[end] == '\n':
        end += 1
    
    return content[:start] + content[end:]

def process_file(filepath: Path, dry_run: bool = False) -> tuple[bool, str]:
    """Process a single file. Returns (changed, message)."""
    content = filepath.read_text()
    original = content
    
    # Remove the sys__types_h module block
    content = remove_sys_types_module(content)
    
    # Update pub use self::sys__types_h::... to pub use crate::types::...
    content = re.sub(
        r'pub use self::sys__types_h::',
        'pub use crate::types::',
        content
    )
    
    # Update use super::sys__types_h::... to use crate::types::...
    content = re.sub(
        r'use super::sys__types_h::',
        'use crate::types::',
        content
    )
    
    if content == original:
        return False, f"No changes needed: {filepath}"
    
    if not dry_run:
        filepath.write_text(content)
        return True, f"Updated: {filepath}"
    else:
        return True, f"Would update: {filepath}"

def main():
    dry_run = '--dry-run' in sys.argv
    src_dir = Path(__file__).parent.parent / 'src'
    
    # Process all .rs files in src/
    files = list(src_dir.glob('*.rs')) + list(src_dir.glob('common/*.rs'))
    
    changed = 0
    for filepath in sorted(files):
        was_changed, msg = process_file(filepath, dry_run)
        if was_changed:
            print(msg)
            changed += 1
    
    print(f"\n{'Would change' if dry_run else 'Changed'}: {changed} files")

if __name__ == '__main__':
    main()
