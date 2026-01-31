#!/usr/bin/env python3
"""
Rewrite iobuf_h modules to use types from crate::types while preserving inline functions.
"""

import re
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


def remove_struct_definitions(content: str) -> str:
    """Remove #[derive...] #[repr...] pub struct ... { ... } blocks."""
    result = content
    
    while True:
        derive_match = re.search(r'\n\s*#\[derive[^\]]*\]\s*\n\s*#\[repr[^\]]*\]\s*\n\s*pub struct \w+\s*\{', result)
        if not derive_match:
            break
        
        start = derive_match.start()
        brace_start = derive_match.end() - 1
        brace_end = find_matching_brace(result, brace_start)
        
        if brace_end == -1:
            break
        
        result = result[:start] + result[brace_end + 1:]
    
    return result


def remove_type_aliases(content: str) -> str:
    """Remove pub type Name = ...; definitions."""
    result = re.sub(r'\n\s*pub type \w+ = [^;]+;', '', content)
    return result


def remove_use_statements(content: str) -> str:
    """Remove use statements (they'll be replaced by pub use crate::types::*)."""
    result = re.sub(r'\n\s*use crate::types::\w+;', '', content)
    return result


def rewrite_iobuf_h_module(file_path: Path) -> bool:
    """Rewrite the iobuf_h module in the given file."""
    content = file_path.read_text()
    
    pattern = r'^(\s*)pub mod iobuf_h \{'
    match = re.search(pattern, content, re.MULTILINE)
    
    if not match:
        return False
    
    indent = match.group(1)
    module_start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        print(f"Error: Could not find closing brace for iobuf_h in {file_path}")
        return False
    
    module_content = content[brace_start + 1:brace_end]
    
    # Transform the module content
    new_module_content = module_content
    new_module_content = remove_struct_definitions(new_module_content)
    new_module_content = remove_type_aliases(new_module_content)
    new_module_content = remove_use_statements(new_module_content)
    
    # Add pub use at the beginning
    inner_indent = indent + "    "
    if file_path.name == "main.rs":
        types_path = "pgbouncer::types"
    else:
        types_path = "crate::types"
    
    use_line = f"\n{inner_indent}pub use {types_path}::*;"
    new_module_content = use_line + new_module_content
    
    # Build new module
    new_module = f"{indent}pub mod iobuf_h {{{new_module_content}\n{indent}}}"
    
    new_content = content[:module_start] + new_module + content[brace_end + 1:]
    
    file_path.write_text(new_content)
    return True


def main():
    src_dir = Path(__file__).parent.parent / "src"
    
    print("Rewriting iobuf_h modules...")
    print("=" * 60)
    
    updated = 0
    
    for file_path in sorted(src_dir.glob("*.rs")):
        if file_path.name == "lib.rs":
            continue
        if rewrite_iobuf_h_module(file_path):
            print(f"  Updated {file_path.name}")
            updated += 1
    
    for file_path in sorted((src_dir / "common").glob("*.rs")):
        if file_path.name == "types.rs":
            continue
        if rewrite_iobuf_h_module(file_path):
            print(f"  Updated common/{file_path.name}")
            updated += 1
    
    print("=" * 60)
    print(f"Updated {updated} files")


if __name__ == "__main__":
    main()
