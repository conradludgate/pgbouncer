#!/usr/bin/env python3
"""
Script to consolidate duplicate c2rust-generated type modules.

Usage:
    python scripts/consolidate_module.py <module_name>           # Apply changes
    python scripts/consolidate_module.py <module_name> --dry-run # Preview changes
    python scripts/consolidate_module.py <module_name> --analyze # Show module contents
    python scripts/consolidate_module.py <module_name> --extract # Extract canonical definitions
    
Example:
    python scripts/consolidate_module.py iobuf_h --analyze

This script will:
1. Find all src/*.rs files containing `pub mod <module_name>`
2. Extract type definitions from the module
3. Add missing types to src/common/types.rs
4. Update imports to use crate::types:: or pgbouncer::types::
5. Remove the duplicate module blocks
"""

import argparse
import os
import re
import sys
from pathlib import Path
from dataclasses import dataclass, field
from typing import Optional
from collections import defaultdict


@dataclass
class ModuleInfo:
    """Information extracted from a module block."""
    file_path: Path
    module_start: int  # Line number where module starts
    module_end: int    # Line number where module ends
    module_content: str
    pub_use_start: Optional[int] = None  # Start line of `pub use self::module_h::`
    pub_use_end: Optional[int] = None    # End line (may be same as start for single-line)
    pub_use_types: list[str] = field(default_factory=list)
    inner_imports: list[tuple[int, str, list[str]]] = field(default_factory=list)  # (line_no, full_line, types)
    extern_decls: list[str] = field(default_factory=list)  # extern "C" declarations to move
    struct_defs: dict[str, str] = field(default_factory=dict)   # name -> full definition
    type_defs: dict[str, str] = field(default_factory=dict)     # name -> full definition  
    const_defs: dict[str, str] = field(default_factory=dict)    # name -> full definition
    fn_defs: dict[str, str] = field(default_factory=dict)       # name -> full definition
    use_stmts: list[str] = field(default_factory=list)          # use statements in module


def find_module_block(content: str, module_name: str) -> Optional[tuple[int, int, str]]:
    """Find the start and end of a `pub mod module_name { ... }` block."""
    lines = content.split('\n')
    
    # Find the module start
    module_start = None
    for i, line in enumerate(lines):
        if re.match(rf'^\s*pub\s+mod\s+{re.escape(module_name)}\s*\{{', line):
            module_start = i
            break
    
    if module_start is None:
        return None
    
    # Find matching closing brace
    brace_count = 0
    module_end = None
    in_module = False
    
    for i in range(module_start, len(lines)):
        line = lines[i]
        # Count braces (simple approach - doesn't handle braces in strings/comments perfectly)
        for char in line:
            if char == '{':
                brace_count += 1
                in_module = True
            elif char == '}':
                brace_count -= 1
                if in_module and brace_count == 0:
                    module_end = i
                    break
        if module_end is not None:
            break
    
    if module_end is None:
        return None
    
    module_content = '\n'.join(lines[module_start:module_end + 1])
    return (module_start, module_end, module_content)


def find_pub_use_line(content: str, module_name: str) -> Optional[tuple[int, int, str, list[str]]]:
    """Find `pub use self::module_name::{...};` statement (may span multiple lines).
    Returns (start_line, end_line, full_text, types).
    """
    lines = content.split('\n')
    
    for i, line in enumerate(lines):
        # Check if line starts a pub use for this module
        if re.match(rf'^\s*pub\s+use\s+self::{re.escape(module_name)}::', line):
            # Collect the full statement (may span multiple lines)
            stmt_lines = [line]
            end_line = i
            
            # Check if statement is complete (ends with ;)
            if not line.rstrip().endswith(';'):
                # Multi-line statement - find the end
                for j in range(i + 1, len(lines)):
                    stmt_lines.append(lines[j])
                    end_line = j
                    if lines[j].rstrip().endswith(';'):
                        break
            
            full_stmt = '\n'.join(stmt_lines)
            
            # Extract types from the statement
            # Handle both single type and multiple types in braces
            single_match = re.match(
                rf'pub\s+use\s+self::{re.escape(module_name)}::(\w+);',
                full_stmt.replace('\n', ' ')
            )
            if single_match:
                return (i, end_line, full_stmt, [single_match.group(1)])
            
            multi_match = re.search(r'\{([^}]+)\}', full_stmt)
            if multi_match:
                types_str = multi_match.group(1)
                types = [t.strip() for t in types_str.split(',') if t.strip()]
                return (i, end_line, full_stmt, types)
            
            # Fallback - just return the module name
            return (i, end_line, full_stmt, [])
    
    return None


