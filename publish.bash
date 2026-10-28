#!/usr/bin/env bash
set -euo pipefail

# Ordered crate list
CRATES=(
  "pawkit-crockford:crockford"
  "pawkit-interner:interner"
  "pawkit-logger:logger"
  "pawkit-input:input"
  "pawkit-fs:fs"
  "pawkit:."
)

# Crates to exclude from publishing only
EXCLUDE_FROM_PUBLISH=

is_excluded_from_publish() {
    local crate_name="$1"
    for ex in "${EXCLUDE_FROM_PUBLISH[@]}"; do
        if [[ "$ex" == "$crate_name" ]]; then
            return 0
        fi
    done
    return 1
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

echo "=== Publishing all crates ==="
for entry in "${CRATES[@]}"; do
    IFS=":" read -r name path <<< "$entry"
    [[ -f "$path/Cargo.toml" ]] || { echo "Skipping $name (no Cargo.toml)"; continue; }
    publish_crate "$name" "$path"
done
echo "Release complete."
