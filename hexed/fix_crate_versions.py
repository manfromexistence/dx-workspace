#!/usr/bin/env python3
"""
Fix crate versions in Cargo.toml to match TOML.md (original Codex versions)
"""

import re
import sys
from pathlib import Path

def parse_toml_versions(content):
    """Parse crate versions from TOML content"""
    versions = {}
    
    # Match patterns like: crate = "version" or crate = { version = "version", ... }
    patterns = [
        r'^(\S+)\s*=\s*"([^"]+)"',  # simple: crate = "version"
        r'^(\S+)\s*=\s*\{\s*version\s*=\s*"([^"]+)"',  # complex: crate = { version = "version"
    ]
    
    for line in content.split('\n'):
        line = line.strip()
        for pattern in patterns:
            match = re.match(pattern, line)
            if match:
                crate_name = match.group(1)
                version = match.group(2)
                versions[crate_name] = version
                break
    
    return versions

def fix_versions(content, original_versions):
    """Fix versions in Cargo.toml content"""
    lines = content.split('\n')
    fixed_lines = []
    changes = []
    
    for line in lines:
        fixed_line = line
        
        # Try to match crate = "version"
        match = re.match(r'^(\s*)(\S+)\s*=\s*"([^"]+)"(.*)$', line)
        if match:
            indent, crate_name, current_ver, rest = match.groups()
            if crate_name in original_versions:
                original_ver = original_versions[crate_name]
                if current_ver != original_ver:
                    fixed_line = f'{indent}{crate_name} = "{original_ver}"{rest}'
                    changes.append((crate_name, current_ver, original_ver))
        
        # Try to match crate = { version = "version", ... }
        match = re.match(r'^(\s*)(\S+)\s*=\s*\{\s*version\s*=\s*"([^"]+)"(.*)$', line)
        if match:
            indent, crate_name, current_ver, rest = match.groups()
            if crate_name in original_versions:
                original_ver = original_versions[crate_name]
                if current_ver != original_ver:
                    fixed_line = f'{indent}{crate_name} = {{ version = "{original_ver}"{rest}'
                    changes.append((crate_name, current_ver, original_ver))
        
        fixed_lines.append(fixed_line)
    
    return '\n'.join(fixed_lines), changes

def main():
    # Read TOML.md (original Codex versions)
    toml_md_path = Path("TOML.md")
    if not toml_md_path.exists():
        print("ERROR: TOML.md not found!")
        sys.exit(1)
    
    with open(toml_md_path, 'r', encoding='utf-8') as f:
        original_content = f.read()
    
    # Read current Cargo.toml
    cargo_toml_path = Path("codex-rs/Cargo.toml")
    if not cargo_toml_path.exists():
        print("ERROR: codex-rs/Cargo.toml not found!")
        sys.exit(1)
    
    with open(cargo_toml_path, 'r', encoding='utf-8') as f:
        current_content = f.read()
    
    # Parse original versions
    original_versions = parse_toml_versions(original_content)
    
    # Fix versions
    fixed_content, changes = fix_versions(current_content, original_versions)
    
    if not changes:
        print("✓ No changes needed - all versions already match!")
        return 0
    
    # Write fixed content
    with open(cargo_toml_path, 'w', encoding='utf-8') as f:
        f.write(fixed_content)
    
    # Report changes
    print(f"✓ Fixed {len(changes)} crate versions:\n")
    print(f"{'Crate':<30} {'Old':<15} {'New':<15}")
    print("-" * 60)
    
    for crate, old_ver, new_ver in sorted(changes):
        print(f"{crate:<30} {old_ver:<15} {new_ver:<15}")
    
    print(f"\n✓ Updated codex-rs/Cargo.toml with {len(changes)} version fixes")
    return 0

if __name__ == "__main__":
    sys.exit(main())