def find_inner_imports(content: str, module_name: str) -> list[tuple[int, str, list[str]]]:
    """Find `use super::module_name::Type;` lines within inner modules."""
    lines = content.split('\n')
    results = []
    
    for i, line in enumerate(lines):
        # Match: use super::module_h::Type;
        match = re.match(rf'^\s*use\s+super::{re.escape(module_name)}::(\w+);', line)
        if match:
            results.append((i, line, [match.group(1)]))
            continue
        # Match: use super::module_h::{Type1, Type2};
        match = re.match(rf'^\s*use\s+super::{re.escape(module_name)}::\{{([^}}]+)\}};', line)
        if match:
            types = [t.strip() for t in match.group(1).split(',')]
            results.append((i, line, types))
    
    return results


def extract_struct_defs(module_content: str) -> dict[str, str]:
    """Extract struct definitions from module content, returning name -> definition."""
    results = {}
    # Match struct definitions including #[repr] and #[derive] attributes
    pattern = r'((?:#\[[^\]]+\]\s*)*pub\s+struct\s+(\w+)\s*\{[^}]+\})'
    for match in re.finditer(pattern, module_content, re.MULTILINE | re.DOTALL):
        full_def = match.group(1).strip()
        name = match.group(2)
        results[name] = full_def
    return results


def extract_type_defs(module_content: str) -> dict[str, str]:
    """Extract type alias definitions from module content."""
    results = {}
    pattern = r'(pub\s+type\s+(\w+)\s*=\s*[^;]+;)'
    for match in re.finditer(pattern, module_content):
        full_def = match.group(1).strip()
        name = match.group(2)
        results[name] = full_def
    return results


def extract_const_defs(module_content: str) -> dict[str, str]:
    """Extract const definitions from module content."""
    results = {}
    pattern = r'(pub\s+const\s+(\w+)\s*:\s*[^;]+;)'
    for match in re.finditer(pattern, module_content):
        full_def = match.group(1).strip()
        name = match.group(2)
        results[name] = full_def
    return results


def extract_fn_defs(module_content: str) -> dict[str, str]:
    """Extract function definitions from module content."""
    results = {}
    lines = module_content.split('\n')
    i = 0
    while i < len(lines):
        line = lines[i]
        # Look for function start with optional #[inline]
        if re.match(r'\s*#\[inline\]', line) or re.match(r'\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+', line):
            fn_lines = []
            # Collect attributes
            while i < len(lines) and re.match(r'\s*#\[', lines[i]):
                fn_lines.append(lines[i])
                i += 1
            # Collect function signature and body
            if i < len(lines) and re.match(r'\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+(\w+)', lines[i]):
                fn_match = re.match(r'\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+(\w+)', lines[i])
                fn_name = fn_match.group(1)
                brace_count = 0
                started = False
                while i < len(lines):
                    fn_lines.append(lines[i])
                    for char in lines[i]:
                        if char == '{':
                            brace_count += 1
                            started = True
                        elif char == '}':
                            brace_count -= 1
                    i += 1
                    if started and brace_count == 0:
                        break
                results[fn_name] = '\n'.join(fn_lines)
        else:
            i += 1
    return results


def extract_use_stmts(module_content: str) -> list[str]:
    """Extract use statements from module content."""
    results = []
    pattern = r'^\s*(use\s+[^;]+;)'
    for match in re.finditer(pattern, module_content, re.MULTILINE):
        results.append(match.group(1).strip())
    return results


def extract_extern_decls(module_content: str) -> list[str]:
    """Extract extern "C" function declarations."""
    # Match extern "C" { ... } blocks
    pattern = r'(extern\s+"C"\s*\{[^}]+\})'
    matches = re.findall(pattern, module_content, re.DOTALL)
    return matches


def check_type_in_types_rs(types_rs_path: Path, type_name: str) -> bool:
    """Check if a type is already defined in types.rs."""
    content = types_rs_path.read_text()
    # Look for struct/type/const definition
    patterns = [
        rf'pub\s+struct\s+{re.escape(type_name)}\s',
        rf'pub\s+type\s+{re.escape(type_name)}\s',
        rf'pub\s+const\s+{re.escape(type_name)}\s*:',
    ]
    for pattern in patterns:
        if re.search(pattern, content):
            return True
    return False


