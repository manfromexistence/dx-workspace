#!/bin/bash
# Downgrade dx to use ratatui 0.29.0 from workspace
# Run from codex-rs/ directory

set -e

echo "🔧 Downgrading dx to ratatui 0.29.0 (workspace version)..."

# Check we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "dx" ]; then
    echo "❌ Error: Run this script from codex-rs/ directory"
    exit 1
fi

# Backup
echo "📦 Creating backup..."
cp dx/Cargo.toml dx/Cargo.toml.backup

# Step 1: Update main dx/Cargo.toml
echo "📝 Updating dx/Cargo.toml..."
sed -i.bak 's/ratatui = { version = "0\.30\.0", features = \["serde"\] }/ratatui = { workspace = true, features = ["serde"] }/' dx/Cargo.toml

# Remove backup file created by sed
rm -f dx/Cargo.toml.bak

# Step 2: Check all file_browser crates
echo "🔍 Checking file_browser crates..."
for dir in dx/src/file_browser/*/; do
    if [ -f "$dir/Cargo.toml" ]; then
        crate_name=$(basename "$dir")
        if grep -q "ratatui" "$dir/Cargo.toml"; then
            echo "  ✓ $crate_name uses ratatui (already workspace = true)"
        fi
    fi
done

# Step 3: Test the configuration
echo "🧪 Testing configuration..."
cargo metadata --format-version 1 > /dev/null 2>&1 || {
    echo "❌ Cargo metadata check failed!"
    echo "Restoring backup..."
    cp dx/Cargo.toml.backup dx/Cargo.toml
    exit 1
}

echo "✅ Configuration valid"

# Step 4: Check if dx compiles
echo "🔨 Checking if dx compiles with ratatui 0.29.0..."
cargo check -p dx-tui 2>&1 | tee /tmp/dx-check.log

if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! dx now uses ratatui 0.29.0"
    echo ""
    echo "Next steps:"
    echo "1. Test dx: cargo run --bin dx"
    echo "2. If it works, proceed with workspace merge"
    echo "3. See WORKSPACE_MERGE_GUIDE.md for merge instructions"
    echo ""
    echo "Backup saved as: dx/Cargo.toml.backup"
else
    echo ""
    echo "⚠️  Compilation errors detected!"
    echo ""
    echo "This means dx uses ratatui 0.30.0-specific APIs."
    echo "Check the errors in /tmp/dx-check.log"
    echo ""
    echo "Options:"
    echo "1. Fix the API incompatibilities (see RATATUI_VERSION_CONFLICT_SOLUTION.md)"
    echo "2. Restore backup: cp dx/Cargo.toml.backup dx/Cargo.toml"
    echo "3. Consider upgrading codex-rs to ratatui 0.30.0 instead (risky)"
    echo ""
    echo "Backup saved as: dx/Cargo.toml.backup"
fi
