#!/usr/bin/env python3
"""
Merge all instances of a module from all files into a single canonical definition.

This script:
1. Finds all instances of pub mod X_h { ... } across all files
2. Extracts all unique items (types, constants, functions, structs)
3. Generates a merged module definition for types.rs
4. Optionally applies the changes
"""

import re
import sys
from pathlib import Path
from collections import defaultdict

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
    
    # Extract just the inner content (between braces)
    inner_content = content[brace_start + 1:brace_end].strip()
    return inner_content, start, brace_end + 1

def parse_module_items(content: str) -> dict:
    """Parse module content into categorized items."""
    items = {
        'extern_types': [],      # extern "C" { pub type X; }
        'extern_fns': [],        # extern "C" { pub fn X(...); }
        'extern_statics': [],    # extern "C" { pub static mut X; }
        'type_aliases': [],      # pub type X = Y;
        'constants': [],         # pub const X: T = V;
        'structs': [],           # pub struct X { ... }
        'unions': [],            # pub union X { ... }
        'uses': [],              # use X::Y;
        'other': [],             # anything else
    }
    
    # Remove use statements first (we'll handle them separately)
    use_pattern = r'^\s*use\s+[^;]+;'
    for match in re.finditer(use_pattern, content, re.MULTILINE):
        items['uses'].append(match.group().strip())
    content = re.sub(use_pattern, '', content, flags=re.MULTILINE)
    
    # Find extern "C" blocks
    extern_pattern = r'extern\s+"C"\s*\{'
    extern_matches = list(re.finditer(extern_pattern, content))
    
    for match in extern_matches:
        brace_start = match.end() - 1
        brace_end = find_matching_brace(content, brace_start)
        if brace_end == -1:
            continue
        
        extern_content = content[brace_start + 1:brace_end]
        
        # Parse extern types
        for type_match in re.finditer(r'pub\s+type\s+(\w+)\s*;', extern_content):
            items['extern_types'].append(type_match.group(1))
        
        # Parse extern functions
        fn_pattern = r'pub\s+fn\s+(\w+)\s*\([^)]*\)[^;]*;'
        for fn_match in re.finditer(fn_pattern, extern_content, re.DOTALL):
            # Get the full function declaration
            fn_start = fn_match.start()
            fn_end = fn_match.end()
            fn_decl = extern_content[fn_start:fn_end].strip()
            items['extern_fns'].append(fn_decl)
        
        # Parse extern statics
        static_pattern = r'pub\s+static\s+(?:mut\s+)?(\w+)\s*:[^;]+;'
        for static_match in re.finditer(static_pattern, extern_content):
            items['extern_statics'].append(static_match.group().strip())
    
    # Remove extern blocks from content for further parsing
    for match in reversed(extern_matches):
        brace_start = match.end() - 1
        brace_end = find_matching_brace(content, brace_start)
        if brace_end != -1:
            content = content[:match.start()] + content[brace_end + 1:]
    
    # Parse type aliases
    type_pattern = r'pub\s+type\s+(\w+)\s*=\s*[^;]+;'
    for match in re.finditer(type_pattern, content):
        items['type_aliases'].append(match.group().strip())
    
    # Parse constants
    const_pattern = r'pub\s+const\s+(\w+)\s*:[^=]+=\s*[^;]+;'
    for match in re.finditer(const_pattern, content):
        items['constants'].append(match.group().strip())
    
    # Parse structs (simplified - just capture the whole thing)
    struct_pattern = r'(?:#\[derive[^\]]+\]\s*)*(?:#\[repr[^\]]+\]\s*)*pub\s+struct\s+(\w+)\s*\{[^}]+\}'
    for match in re.finditer(struct_pattern, content, re.DOTALL):
        items['structs'].append(match.group().strip())
    
    # Parse unions
    union_pattern = r'(?:#\[derive[^\]]+\]\s*)*(?:#\[repr[^\]]+\]\s*)*pub\s+union\s+(\w+)\s*\{[^}]+\}'
    for match in re.finditer(union_pattern, content, re.DOTALL):
        items['unions'].append(match.group().strip())
    
    return items

