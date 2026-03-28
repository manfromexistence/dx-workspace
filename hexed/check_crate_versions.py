#!/usr/bin/env python3
"""
Check crate version differences between TOML.md (original Codex) and current Cargo.toml
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
    
    # Parse versions
    original_versions = parse_toml_versions(original_content)
    current_versions = parse_toml_versions(current_content)
    
    # Find differences
    differences = []
    for crate, original_ver in original_versions.items():
        if crate in current_versions:
            current_ver = current_versions[crate]
            if original_ver != current_ver:
                differences.append((crate, original_ver, current_ver))
    
    # Report results
    if not differences:
        print("✓ All crate versions match!")
        return 0
    
    print(f"Found {len(differences)} version differences:\n")
    print(f"{'Crate':<30} {'Original':<15} {'Current':<15}")
    print("-" * 60)
    
    for crate, original, current in sorted(differences):
        print(f"{crate:<30} {original:<15} {current:<15}")
    
    print(f"\n{len(differences)} crates need version updates")
    return 1

if __name__ == "__main__":
    sys.exit(main())