def get_types_from_pub_use(content: str, module_name: str) -> list[str]:
    """Get all types exported from a module via pub use."""
    result = find_pub_use_line(content, module_name)
    if result:
        return result[2]
    return []


def analyze_file(file_path: Path, module_name: str) -> Optional[ModuleInfo]:
    """Analyze a file to extract module information."""
    content = file_path.read_text()
    
    # Check if module exists
    module_block = find_module_block(content, module_name)
    if module_block is None:
        return None
    
    module_start, module_end, module_content = module_block
    
    info = ModuleInfo(
        file_path=file_path,
        module_start=module_start,
        module_end=module_end,
        module_content=module_content,
    )
    
    # Find pub use line
    pub_use_result = find_pub_use_line(content, module_name)
    if pub_use_result:
        info.pub_use_start = pub_use_result[0]
        info.pub_use_end = pub_use_result[1]
        info.pub_use_types = pub_use_result[3]
    
    # Find inner imports
    inner_imports = find_inner_imports(content, module_name)
    info.inner_imports = inner_imports
    
    # Extract definitions from module
    info.struct_defs = extract_struct_defs(module_content)
    info.type_defs = extract_type_defs(module_content)
    info.const_defs = extract_const_defs(module_content)
    info.fn_defs = extract_fn_defs(module_content)
    info.use_stmts = extract_use_stmts(module_content)
    info.extern_decls = extract_extern_decls(module_content)
    
    return info


def analyze_modules(infos: list[ModuleInfo], module_name: str):
    """Analyze and display module contents across all files."""
    print(f"\n{'='*70}")
    print(f"ANALYSIS: {module_name}")
    print(f"{'='*70}")
    
    # Collect all unique definitions
    all_structs = defaultdict(list)  # name -> [(file, definition), ...]
    all_types = defaultdict(list)
    all_consts = defaultdict(list)
    all_fns = defaultdict(list)
    all_uses = defaultdict(list)
    
    for info in infos:
        for name, defn in info.struct_defs.items():
            all_structs[name].append((info.file_path.name, defn))
        for name, defn in info.type_defs.items():
            all_types[name].append((info.file_path.name, defn))
        for name, defn in info.const_defs.items():
            all_consts[name].append((info.file_path.name, defn))
        for name, defn in info.fn_defs.items():
            all_fns[name].append((info.file_path.name, defn))
        for use_stmt in info.use_stmts:
            all_uses[use_stmt].append(info.file_path.name)
    
    # Print structs
    if all_structs:
        print(f"\n## STRUCTS ({len(all_structs)} unique)")
        for name, occurrences in sorted(all_structs.items()):
            files = [f for f, _ in occurrences]
            print(f"\n  {name} (in {len(occurrences)} files: {', '.join(files[:3])}{'...' if len(files) > 3 else ''})")
            # Show first definition
            _, defn = occurrences[0]
            for line in defn.split('\n')[:10]:
                print(f"    {line}")
            if len(defn.split('\n')) > 10:
                print(f"    ... ({len(defn.split(chr(10)))} lines total)")
    
    # Print type aliases
    if all_types:
        print(f"\n## TYPE ALIASES ({len(all_types)} unique)")
        for name, occurrences in sorted(all_types.items()):
            files = [f for f, _ in occurrences]
            _, defn = occurrences[0]
            print(f"  {name} (in {len(occurrences)} files): {defn}")
    
    # Print constants
    if all_consts:
        print(f"\n## CONSTANTS ({len(all_consts)} unique)")
        for name, occurrences in sorted(all_consts.items()):
            files = [f for f, _ in occurrences]
            _, defn = occurrences[0]
            print(f"  {name} (in {len(occurrences)} files): {defn}")
    
    # Print functions
    if all_fns:
        print(f"\n## FUNCTIONS ({len(all_fns)} unique)")
        for name, occurrences in sorted(all_fns.items()):
            files = [f for f, _ in occurrences]
            print(f"\n  {name}() (in {len(occurrences)} files: {', '.join(files[:3])}{'...' if len(files) > 3 else ''})")
            # Show first function signature
            _, defn = occurrences[0]
            lines = defn.split('\n')
            # Find the fn line
            for line in lines:
                if 'fn ' in line:
                    print(f"    {line.strip()}")
                    break
    
    # Print use statements
    if all_uses:
        print(f"\n## DEPENDENCIES (use statements)")
        for use_stmt, files in sorted(all_uses.items()):
            print(f"  {use_stmt}")
    
    print(f"\n{'='*70}")
    print("SUMMARY")
    print(f"{'='*70}")
    print(f"  Files with module: {len(infos)}")
    print(f"  Unique structs: {len(all_structs)}")
    print(f"  Unique type aliases: {len(all_types)}")
    print(f"  Unique constants: {len(all_consts)}")
    print(f"  Unique functions: {len(all_fns)}")
    print(f"  Unique dependencies: {len(all_uses)}")
    
    return all_structs, all_types, all_consts, all_fns, all_uses


