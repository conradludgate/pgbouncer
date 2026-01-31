#!/usr/bin/env python3
"""
Rewrite bouncer_h modules to use types from crate::types.

For each bouncer_h module:
1. Extract extern "C" blocks (statics and functions)
2. Replace the module body with:
   - pub use crate::types::*; (re-export all types)
   - use c2rust_bitfields::BitfieldStruct;
   - Any extern "C" blocks from the original
"""

import re
import sys
from pathlib import Path


def find_matching_brace(content: str, start: int) -> int:
    """Find the index of the closing brace that matches the opening brace at start."""
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


def extract_extern_blocks(module_content: str) -> list[str]:
    """Extract all extern "C" blocks from module content."""
    blocks = []
    # Pattern to find extern "C" { ... }
    i = 0
    while i < len(module_content):
        # Look for extern "C"
        match = re.search(r'extern\s+"C"\s*\{', module_content[i:])
        if not match:
            break
        
        block_start = i + match.start()
        brace_pos = i + match.end() - 1  # Position of opening brace
        brace_end = find_matching_brace(module_content, brace_pos)
        
        if brace_end == -1:
            print(f"Warning: Could not find closing brace for extern block at pos {block_start}")
            i = brace_pos + 1
            continue
            
        block = module_content[block_start:brace_end + 1]
        blocks.append(block)
        i = brace_end + 1
    
    return blocks


def rewrite_bouncer_h_module(file_path: Path) -> bool:
    """Rewrite the bouncer_h module in the given file."""
    content = file_path.read_text()
    
    # Find the bouncer_h module
    pattern = r'^(\s*)pub mod bouncer_h \{'
    match = re.search(pattern, content, re.MULTILINE)
    
    if not match:
        return False
    
    indent = match.group(1)
    module_start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        print(f"Error: Could not find closing brace for bouncer_h in {file_path}")
        return False
    
    # Extract the module content (between braces)
    module_content = content[brace_start + 1:brace_end]
    
    # Extract extern "C" blocks
    extern_blocks = extract_extern_blocks(module_content)
    
    # Determine if this is main.rs (needs pgbouncer::types instead of crate::types)
    if file_path.name == "main.rs":
        types_path = "pgbouncer::types"
    else:
        types_path = "crate::types"
    
    # Build new module content
    inner_indent = indent + "    "
    new_lines = [
        f"{inner_indent}pub use {types_path}::*;",
        f"{inner_indent}pub use c2rust_bitfields::BitfieldStruct;",
    ]
    
    # Add extern blocks with proper indentation
    for block in extern_blocks:
        # Re-indent the block
        lines = block.split('\n')
        indented_lines = []
        for line in lines:
            # Strip existing leading whitespace and add our indentation
            stripped = line.strip()
            if stripped:
                indented_lines.append(f"{inner_indent}{stripped}")
            else:
                indented_lines.append("")
        new_lines.append('\n'.join(indented_lines))
    
    # Create the new module
    new_module = f"{indent}pub mod bouncer_h {{\n"
    new_module += '\n'.join(new_lines)
    new_module += f"\n{indent}}}"
    
    # Replace in content
    new_content = content[:module_start] + new_module + content[brace_end + 1:]
    
    file_path.write_text(new_content)
    return True


def main():
    src_dir = Path(__file__).parent.parent / "src"
    
    print("Rewriting bouncer_h modules to use crate::types...")
    print("=" * 60)
    
    updated = 0
    
    # Process main src files
    for file_path in sorted(src_dir.glob("*.rs")):
        if file_path.name == "lib.rs":
            continue
        if rewrite_bouncer_h_module(file_path):
            print(f"  Updated {file_path.name}")
            updated += 1
    
    # Process common directory
    for file_path in sorted((src_dir / "common").glob("*.rs")):
        if file_path.name == "types.rs":
            continue
        if rewrite_bouncer_h_module(file_path):
            print(f"  Updated common/{file_path.name}")
            updated += 1
    
    print("=" * 60)
    print(f"Updated {updated} files")


if __name__ == "__main__":
    main()