def merge_items(all_items: list[dict]) -> dict:
    """Merge items from multiple modules, keeping unique items."""
    merged = {
        'extern_types': set(),
        'extern_fns': set(),
        'extern_statics': set(),
        'type_aliases': set(),
        'constants': set(),
        'structs': {},  # name -> definition (keep longest/most complete)
        'unions': {},
        'uses': set(),
    }
    
    for items in all_items:
        merged['extern_types'].update(items['extern_types'])
        merged['extern_fns'].update(items['extern_fns'])
        merged['extern_statics'].update(items['extern_statics'])
        merged['type_aliases'].update(items['type_aliases'])
        merged['constants'].update(items['constants'])
        merged['uses'].update(items['uses'])
        
        # For structs/unions, keep the longest definition (most complete)
        for struct in items['structs']:
            name_match = re.search(r'struct\s+(\w+)', struct)
            if name_match:
                name = name_match.group(1)
                if name not in merged['structs'] or len(struct) > len(merged['structs'][name]):
                    merged['structs'][name] = struct
        
        for union in items['unions']:
            name_match = re.search(r'union\s+(\w+)', union)
            if name_match:
                name = name_match.group(1)
                if name not in merged['unions'] or len(union) > len(merged['unions'][name]):
                    merged['unions'][name] = union
    
    return merged

def get_existing_symbols(types_file: Path) -> set:
    """Get symbols already defined in types.rs to avoid duplicates."""
    content = types_file.read_text()
    symbols = set()
    
    # Find extern types
    for match in re.finditer(r'pub\s+type\s+(\w+)\s*;', content):
        symbols.add(match.group(1))
    
    # Find type aliases
    for match in re.finditer(r'pub\s+type\s+(\w+)\s*=', content):
        symbols.add(match.group(1))
    
    # Find constants
    for match in re.finditer(r'pub\s+const\s+(\w+)\s*:', content):
        symbols.add(match.group(1))
    
    # Find functions (extern declarations)
    for match in re.finditer(r'pub\s+fn\s+(\w+)\s*\(', content):
        symbols.add(match.group(1))
    
    # Find structs
    for match in re.finditer(r'pub\s+struct\s+(\w+)', content):
        symbols.add(match.group(1))
    
    # Find pub use re-exports (e.g., pub use libc::socket;)
    for match in re.finditer(r'pub\s+use\s+\w+::(\w+)\s*;', content):
        symbols.add(match.group(1))
    
    return symbols

def generate_merged_module(module_name: str, merged: dict, existing_symbols: set = None) -> str:
    """Generate the merged module definition, skipping existing symbols."""
    if existing_symbols is None:
        existing_symbols = set()
    
    lines = [f"// ============================================================================="]
    lines.append(f"// {module_name} (merged from all files)")
    lines.append(f"// =============================================================================")
    lines.append("")
    
    # Type aliases (skip existing)
    new_type_aliases = []
    for ta in sorted(merged['type_aliases']):
        match = re.match(r'pub\s+type\s+(\w+)', ta)
        if match and match.group(1) not in existing_symbols:
            new_type_aliases.append(ta)
    if new_type_aliases:
        for ta in new_type_aliases:
            lines.append(ta)
        lines.append("")
    
    # Constants (skip existing)
    new_constants = []
    for const in sorted(merged['constants']):
        match = re.match(r'pub\s+const\s+(\w+)', const)
        if match and match.group(1) not in existing_symbols:
            new_constants.append(const)
    if new_constants:
        for const in new_constants:
            lines.append(const)
        lines.append("")
    
    # Structs (skip existing)
    new_structs = {k: v for k, v in merged['structs'].items() if k not in existing_symbols}
    if new_structs:
        for name in sorted(new_structs.keys()):
            lines.append(new_structs[name])
            lines.append("")
    
    # Unions (skip existing)
    new_unions = {k: v for k, v in merged['unions'].items() if k not in existing_symbols}
    if new_unions:
        for name in sorted(new_unions.keys()):
            lines.append(new_unions[name])
            lines.append("")
    
    # Extern block for types and functions (skip existing)
    new_extern_types = [et for et in merged['extern_types'] if et not in existing_symbols]
    new_extern_fns = []
    for ef in merged['extern_fns']:
        match = re.search(r'pub\s+fn\s+(\w+)', ef)
        if match and match.group(1) not in existing_symbols:
            new_extern_fns.append(ef)
    new_extern_statics = []
    for es in merged['extern_statics']:
        match = re.search(r'(?:static\s+(?:mut\s+)?)?(\w+)\s*:', es)
        if match and match.group(1) not in existing_symbols:
            new_extern_statics.append(es)
    
    if new_extern_types or new_extern_fns or new_extern_statics:
        lines.append("extern \"C\" {")
        
        for et in sorted(new_extern_types):
            lines.append(f"    pub type {et};")
        
        for es in sorted(new_extern_statics):
            lines.append(f"    {es}")
        
        for ef in sorted(new_extern_fns):
            # Indent the function
            indented = '\n'.join('    ' + line if line.strip() else '' for line in ef.split('\n'))
            lines.append(indented)
        
        lines.append("}")
        lines.append("")
    
    return '\n'.join(lines)

