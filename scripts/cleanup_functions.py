#!/usr/bin/env python3
"""
Script to apply function cleanup patterns from stats.rs findings.

Usage:
    python scripts/cleanup_functions.py <file>           # Apply changes
    python scripts/cleanup_functions.py <file> --dry-run # Preview changes
    python scripts/cleanup_functions.py --all            # Process all src/*.rs files
    python scripts/cleanup_functions.py --all --dry-run  # Preview all

Transformations applied:
1. .wrapping_add(x) → += x (for simple cases)
2. .wrapping_sub(x) → -= x (for simple cases)
3. b"str\0" as *const u8 as *const c_char → c"str".as_ptr()
4. b"str\0" as *const u8 as *const ::core::ffi::c_char → c"str".as_ptr()
5. 0 as usec_t → 0 (and similar simple casts)
6. ::core::mem::zeroed() suggestions for large struct initialization
"""

import argparse
import os
import re
import sys
from pathlib import Path


def find_matching_paren(s: str, start: int) -> int:
    """Find the index of the closing paren matching the one at start."""
    depth = 1
    i = start + 1
    while i < len(s) and depth > 0:
        if s[i] == '(':
            depth += 1
        elif s[i] == ')':
            depth -= 1
        i += 1
    return i - 1 if depth == 0 else -1


def transform_wrapping_arithmetic(content: str) -> tuple[str, int]:
    """
    Transform .wrapping_add(x) to += x and .wrapping_sub(x) to -= x.
    Only for simple cases like (*ptr).field = (*ptr).field.wrapping_add(x)
    Handles nested parentheses properly.
    """
    count = 0
    
    # Pattern: (*ptr).field = (*ptr).field.wrapping_add(...)
    # We need to handle nested parens, so we match up to wrapping_add( then find matching )
    pattern_add = re.compile(r'\(\*(\w+)\)\.(\w+)\s*=\s*\(\*\1\)\.\2\.wrapping_add\(')
    pattern_sub = re.compile(r'\(\*(\w+)\)\.(\w+)\s*=\s*\(\*\1\)\.\2\.wrapping_sub\(')
    
    # Process wrapping_add
    while True:
        m = pattern_add.search(content)
        if not m:
            break
        # Find the matching closing paren
        paren_start = m.end() - 1  # Position of the opening (
        paren_end = find_matching_paren(content, paren_start)
        if paren_end == -1:
            break  # Malformed, skip
        
        expr = content[m.end():paren_end]
        replacement = f'(*{m.group(1)}).{m.group(2)} += ({expr})'
        content = content[:m.start()] + replacement + content[paren_end+1:]
        count += 1
    
    # Process wrapping_sub
    while True:
        m = pattern_sub.search(content)
        if not m:
            break
        paren_start = m.end() - 1
        paren_end = find_matching_paren(content, paren_start)
        if paren_end == -1:
            break
        
        expr = content[m.end():paren_end]
        replacement = f'(*{m.group(1)}).{m.group(2)} -= ({expr})'
        content = content[:m.start()] + replacement + content[paren_end+1:]
        count += 1
    
    return content, count


def transform_byte_string_casts(content: str) -> tuple[str, int]:
    """
    Transform b"str\0" as *const u8 as *const c_char → c"str".as_ptr()
    Note: b"\0" becomes c"" (empty C string), not c"\0" which is invalid.
    """
    count = 0
    
    def make_c_string(match_content: str) -> str:
        """Convert escaped content to C string literal, removing trailing \0."""
        # The regex already strips the trailing \0
        # But check for any remaining \0 which would be embedded nulls
        if '\\0' in match_content:
            # Embedded null - can't use c"..." literal, skip this match
            return None
        return match_content
    
    # Pattern with ::core::ffi::c_char
    pattern1 = r'b"([^"]*?)\\0"\s+as\s+\*const\s+u8\s+as\s+\*const\s+::core::ffi::c_char'
    
    def replace1(m):
        nonlocal count
        c_content = make_c_string(m.group(1))
        if c_content is None:
            return m.group(0)  # Keep original
        count += 1
        return f'c"{c_content}".as_ptr()'
    
    content = re.sub(pattern1, replace1, content)
    
    # Pattern with plain c_char
    pattern2 = r'b"([^"]*?)\\0"\s+as\s+\*const\s+u8\s+as\s+\*const\s+c_char'
    
    def replace2(m):
        nonlocal count
        c_content = make_c_string(m.group(1))
        if c_content is None:
            return m.group(0)  # Keep original
        count += 1
        return f'c"{c_content}".as_ptr()'
    
    content = re.sub(pattern2, replace2, content)
    
    return content, count


