#!/usr/bin/env python3
"""
Consolidate extern function modules by:
1. Removing pub mod X_h { } blocks that just declare extern functions
2. Updating imports to use crate::externs::X or inline the extern block
"""

import re
import sys
from pathlib import Path

# Modules that just declare extern functions and can be consolidated
# Format: module_name -> list of symbols it exports
EXTERN_MODULES = {
    'logging_h': ['LogLevel', 'LG_NOISE', 'LG_DEBUG', 'LG_INFO', 'LG_STATS', 'LG_WARNING', 'LG_ERROR', 'LG_FATAL', 'log_generic', 'log_warning', 'log_error', 'log_debug', 'log_noise', 'fatal'],
    '_string_h': ['memset', 'memcpy', 'memmove', 'memcmp', 'strlen', 'strcpy', 'strncpy', 'strcat', 'strncat', 'strcmp', 'strncmp', 'strchr', 'strrchr', 'strstr', 'strtok', 'strerror'],
    '_stdlib_h': ['exit', 'abort', 'malloc', 'free', 'realloc', 'calloc', 'atoi', 'atol', 'strtol', 'strtoul', 'getenv', 'setenv', 'unsetenv', 'qsort', 'bsearch', 'rand', 'srand'],
    '_malloc_h': ['malloc', 'free', 'realloc', 'calloc'],
    '_stdio_h': ['FILE', '__stderrp', '__stdoutp', 'fprintf', 'printf', 'snprintf', 'sprintf', 'fopen', 'fclose', 'fread', 'fwrite', 'fgets', 'fputs', 'fflush', 'feof', 'ferror'],
    '_ctype_h': ['isalpha', 'isdigit', 'isalnum', 'isspace', 'isupper', 'islower', 'toupper', 'tolower', 'isprint', 'isxdigit'],
    'errno_h': ['EIO', 'EAGAIN', 'EINPROGRESS', 'ENOENT', 'EACCES', 'EEXIST', '__error'],
    'unistd_h': ['read', 'write', 'close', 'sleep', 'usleep', 'getpid', 'getppid', 'getuid', 'geteuid', 'getgid', 'getegid', 'fork', 'execv', 'execve', 'chdir', 'getcwd', 'access', 'unlink', 'rmdir', 'pipe', 'dup', 'dup2'],
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

def remove_module(content: str, module_name: str) -> str:
    """Remove a pub mod X_h { ... } block."""
    pattern = rf'^pub mod {re.escape(module_name)} \{{'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        return content
    
    start = match.start()
    brace_start = match.end() - 1
    brace_end = find_matching_brace(content, brace_start)
    
    if brace_end == -1:
        print(f"Warning: Could not find matching brace for {module_name}")
        return content
    
    # Include trailing newlines
    end = brace_end + 1
    while end < len(content) and content[end] == '\n':
        end += 1
    
    return content[:start] + content[end:]

def process_file(filepath: Path, modules_to_remove: list, dry_run: bool = False) -> tuple[bool, list[str]]:
    """Process a single file. Returns (changed, messages)."""
    content = filepath.read_text()
    original = content
    messages = []
    
    is_main = filepath.name == 'main.rs'
    types_prefix = 'pgbouncer::types' if is_main else 'crate::types'
    
    for module_name in modules_to_remove:
        # Check if this module exists in the file
        if f'pub mod {module_name}' not in content:
            continue
        
        # Remove the module
        new_content = remove_module(content, module_name)
        if new_content != content:
            messages.append(f"  Removed {module_name}")
            content = new_content
        
        # Update imports: self::X_h::symbol -> crate::types::symbol
        # For now, we keep the imports but they'll be updated later
        # when we add the symbols to types.rs
    
    if content == original:
        return False, []
    
    if not dry_run:
        filepath.write_text(content)
    
    return True, messages

def main():
    dry_run = '--dry-run' in sys.argv
    modules = sys.argv[1:] if len(sys.argv) > 1 else []
    modules = [m for m in modules if m != '--dry-run']
    
    if not modules:
        print("Usage: python consolidate_extern_modules.py <module1> [module2] ... [--dry-run]")
        print(f"Available modules: {', '.join(EXTERN_MODULES.keys())}")
        return
    
    src_dir = Path(__file__).parent.parent / 'src'
    files = list(src_dir.glob('*.rs')) + list(src_dir.glob('common/*.rs'))
    
    changed = 0
    for filepath in sorted(files):
        was_changed, messages = process_file(filepath, modules, dry_run)
        if was_changed:
            action = "Would update" if dry_run else "Updated"
            print(f"{action}: {filepath}")
            for msg in messages:
                print(msg)
            changed += 1
    
    print(f"\n{'Would change' if dry_run else 'Changed'}: {changed} files")

if __name__ == '__main__':
    main()