def extract_canonical_definitions(infos: list[ModuleInfo], module_name: str) -> str:
    """Extract canonical definitions to add to types.rs."""
    all_structs, all_types, all_consts, all_fns, all_uses = analyze_modules(infos, module_name)
    
    output = []
    output.append(f"// === {module_name} types ===")
    output.append("")
    
    # Add type aliases first (they often reference structs)
    for name, occurrences in sorted(all_types.items()):
        _, defn = occurrences[0]
        output.append(defn)
    
    if all_types:
        output.append("")
    
    # Add structs
    for name, occurrences in sorted(all_structs.items()):
        _, defn = occurrences[0]
        output.append(defn)
        output.append("")
    
    # Add constants
    for name, occurrences in sorted(all_consts.items()):
        _, defn = occurrences[0]
        output.append(defn)
    
    if all_consts:
        output.append("")
    
    # Add functions
    for name, occurrences in sorted(all_fns.items()):
        _, defn = occurrences[0]
        output.append(defn)
        output.append("")
    
    return '\n'.join(output)


def get_crate_prefix(file_path: Path) -> str:
    """Determine if file uses crate:: or pgbouncer:: prefix."""
    if file_path.name == 'main.rs':
        return 'pgbouncer'
    return 'crate'


def generate_new_pub_use(types: list[str], prefix: str) -> str:
    """Generate a new pub use statement."""
    if len(types) == 1:
        return f'pub use {prefix}::types::{types[0]};'
    else:
        types_str = ', '.join(types)
        return f'pub use {prefix}::types::{{{types_str}}};'


def generate_new_inner_import(types: list[str], prefix: str, indent: str = '    ') -> str:
    """Generate a new inner import statement."""
    if len(types) == 1:
        return f'{indent}use {prefix}::types::{types[0]};'
    else:
        types_str = ', '.join(types)
        return f'{indent}use {prefix}::types::{{{types_str}}};'