def transform_simple_casts(content: str) -> tuple[str, int]:
    """
    Transform simple casts like 0 as usec_t → 0
    Only for known safe cases.
    """
    count = 0
    
    # 0 as type → 0 (for numeric types)
    safe_types = ['usec_t', 'uint64_t', 'uint32_t', 'uint16_t', 'uint8_t',
                  'int64_t', 'int32_t', 'int16_t', 'int8_t', 'size_t', 'usize', 'isize']
    
    for typ in safe_types:
        pattern = rf'\b0\s+as\s+{typ}\b'
        new_content = re.sub(pattern, '0', content)
        if new_content != content:
            count += content.count(f'0 as {typ}') - new_content.count(f'0 as {typ}')
            content = new_content
    
    # NOTE: FFI type casts (0 as ::core::ffi::c_int, etc.) are NOT safe to
    # convert automatically because:
    # 1. The cast is often needed for type inference in expressions
    # 2. Removing them can cause type mismatches with surrounding code
    # These should be handled manually on a case-by-case basis.
    
    return content, count


def transform_redundant_wrapping(content: str) -> tuple[str, int]:
    """
    Transform .wrapping_sub(0) → just remove it, .wrapping_add(0) → remove.
    """
    count = 0
    
    # .wrapping_add(0) → remove
    pattern = r'\.wrapping_add\(\s*0\s*\)'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, '', content)
    
    # .wrapping_sub(0) → remove
    pattern = r'\.wrapping_sub\(\s*0\s*\)'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, '', content)
    
    return content, count


def transform_offset_zero(content: str) -> tuple[str, int]:
    """
    Transform .offset(0 as ::core::ffi::c_int as isize) to nothing (remove it).
    ptr.offset(0) == ptr, so this is a no-op.
    """
    count = 0
    
    # Pattern: .offset(0 as ::core::ffi::c_int as isize) → remove
    pattern = r'\.offset\(0\s+as\s+::core::ffi::c_int\s+as\s+isize\)'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, '', content)
    
    # Also handle simpler variant: .offset(0 as isize) → remove
    pattern = r'\.offset\(0\s+as\s+isize\)'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, '', content)
    
    # And even simpler: .offset(0) → remove
    pattern = r'\.offset\(0\)'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, '', content)
    
    # .offset(-(0 as ::core::ffi::c_ulong as isize)) → remove (it's -0 = 0)
    pattern = r'\.offset\(-\(0\s+as\s+::core::ffi::c_ulong\s+as\s+isize\)\)'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, '', content)
    
    return content, count


def transform_verbose_bool_checks(content: str) -> tuple[str, int]:
    """
    Transform verbose c2rust boolean patterns:
    - (expr) as ::core::ffi::c_int as ::core::ffi::c_long != 0 → expr
    
    This pattern appears when C code does: if (comparison) { ... }
    and c2rust converts it to checking if the comparison cast to int is non-zero.
    """
    count = 0
    
    # Pattern: (comparison) as c_int as c_long != 0 → comparison
    # We need to match balanced parentheses for the expression
    pattern = r'\(([^()]+)\)\s+as\s+::core::ffi::c_int\s+as\s+::core::ffi::c_long\s*!=\s*0'
    
    def replace_verbose(m):
        nonlocal count
        count += 1
        return m.group(1)
    
    content = re.sub(pattern, replace_verbose, content)
    
    return content, count


