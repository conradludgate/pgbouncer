#!/usr/bin/env python3
"""
Rewrite _h modules to use types from crate::types.

For each target module:
1. Extract extern "C" blocks (statics and functions)
2. Replace the module body with:
   - pub use crate::types::*; (re-export all types)
   - Any extern "C" blocks from the original (but strip type declarations)
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
    """Extract all extern "C" blocks from module content, stripping type declarations."""
    blocks = []
    i = 0
    while i < len(module_content):
        match = re.search(r'extern\s+"C"\s*\{', module_content[i:])
        if not match:
            break
        
        block_start = i + match.start()
        brace_pos = i + match.end() - 1
        brace_end = find_matching_brace(module_content, brace_pos)
        
        if brace_end == -1:
            print(f"Warning: Could not find closing brace for extern block")
            i = brace_pos + 1
            continue
        
        block_content = module_content[brace_pos + 1:brace_end]
        
        # Remove `pub type TypeName;` declarations (opaque types that are in types.rs)
        cleaned_content = re.sub(r'\s*pub\s+type\s+\w+\s*;', '', block_content)
        
        # Only keep block if it has content
        if cleaned_content.strip():
            blocks.append(f"extern \"C\" {{\n{cleaned_content}\n}}")
        
        i = brace_end + 1
    
    return blocks


def rewrite_module(file_path: Path, module_name: str) -> bool:
    """Rewrite the specified module in the given file."""
    content = file_path.read_text()
    
    pattern = rf'^(\s*)pub mod {re.escape(module_name)} \{{'
    match = re.search(pattern, content, re.MULTILINE)
    
    if not match:
        return False
    
    indent = match.group(1)
    module_start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        print(f"Error: Could not find closing brace for {module_name} in {file_path}")
        return False
    
    module_content = content[brace_start + 1:brace_end]
    extern_blocks = extract_extern_blocks(module_content)
    
    if file_path.name == "main.rs":
        types_path = "pgbouncer::types"
    else:
        types_path = "crate::types"
    
    inner_indent = indent + "    "
    new_lines = [f"{inner_indent}pub use {types_path}::*;"]
    
    for block in extern_blocks:
        lines = block.split('\n')
        indented_lines = []
        for line in lines:
            stripped = line.strip()
            if stripped:
                indented_lines.append(f"{inner_indent}{stripped}")
            else:
                indented_lines.append("")
        new_lines.append('\n'.join(indented_lines))
    
    new_module = f"{indent}pub mod {module_name} {{\n"
    new_module += '\n'.join(new_lines)
    new_module += f"\n{indent}}}"
    
    new_content = content[:module_start] + new_module + content[brace_end + 1:]
    
    file_path.write_text(new_content)
    return True


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 rewrite_module.py <module_name>")
        print("Example: python3 rewrite_module.py sbuf_h")
        sys.exit(1)
    
    module_name = sys.argv[1]
    src_dir = Path(__file__).parent.parent / "src"
    
    print(f"Rewriting {module_name} modules to use crate::types...")
    print("=" * 60)
    
    updated = 0
    
    for file_path in sorted(src_dir.glob("*.rs")):
        if file_path.name == "lib.rs":
            continue
        if rewrite_module(file_path, module_name):
            print(f"  Updated {file_path.name}")
            updated += 1
    
    for file_path in sorted((src_dir / "common").glob("*.rs")):
        if file_path.name == "types.rs":
            continue
        if rewrite_module(file_path, module_name):
            print(f"  Updated common/{file_path.name}")
            updated += 1
    
    print("=" * 60)
    print(f"Updated {updated} files")


if __name__ == "__main__":
    main()
