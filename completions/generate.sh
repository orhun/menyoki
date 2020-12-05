#!/usr/bin/env bash
set -e

metadata=$(cargo metadata --no-deps --format-version 1)
bin=$(jq -r '.packages[0].name' <<< "$metadata")
target=$(jq -r '.target_directory' <<< "$metadata")
workspace=$(jq -r '.workspace_root' <<< "$metadata")
echo "==> Building the project..."
cargo build
echo "==> Generating shell completions..."
for sh in "bash" "fish" "zsh" "powershell" "elvish"; do
    "$target/debug/$bin" misc -g $sh > "$workspace/completions/$bin.$sh"
done
echo "==> Done."
