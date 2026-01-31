# Refactoring Scripts

Documentation for scripts in the `scripts/` directory.

## merge_module.py ⭐ Recommended

Collects ALL items from ALL instances of a module across files and merges them into `types.rs`.

```bash
# Preview what would be merged
python3 scripts/merge_module.py MODULE_NAME

# Apply changes
python3 scripts/merge_module.py MODULE_NAME --apply
```

**Features:**
- Collects types, constants, functions from all files
- Detects existing symbols in types.rs to avoid duplicates
- Handles `super::module_name::` patterns in nested modules
- Skips symbols already defined (e.g., libc re-exports)

**Best for:** Modules with varying content across files.

---

## remove_module.py

Removes a module from all files when types already exist in `types.rs`.

```bash
# Preview changes
python3 scripts/remove_module.py MODULE_NAME

# Apply changes
python3 scripts/remove_module.py MODULE_NAME --apply
```

**Best for:** Modules where you've already added types to types.rs manually.

---

## consolidate_sys_types.py

Removes `sys__types_h` modules and updates imports.

```bash
python3 scripts/consolidate_sys_types.py --dry-run  # Preview
python3 scripts/consolidate_sys_types.py            # Apply
```

---

## consolidate_wrapper_modules.py

Removes simple type wrapper modules (`_gid_t_h`, `_socklen_t_h`, etc.).

```bash
python3 scripts/consolidate_wrapper_modules.py --dry-run  # Preview
python3 scripts/consolidate_wrapper_modules.py            # Apply
```

---

## consolidate_module.py (Original)

Older consolidation script with more options.

```bash
# Analyze module content
python3 scripts/consolidate_module.py bouncer_h --analyze

# Extract definitions for types.rs
python3 scripts/consolidate_module.py bouncer_h --extract

# Preview changes
python3 scripts/consolidate_module.py bouncer_h --dry-run

# Apply changes
python3 scripts/consolidate_module.py bouncer_h --force
```

**Known limitations:**
- Tries to import extern statics from types.rs (they should stay in files)
- Doesn't distinguish types from extern declarations
- Fails for complex modules like `bouncer_h`, `iobuf_h`

---

## cleanup_functions.py

Applies function cleanup patterns.

```bash
python3 scripts/cleanup_functions.py src/admin.rs --dry-run
```

**Transformations:**
- `.wrapping_add(x)` → `+= x`
- `b"str\0" as *const...` → `c"str".as_ptr()`
- Remove redundant `mut` on pointer parameters
- `0 as usec_t` → `0`

---

## convert_static_mut.py

Analysis tool for static mut conversion planning.

```bash
python3 scripts/convert_static_mut.py src/main.rs
```

**Note:** Actual conversion is blocked until modules are fully migrated.

---

## Script Improvement Ideas

### For consolidate_module.py

1. Add `--types-only` flag to only consolidate type definitions
2. Add `--skip-statics` flag to not import `static mut` declarations
3. Distinguish symbol types:
   - Type definitions → import from types.rs
   - Extern statics → keep in file
   - Inline functions → check dependencies first

### Proposed New Scripts

**analyze_module_deps.py** — Categorize modules by complexity:
```bash
python3 scripts/analyze_module_deps.py bouncer_h
# Output: Lists extern dependencies, flags as "simple" or "complex"
```
