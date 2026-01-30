#!/usr/bin/env python3
"""
Script to convert static mut variables to thread-local RefCell.

IMPORTANT: This script should only be used AFTER a module has been fully
migrated from C to Rust (i.e., the .c file has been removed from the build).

If a static mut is still an extern declaration pointing to C code, converting
it will cause linker errors.

Usage:
    python scripts/convert_static_mut.py --analyze <file>     # Analyze a file
    python scripts/convert_static_mut.py --convert <var> <file>  # Convert a variable
    python scripts/convert_static_mut.py --dry-run ...        # Preview changes

Prerequisites:
    1. The variable must be DEFINED in Rust (with initialization), not extern
    2. The variable must NOT be used in any .c files
    3. The module's .c file must be removed from the Makefile

Example workflow:
    1. Remove src/stats.c from Makefile (after verifying Rust version works)
    2. Run: python scripts/convert_static_mut.py --analyze src/stats.rs
    3. Run: python scripts/convert_static_mut.py --convert old_stamp src/stats.rs
"""

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path


def find_project_root():
    """Find the project root (where Cargo.toml is)."""
    script_dir = Path(__file__).parent
    return script_dir.parent


def is_extern_declaration(line: str) -> bool:
    """Check if a static mut line is an extern declaration (no = initializer)."""
    # Extern declarations end with just the type and semicolon, no =
    line = line.strip()
    if 'static mut' not in line:
        return False
    # Has initializer = it's a definition
    if '=' in line:
        return False
    # Just "pub static mut foo: Type;" is an extern declaration
    return line.endswith(';')


def parse_static_mut(content: str) -> list[dict]:
    """
    Parse static mut declarations/definitions from file content.
    Returns list of dicts with: name, type, value (if defined), is_extern, line_number
    """
    results = []
    lines = content.split('\n')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        # Look for static mut at start of line (not in extern block)
        match = re.match(r'^(pub\s+)?static\s+mut\s+(\w+)\s*:\s*(.+)', line)
        if match:
            is_pub = bool(match.group(1))
            var_name = match.group(2)
            rest = match.group(3)
            
            # Check if it's a definition (has =) or declaration (just type;)
            if '=' in rest:
                # Definition - may span multiple lines
                type_match = re.match(r'([^=]+)\s*=\s*(.*)', rest)
                if type_match:
                    var_type = type_match.group(1).strip()
                    value_start = type_match.group(2)
                    
                    # Collect multi-line values
                    value_lines = [value_start]
                    j = i + 1
                    while j < len(lines) and not value_lines[-1].rstrip().endswith(';'):
                        value_lines.append(lines[j])
                        j += 1
                    
                    value = '\n'.join(value_lines).rstrip(';').strip()
                    
                    results.append({
                        'name': var_name,
                        'type': var_type,
                        'value': value,
                        'is_extern': False,
                        'is_pub': is_pub,
                        'line_number': i + 1,
                        'end_line': j,
                    })
                    i = j
                    continue
            else:
                # Declaration only (extern)
                type_part = rest.rstrip(';').strip()
                results.append({
                    'name': var_name,
                    'type': type_part,
                    'value': None,
                    'is_extern': True,
                    'is_pub': is_pub,
                    'line_number': i + 1,
                    'end_line': i + 1,
                })
        i += 1
    
    return results


def check_c_usage(var_name: str, project_root: Path) -> list[str]:
    """Check if a variable is used in any .c files."""
    try:
        result = subprocess.run(
            ['grep', '-rn', f'\\b{var_name}\\b', 'src/'],
            capture_output=True,
            text=True,
            cwd=project_root
        )
        c_files = []
        for line in result.stdout.split('\n'):
            if line and line.endswith('.c:'):
                c_files.append(line.split(':')[0])
            elif '.c:' in line:
                c_files.append(line.split(':')[0])
        return list(set(f for f in c_files if f.endswith('.c')))
    except Exception:
        return []


