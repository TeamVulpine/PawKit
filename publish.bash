#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./publish.bash -u | --update     # update versions
#   ./publish.bash -r | --release    # cargo publish

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 [-u|--update] [-r|--release]"
    exit 1
fi

MODE="$1"
VERSION="0.1.4"
WORKSPACE_TOML="Cargo.toml"

# Ordered crate list
CRATES=(
  "pawkit-holy-array:holy-array"
  "pawkit-crockford:crockford"
  "pawkit-bitarray:bitarray"
  "pawkit-logger:logger"
  "pawkit-input:input"
  "pawkit-fs:fs"
  "pawkit-net-http:net/http"
  "pawkit-net-websocket:net/websocket"
  "pawkit-net-signaling:net/signaling"
  "pawkit-net:net/runtime"
  "pawkit-bindings-c:bindings/c"
  "pawkit-bindings-lua:bindings/lua"
  "pawkit-bindings-godot:bindings/godot"
  "pawkit:."
)

# Crates to exclude from publishing only
EXCLUDE_FROM_PUBLISH=(
  "pawkit-bindings-godot"
)

is_excluded_from_publish() {
    local crate_name="$1"
    for ex in "${EXCLUDE_FROM_PUBLISH[@]}"; do
        if [[ "$ex" == "$crate_name" ]]; then
            return 0
        fi
    done
    return 1
}

update_crate_version() {
    local crate_name="$1"
    local crate_path="$2"
    echo "Updating $crate_name ($crate_path) → $VERSION"

    sed -i.bak -E "s/^version\s*=\s*\"[^\"]+\"/version = \"$VERSION\"/" "$crate_path/Cargo.toml"
    rm -f "$crate_path/Cargo.toml.bak"

    if grep -qE "^${crate_name}\.version\s*=" "$WORKSPACE_TOML"; then
        sed -i.bak -E "s|^(${crate_name}\.version\s*=\s*\")[^\"]+(\")|\1${VERSION}\2|" "$WORKSPACE_TOML"
        rm -f "$WORKSPACE_TOML.bak"
    fi
}

publish_crate() {
    local crate_name="$1"
    local crate_path="$2"

    if is_excluded_from_publish "$crate_name"; then
        echo "Skipping publish for $crate_name (excluded)"
        return
    fi

    echo "Publishing $crate_name..."
    (cd "$crate_path" && cargo publish --no-verify)
}

run_update() {
    echo "=== Updating all crate versions to $VERSION ==="
    for entry in "${CRATES[@]}"; do
        IFS=":" read -r name path <<< "$entry"
        [[ -f "$path/Cargo.toml" ]] || { echo "Skipping $name (no Cargo.toml)"; continue; }
        update_crate_version "$name" "$path"
    done
    echo "Update complete."
}

run_release() {
    echo "=== Updating and publishing all crates ==="
    for entry in "${CRATES[@]}"; do
        IFS=":" read -r name path <<< "$entry"
        [[ -f "$path/Cargo.toml" ]] || { echo "Skipping $name (no Cargo.toml)"; continue; }
        publish_crate "$name" "$path"
    done
    echo "Release complete."
}

case "$MODE" in
  -u|--update)
    run_update
    ;;
  -r|--release)
    run_release
    ;;
  *)
    echo "Invalid argument: $MODE"
    echo "Usage: $0 [-u|--update] [-r|--release]"
    exit 1
    ;;
esac