def transform_boolean_literals(content: str) -> tuple[str, int]:
    """
    Transform c2rust boolean patterns to idiomatic Rust.
    - true_0 != 0 → true
    - false_0 != 0 → false
    - true_0 == 0 → false (negated)
    - false_0 == 0 → true (negated)
    - = true_0; → = true;
    - = false_0; → = false;
    """
    count = 0
    
    # true_0 != 0 → true
    pattern = r'\btrue_0\s*!=\s*0\b'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, 'true', content)
    
    # false_0 != 0 → false
    pattern = r'\bfalse_0\s*!=\s*0\b'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, 'false', content)
    
    # true_0 == 0 → false
    pattern = r'\btrue_0\s*==\s*0\b'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, 'false', content)
    
    # false_0 == 0 → true
    pattern = r'\bfalse_0\s*==\s*0\b'
    matches = len(re.findall(pattern, content))
    if matches:
        count += matches
        content = re.sub(pattern, 'true', content)
    
    # NOTE: Assignment patterns (= true_0;) are NOT safe to convert automatically
    # because true_0/false_0 may be used as integers (c_int), not bools.
    # Only comparison patterns (true_0 != 0) are safe because they already
    # imply boolean semantics.
    
    return content, count


def analyze_struct_zeroing(content: str) -> list[tuple[int, str]]:
    """
    Find verbose struct initializations that could use ::core::mem::zeroed().
    Returns list of (line_number, suggestion) tuples.
    """
    suggestions = []
    lines = content.split('\n')
    
    # Look for patterns like:
    # let mut x = StructName {
    #     field1: 0,
    #     field2: 0,
    #     ...
    # };
    
    i = 0
    while i < len(lines):
        line = lines[i]
        match = re.search(r'let\s+mut\s+(\w+)\s*=\s*(\w+)\s*\{', line)
        if match:
            var_name = match.group(1)
            struct_name = match.group(2)
            
            # Count how many fields are just 0 or default
            zero_fields = 0
            j = i + 1
            while j < len(lines) and '}' not in lines[j]:
                if re.search(r':\s*0\s*,?\s*$', lines[j]) or \
                   re.search(r':\s*::core::ptr::null_mut\(\)', lines[j]):
                    zero_fields += 1
                j += 1
            
            # If more than 5 zero fields, suggest zeroed()
            if zero_fields > 5:
                suggestions.append((
                    i + 1,  # 1-indexed line number
                    f"Consider: let mut {var_name}: {struct_name} = ::core::mem::zeroed();"
                ))
            
            i = j
        else:
            i += 1
    
    return suggestions


def process_file(file_path: Path, dry_run: bool = False) -> dict:
    """Process a single file and return statistics."""
    content = file_path.read_text()
    original_content = content
    
    stats = {
        'file': str(file_path),
        'wrapping_arithmetic': 0,
        'byte_strings': 0,
        'simple_casts': 0,
        'redundant_wrapping': 0,
        'offset_zero': 0,
        'verbose_bool_checks': 0,
        'boolean_literals': 0,
        'zeroing_suggestions': [],
        'total_changes': 0,
    }
    
    # Apply transformations
    content, count = transform_wrapping_arithmetic(content)
    stats['wrapping_arithmetic'] = count
    
    content, count = transform_byte_string_casts(content)
    stats['byte_strings'] = count
    
    content, count = transform_simple_casts(content)
    stats['simple_casts'] = count
    
    content, count = transform_redundant_wrapping(content)
    stats['redundant_wrapping'] = count
    
    content, count = transform_offset_zero(content)
    stats['offset_zero'] = count
    
    content, count = transform_verbose_bool_checks(content)
    stats['verbose_bool_checks'] = count
    
    content, count = transform_boolean_literals(content)
    stats['boolean_literals'] = count
    
    # Analyze for zeroing suggestions (informational only)
    stats['zeroing_suggestions'] = analyze_struct_zeroing(content)
    
    stats['total_changes'] = (
        stats['wrapping_arithmetic'] + 
        stats['byte_strings'] + 
        stats['simple_casts'] +
        stats['redundant_wrapping'] +
        stats['offset_zero'] +
        stats['verbose_bool_checks'] +
        stats['boolean_literals']
    )
    
    # Write changes
    if content != original_content and not dry_run:
        file_path.write_text(content)
    
    return stats