def generate_thread_local(var: dict) -> str:
    """Generate thread-local RefCell code for a variable."""
    name = var['name']
    typ = var['type']
    value = var['value']
    
    # Convert c_int etc to Rust types
    type_map = {
        '::core::ffi::c_int': 'i32',
        '::core::ffi::c_uint': 'u32',
        '::core::ffi::c_long': 'i64',
        '::core::ffi::c_ulong': 'u64',
        'c_int': 'i32',
        'c_uint': 'u32',
    }
    rust_type = type_map.get(typ, typ)
    
    # Convert value
    rust_value = value
    if value:
        # Simple conversions
        rust_value = re.sub(r'\b0\s+as\s+\S+', '0', value)
        rust_value = rust_value.strip()
    else:
        rust_value = 'Default::default()'
    
    # Generate the thread-local and accessors
    upper_name = name.upper()
    
    code = f'''thread_local! {{
    static {upper_name}: std::cell::RefCell<{rust_type}> = std::cell::RefCell::new({rust_value});
}}

pub fn get_{name}() -> {rust_type} {{
    {upper_name}.with(|v| *v.borrow())
}}

pub fn set_{name}(val: {rust_type}) {{
    {upper_name}.with(|v| *v.borrow_mut() = val);
}}
'''
    return code


def analyze_file(file_path: Path, project_root: Path):
    """Analyze a file for static mut variables."""
    content = file_path.read_text()
    variables = parse_static_mut(content)
    
    print(f"\nAnalyzing: {file_path.relative_to(project_root)}")
    print("=" * 60)
    
    definitions = [v for v in variables if not v['is_extern']]
    declarations = [v for v in variables if v['is_extern']]
    
    if declarations:
        print(f"\nExtern declarations (cannot convert - defined in C):")
        for v in declarations[:10]:
            print(f"  - {v['name']}: {v['type']} (line {v['line_number']})")
        if len(declarations) > 10:
            print(f"  ... and {len(declarations) - 10} more")
    
    if definitions:
        print(f"\nRust-defined static muts (potentially convertible):")
        for v in definitions:
            c_usage = check_c_usage(v['name'], project_root)
            status = "❌ Used in C" if c_usage else "✓ Rust-only"
            print(f"  - {v['name']}: {v['type']} [{status}] (line {v['line_number']})")
            if c_usage:
                print(f"      C files: {', '.join(c_usage)}")
    
    convertible = [v for v in definitions if not check_c_usage(v['name'], project_root)]
    print(f"\nSummary:")
    print(f"  - Total static mut: {len(variables)}")
    print(f"  - Extern (C-defined): {len(declarations)}")
    print(f"  - Rust-defined: {len(definitions)}")
    print(f"  - Convertible now: {len(convertible)}")
    
    return convertible


def convert_variable(var_name: str, file_path: Path, dry_run: bool = False):
    """Convert a specific static mut to thread-local RefCell."""
    content = file_path.read_text()
    variables = parse_static_mut(content)
    
    # Find the variable
    target = None
    for v in variables:
        if v['name'] == var_name:
            target = v
            break
    
    if not target:
        print(f"Error: Variable '{var_name}' not found in {file_path}")
        return False
    
    if target['is_extern']:
        print(f"Error: '{var_name}' is an extern declaration, cannot convert")
        return False
    
    # Generate replacement code
    new_code = generate_thread_local(target)
    
    print(f"\nConverting: {var_name}")
    print("-" * 40)
    print("Generated code:")
    print(new_code)
    
    if dry_run:
        print("\n[DRY RUN] Would need to:")
        print(f"  1. Replace static mut definition (lines {target['line_number']}-{target['end_line']})")
        print(f"  2. Update all usages of '{var_name}' to use get_{var_name}()/set_{var_name}()")
        return True
    
    # TODO: Implement actual replacement
    print("\nActual conversion not yet implemented - use --dry-run to preview")
    return False


def main():
    parser = argparse.ArgumentParser(description='Convert static mut to thread-local RefCell')
    parser.add_argument('--analyze', action='store_true', help='Analyze file for static muts')
    parser.add_argument('--convert', metavar='VAR', help='Convert specific variable')
    parser.add_argument('--dry-run', '-n', action='store_true', help='Preview changes')
    parser.add_argument('file', nargs='?', help='File to process')
    
    args = parser.parse_args()
    
    if not args.file:
        parser.print_help()
        sys.exit(1)
    
    project_root = find_project_root()
    file_path = Path(args.file)
    if not file_path.is_absolute():
        file_path = project_root / file_path
    
    if not file_path.exists():
        print(f"Error: File not found: {file_path}")
        sys.exit(1)
    
    if args.analyze:
        analyze_file(file_path, project_root)
    elif args.convert:
        convert_variable(args.convert, file_path, args.dry_run)
    else:
        # Default to analyze
        analyze_file(file_path, project_root)


if __name__ == '__main__':
    main()