def update_file(info: ModuleInfo, module_name: str, dry_run: bool = False) -> tuple[int, int]:
    """
    Update a file to consolidate the module.
    Returns (lines_removed, lines_added).
    """
    content = info.file_path.read_text()
    lines = content.split('\n')
    prefix = get_crate_prefix(info.file_path)
    
    lines_removed = 0
    lines_added = 0
    
    # Track lines to remove (we'll do this in reverse order)
    removals = set()
    replacements = {}
    insertions = []  # (after_line, content)
    
    # Get extern function names so we can filter them from pub use
    extern_fns = extract_extern_block_functions(info.module_content)
    extern_fn_names = {fn_name for fn_name, _ in extern_fns}
    
    # 1. Update pub use statement (may span multiple lines, filter out extern functions)
    if info.pub_use_start is not None:
        old_lines_range = f"{info.pub_use_start + 1}-{info.pub_use_end + 1}" if info.pub_use_start != info.pub_use_end else str(info.pub_use_start + 1)
        old_text = lines[info.pub_use_start].strip()
        
        # Filter out extern function names from types
        types_only = [t for t in info.pub_use_types if t not in extern_fn_names]
        
        # Mark all lines of the pub use statement for removal
        for i in range(info.pub_use_start, info.pub_use_end + 1):
            removals.add(i)
        
        if types_only:
            new_line = generate_new_pub_use(types_only, prefix)
            # Replace first line, rest will be removed
            replacements[info.pub_use_start] = new_line
            print(f"  Replace lines {old_lines_range}: {old_text}...")
            print(f"     With: {new_line}")
        else:
            # All were extern functions, just remove
            print(f"  Remove lines {old_lines_range}: {old_text}... (only contained extern functions)")
    
    # 2. Update inner imports (filter out extern functions)
    for line_no, old_line, types in info.inner_imports:
        indent = len(old_line) - len(old_line.lstrip())
        types_only = [t for t in types if t not in extern_fn_names]
        if types_only:
            new_line = generate_new_inner_import(types_only, prefix, ' ' * indent)
            replacements[line_no] = new_line
            print(f"  Replace line {line_no + 1}: {old_line.strip()}")
            print(f"     With: {new_line.strip()}")
        else:
            # All were extern functions, remove the line
            removals.add(line_no)
            print(f"  Remove line {line_no + 1}: {old_line.strip()} (only contained extern functions)")
    
    # 3. Mark module block for removal
    for i in range(info.module_start, info.module_end + 1):
        removals.add(i)
    lines_removed += (info.module_end - info.module_start + 1)
    print(f"  Remove module block: lines {info.module_start + 1}-{info.module_end + 1} ({lines_removed} lines)")
    
    # 4. Handle extern declarations - extract and move to top level
    extern_fns = extract_extern_block_functions(info.module_content)
    if extern_fns:
        print(f"  Moving {len(extern_fns)} extern function declarations to top level")
        
        # Update type references in each function declaration
        updated_fns = []
        for fn_name, fn_decl in extern_fns:
            updated_decl = update_extern_decl_types(fn_decl, prefix, module_name)
            updated_fns.append(updated_decl)
            print(f"    - {fn_name}()")
        
        # Generate new extern block
        extern_block = generate_extern_block(updated_fns)
        
        # Find a good place to insert - after the module block we're removing
        # This ensures we don't insert inside another statement
        insert_after = info.module_end
        
        insertions.append((insert_after, extern_block))
        lines_added += len(extern_block.split('\n'))
    
    if dry_run:
        print(f"  [DRY RUN] Would modify {info.file_path}")
        if extern_fns:
            print(f"  [DRY RUN] Would add extern block:")
            for line in extern_block.split('\n')[:5]:
                print(f"    {line}")
            if len(extern_block.split('\n')) > 5:
                print(f"    ... ({len(extern_block.split(chr(10)))} lines)")
        return (lines_removed, lines_added)
    
    # Apply changes
    new_lines = []
    for i, line in enumerate(lines):
        if i in removals:
            # Check if this line has a replacement (e.g., first line of multi-line pub use)
            if i in replacements:
                new_lines.append(replacements[i])
            continue
        if i in replacements:
            new_lines.append(replacements[i])
        else:
            new_lines.append(line)
    
    # Append extern blocks at the end of the file
    for _, insert_content in insertions:
        new_lines.append('')
        new_lines.append(insert_content)
    
    new_content = '\n'.join(new_lines)
    info.file_path.write_text(new_content)
    print(f"  Updated {info.file_path}")
    
    return (lines_removed, lines_added)


def update_extern_decl_types(decl: str, prefix: str, module_name: str) -> str:
    """Update type references in extern declarations."""
    # Replace patterns like module_h::Type with prefix::types::Type
    decl = re.sub(rf'{re.escape(module_name)}::(\w+)', rf'{prefix}::types::\1', decl)
    # Also replace super::module_h::Type patterns
    decl = re.sub(rf'super::{re.escape(module_name)}::(\w+)', rf'{prefix}::types::\1', decl)
    return decl


def extract_extern_block_functions(module_content: str) -> list[tuple[str, str]]:
    """Extract individual function declarations from extern "C" blocks.
    Returns list of (fn_name, full_declaration_with_signature).
    """
    results = []
    # Find extern "C" blocks
    for block_match in re.finditer(r'extern\s+"C"\s*\{([^}]+)\}', module_content, re.DOTALL):
        block_content = block_match.group(1)
        # Find individual function declarations
        for fn_match in re.finditer(
            r'((?:#\[[^\]]+\]\s*)*pub\s+fn\s+(\w+)[^;]+;)',
            block_content,
            re.DOTALL
        ):
            full_decl = fn_match.group(1).strip()
            fn_name = fn_match.group(2)
            results.append((fn_name, full_decl))
    return results


def generate_extern_block(functions: list[str], indent: str = '') -> str:
    """Generate an extern "C" block with the given function declarations."""
    if not functions:
        return ''
    lines = [f'{indent}extern "C" {{']
    for fn_decl in functions:
        # Indent each line of the declaration
        for line in fn_decl.split('\n'):
            lines.append(f'{indent}    {line.strip()}')
    lines.append(f'{indent}}}')
    return '\n'.join(lines)


