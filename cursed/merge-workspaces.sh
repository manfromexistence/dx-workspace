#!/bin/bash
# Merge dx workspace into codex-rs main workspace
# Run from codex-rs/ directory

set -e  # Exit on error

echo "🔧 Merging dx workspace into codex-rs main workspace..."

# Check we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "dx" ]; then
    echo "❌ Error: Run this script from codex-rs/ directory"
    exit 1
fi

# Backup
echo "📦 Creating backups..."
cp Cargo.toml Cargo.toml.backup
cp dx/Cargo.toml dx/Cargo.toml.backup

# Step 1: Add dx members to main workspace
echo "📝 Updating main workspace Cargo.toml..."
cat >> Cargo.toml << 'EOF'

# DX TUI workspace members
    "dx",
    "dx/src/file_browser/actor",
    "dx/src/file_browser/adapter",
    "dx/src/file_browser/binding",
    "dx/src/file_browser/boot",
    "dx/src/file_browser/build",
    "dx/src/file_browser/cli",
    "dx/src/file_browser/codegen",
    "dx/src/file_browser/config",
    "dx/src/file_browser/core",
    "dx/src/file_browser/dds",
    "dx/src/file_browser/emulator",
    "dx/src/file_browser/ffi",
    "dx/src/file_browser/fs",
    "dx/src/file_browser/macro",
    "dx/src/file_browser/packing",
    "dx/src/file_browser/parser",
    "dx/src/file_browser/plugin",
    "dx/src/file_browser/proxy",
    "dx/src/file_browser/scheduler",
    "dx/src/file_browser/sftp",
    "dx/src/file_browser/shared",
    "dx/src/file_browser/shim",
    "dx/src/file_browser/term",
    "dx/src/file_browser/tty",
    "dx/src/file_browser/vfs",
    "dx/src/file_browser/watcher",
    "dx/src/file_browser/widgets",
EOF

echo "✅ Main workspace updated"

# Step 2: Test the configuration
echo "🧪 Testing workspace configuration..."
cargo metadata --format-version 1 > /dev/null 2>&1 || {
    echo "❌ Workspace configuration invalid!"
    echo "Restoring backups..."
    cp Cargo.toml.backup Cargo.toml
    exit 1
}

echo "✅ Workspace configuration valid"

# Step 3: Check build
echo "🔨 Testing build..."
cargo check --workspace -j3 || {
    echo "⚠️  Build check failed, but workspace structure is valid"
    echo "You may need to update dependencies manually"
}

echo ""
echo "✅ Workspace merge complete!"
echo ""
echo "Next steps:"
echo "1. Edit dx/Cargo.toml to remove [workspace] sections"
echo "2. Update dx dependencies to use 'workspace = true'"
echo "3. Run: cargo check --workspace"
echo ""
echo "Backups saved as:"
echo "  - Cargo.toml.backup"
echo "  - dx/Cargo.toml.backup"