def get_exported_symbols(merged: dict) -> list[str]:
    """Get list of all symbols that would be exported."""
    symbols = []
    symbols.extend(merged['extern_types'])
    
    # Extract names from type aliases
    for ta in merged['type_aliases']:
        match = re.match(r'pub\s+type\s+(\w+)', ta)
        if match:
            symbols.append(match.group(1))
    
    # Extract names from constants
    for const in merged['constants']:
        match = re.match(r'pub\s+const\s+(\w+)', const)
        if match:
            symbols.append(match.group(1))
    
    # Struct names
    symbols.extend(merged['structs'].keys())
    
    # Union names
    symbols.extend(merged['unions'].keys())
    
    # Function names
    for ef in merged['extern_fns']:
        match = re.search(r'pub\s+fn\s+(\w+)', ef)
        if match:
            symbols.append(match.group(1))
    
    return sorted(set(symbols))

def main():
    if len(sys.argv) < 2:
        print("Usage: python merge_module.py <module_name> [--apply]")
        print("Example: python merge_module.py tls_h")
        print("         python merge_module.py logging_h --apply")
        return
    
    module_name = sys.argv[1]
    apply_changes = '--apply' in sys.argv
    
    src_dir = Path(__file__).parent.parent / 'src'
    files = list(src_dir.glob('*.rs')) + list(src_dir.glob('common/*.rs'))
    
    print(f"Analyzing module: {module_name}")
    print("=" * 60)
    
    all_items = []
    file_locations = []
    
    for filepath in sorted(files):
        content = filepath.read_text()
        result = extract_module_content(content, module_name)
        if result:
            inner_content, start, end = result
            items = parse_module_items(inner_content)
            all_items.append(items)
            file_locations.append((filepath, start, end))
            
            # Summary for this file
            item_count = (
                len(items['extern_types']) + len(items['extern_fns']) + 
                len(items['type_aliases']) + len(items['constants']) +
                len(items['structs']) + len(items['unions'])
            )
            print(f"  {filepath.name}: {item_count} items")
    
    if not all_items:
        print(f"No instances of {module_name} found.")
        return
    
    print(f"\nFound {len(all_items)} instances of {module_name}")
    print("=" * 60)
    
    # Merge all items
    merged = merge_items(all_items)
    
    # Get existing symbols from types.rs to avoid duplicates
    types_file = src_dir / 'common' / 'types.rs'
    existing_symbols = get_existing_symbols(types_file)
    
    # Generate merged module
    merged_code = generate_merged_module(module_name, merged, existing_symbols)
    
    print("\nMerged module content:")
    print("-" * 60)
    print(merged_code)
    print("-" * 60)
    
    # List all exported symbols
    symbols = get_exported_symbols(merged)
    print(f"\nExported symbols ({len(symbols)}):")
    for sym in symbols:
        print(f"  - {sym}")
    
    if apply_changes:
        print("\n--apply flag detected. Applying changes...")
        
        # 1. Add merged content to types.rs
        types_file = src_dir / 'common' / 'types.rs'
        types_content = types_file.read_text()
        
        # Find the insertion point (before the final pgbouncer-specific section)
        insertion_marker = "// =============================================================================\n// pgbouncer-specific forward declarations"
        if insertion_marker in types_content:
            insert_pos = types_content.find(insertion_marker)
            new_types_content = types_content[:insert_pos] + merged_code + "\n" + types_content[insert_pos:]
            types_file.write_text(new_types_content)
            print(f"  Added merged content to {types_file}")
        else:
            print(f"  Warning: Could not find insertion point in types.rs")
            print(f"  Please add the merged content manually")
        
        # 2. Update imports and remove modules from each file
        is_main = lambda p: p.name == 'main.rs'
        
        for filepath, start, end in file_locations:
            content = filepath.read_text()
            types_prefix = 'pgbouncer::types' if is_main(filepath) else 'crate::types'
            
            # Update imports: self::module_name::X -> types_prefix::X
            # and super::module_name::X -> types_prefix::X
            for sym in symbols:
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
            
            # Also update pub use statements
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
            
            # Also update super::module_name:: patterns inside nested modules
            content = re.sub(
                rf'use super::{re.escape(module_name)}::',
                f'use {types_prefix}::',
                content
            )
            
            # Remove the module (need to re-find it since content changed)
            result = extract_module_content(content, module_name)
            if result:
                _, mod_start, mod_end = result
                # Include trailing newlines
                while mod_end < len(content) and content[mod_end] == '\n':
                    mod_end += 1
                content = content[:mod_start] + content[mod_end:]
            
            filepath.write_text(content)
            print(f"  Updated {filepath.name}")
        
        print(f"\nApplied changes to {len(file_locations)} files + types.rs")
        print("Run 'cargo build' to verify")
    else:
        print("\nTo apply changes, run with --apply flag")
        print("Or copy the merged module content to src/common/types.rs")

if __name__ == '__main__':
    main()