def collect_all_types(infos: list[ModuleInfo]) -> tuple[set[str], set[str]]:
    """Collect all unique types and extern function names from all files.
    Returns (types_to_add_to_types_rs, extern_function_names).
    """
    all_exported = set()
    all_extern_fns = set()
    
    for info in infos:
        all_exported.update(info.pub_use_types)
        for _, _, types in info.inner_imports:
            all_exported.update(types)
        # Collect extern function names
        extern_fns = extract_extern_block_functions(info.module_content)
        for fn_name, _ in extern_fns:
            all_extern_fns.add(fn_name)
    
    # Types are exports that are NOT extern functions
    types_only = all_exported - all_extern_fns
    
    return types_only, all_extern_fns


def main():
    parser = argparse.ArgumentParser(
        description='Consolidate duplicate c2rust type modules'
    )
    parser.add_argument('module_name', help='Name of the module to consolidate (e.g., iobuf_h)')
    parser.add_argument('--dry-run', '-n', action='store_true',
                        help='Show what would be done without making changes')
    parser.add_argument('--analyze', '-a', action='store_true',
                        help='Analyze module contents across all files')
    parser.add_argument('--extract', '-e', action='store_true',
                        help='Extract canonical definitions to stdout (for adding to types.rs)')
    parser.add_argument('--src-dir', default='src',
                        help='Source directory (default: src)')
    parser.add_argument('--types-file', default='src/common/types.rs',
                        help='Path to types.rs (default: src/common/types.rs)')
    parser.add_argument('--force', '-f', action='store_true',
                        help='Skip confirmation prompts')
    
    args = parser.parse_args()
    
    # Find project root (where Cargo.toml is)
    project_root = Path.cwd()
    while not (project_root / 'Cargo.toml').exists():
        if project_root.parent == project_root:
            print("Error: Could not find Cargo.toml in parent directories")
            sys.exit(1)
        project_root = project_root.parent
    
    src_dir = project_root / args.src_dir
    types_file = project_root / args.types_file
    
    # Find all .rs files with the module
    rs_files = list(src_dir.glob('*.rs'))
    
    infos = []
    for rs_file in sorted(rs_files):
        info = analyze_file(rs_file, args.module_name)
        if info:
            infos.append(info)
    
    if not infos:
        print(f"No files found containing module '{args.module_name}'")
        sys.exit(0)
    
    # Handle --analyze mode
    if args.analyze:
        analyze_modules(infos, args.module_name)
        sys.exit(0)
    
    # Handle --extract mode
    if args.extract:
        output = extract_canonical_definitions(infos, args.module_name)
        print("\n" + "="*70)
        print("CANONICAL DEFINITIONS (add to types.rs)")
        print("="*70 + "\n")
        print(output)
        sys.exit(0)
    
    print(f"Consolidating module: {args.module_name}")
    print(f"Source directory: {src_dir}")
    print(f"Types file: {types_file}")
    print()
    
    print(f"Found {len(infos)} files with module '{args.module_name}':")
    for info in infos:
        print(f"  - {info.file_path.name}")
    print()
    
    # Collect all types that need to be in types.rs
    types_only, extern_fns = collect_all_types(infos)
    print(f"Types to consolidate: {', '.join(sorted(types_only))}")
    if extern_fns:
        print(f"Extern functions (will be moved to top-level): {', '.join(sorted(extern_fns))}")
    print()
    
    # Check which types are missing from types.rs
    if types_file.exists():
        missing_types = []
        for t in types_only:
            if not check_type_in_types_rs(types_file, t):
                missing_types.append(t)
        
        if missing_types:
            print(f"WARNING: These types are NOT in {types_file}:")
            for t in missing_types:
                print(f"  - {t}")
            print()
            print("Run with --extract to see definitions to add.")
            print("You may need to add them manually before running this script.")
            if not args.dry_run and not args.force:
                response = input("Continue anyway? [y/N] ")
                if response.lower() != 'y':
                    sys.exit(1)
    else:
        print(f"WARNING: {types_file} does not exist")
    
    print()
    
    # Process each file
    total_removed = 0
    total_added = 0
    
    for info in infos:
        print(f"Processing {info.file_path.name}:")
        removed, added = update_file(info, args.module_name, dry_run=args.dry_run)
        total_removed += removed
        total_added += added
        print()
    
    print("=" * 60)
    print(f"Total lines removed: {total_removed}")
    print(f"Total lines added: {total_added}")
    print(f"Net savings: ~{total_removed - total_added} lines")
    
    if args.dry_run:
        print()
        print("This was a dry run. Use without --dry-run to apply changes.")


if __name__ == '__main__':
    main()
