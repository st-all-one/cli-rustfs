#!/usr/bin/env bash
# Production source files stay reviewable. Tests and generated fixtures are exempt.
set -euo pipefail
limit="${FILE_LENGTH_LIMIT:-15000}"
status=0
while IFS= read -r -d '' file; do
    lines=$(wc -l <"$file")
    if (( lines > limit )); then
        printf '%s: %s lines (limit %s)\n' "$file" "$lines" "$limit" >&2
        status=1
    fi
done < <(find crates -type f -path '*/src/*.rs' -print0)
exit "$status"