def print_stats(stats: dict, dry_run: bool = False):
    """Print statistics for a processed file."""
    if stats['total_changes'] == 0 and not stats['zeroing_suggestions']:
        return
    
    prefix = "[DRY RUN] " if dry_run else ""
    print(f"\n{prefix}{stats['file']}:")
    
    if stats['wrapping_arithmetic']:
        print(f"  - Wrapping arithmetic: {stats['wrapping_arithmetic']} changes")
    if stats['byte_strings']:
        print(f"  - Byte string casts: {stats['byte_strings']} changes")
    if stats['simple_casts']:
        print(f"  - Simple casts: {stats['simple_casts']} changes")
    if stats['redundant_wrapping']:
        print(f"  - Redundant wrapping: {stats['redundant_wrapping']} changes")
    if stats['offset_zero']:
        print(f"  - Offset zero removal: {stats['offset_zero']} changes")
    if stats['verbose_bool_checks']:
        print(f"  - Verbose bool checks: {stats['verbose_bool_checks']} changes")
    if stats['boolean_literals']:
        print(f"  - Boolean literals: {stats['boolean_literals']} changes")
    
    if stats['zeroing_suggestions']:
        print(f"  - Zeroing suggestions ({len(stats['zeroing_suggestions'])}):")
        for line_no, suggestion in stats['zeroing_suggestions'][:3]:  # Show first 3
            print(f"      Line {line_no}: {suggestion}")
        if len(stats['zeroing_suggestions']) > 3:
            print(f"      ... and {len(stats['zeroing_suggestions']) - 3} more")


def main():
    parser = argparse.ArgumentParser(
        description='Apply function cleanup patterns to Rust files.'
    )
    parser.add_argument('file', nargs='?', help='File to process')
    parser.add_argument('--dry-run', '-n', action='store_true',
                        help='Preview changes without modifying files')
    parser.add_argument('--all', '-a', action='store_true',
                        help='Process all src/*.rs files')
    parser.add_argument('--src-dir', default='src',
                        help='Source directory (default: src)')
    
    args = parser.parse_args()
    
    # Find project root
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    src_dir = project_root / args.src_dir
    
    if not src_dir.exists():
        print(f"Error: Source directory not found: {src_dir}")
        sys.exit(1)
    
    # Collect files to process
    files = []
    if args.all:
        files = list(src_dir.glob('*.rs'))
        files.extend(src_dir.glob('common/*.rs'))
    elif args.file:
        file_path = Path(args.file)
        if not file_path.is_absolute():
            file_path = project_root / file_path
        if not file_path.exists():
            print(f"Error: File not found: {file_path}")
            sys.exit(1)
        files = [file_path]
    else:
        parser.print_help()
        sys.exit(1)
    
    # Process files
    total_changes = 0
    files_changed = 0
    
    for file_path in sorted(files):
        stats = process_file(file_path, args.dry_run)
        print_stats(stats, args.dry_run)
        total_changes += stats['total_changes']
        if stats['total_changes'] > 0:
            files_changed += 1
    
    # Summary
    print(f"\n{'=' * 60}")
    mode = "Would change" if args.dry_run else "Changed"
    print(f"Summary: {mode} {total_changes} items in {files_changed} files")
    
    if args.dry_run and total_changes > 0:
        print("\nRun without --dry-run to apply changes.")


if __name__ == '__main__':
    main()
