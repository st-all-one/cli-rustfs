#!/usr/bin/env bash
# Print the current clippy diagnostic inventory. This is a paydown dashboard,
# not a gate. The gate is `make clippy-strict`.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo clippy --workspace --all-targets --message-format=json 2>/dev/null \
  | python3 -c '
import collections
import json
import sys

codes = collections.Counter()
files = collections.Counter()
total = 0

for line in sys.stdin:
    try:
        message = json.loads(line)
    except json.JSONDecodeError:
        continue
    if message.get("reason") != "compiler-message":
        continue
    body = message["message"]
    if body.get("level") not in ("warning", "error"):
        continue
    code = (body.get("code") or {}).get("code") or "rustc"
    codes[code] += 1
    total += 1
    for span in body.get("spans", []):
        if span.get("is_primary"):
            files[span.get("file_name", "?")] += 1
            break

print(f"total diagnostics: {total}")
print()
for code, count in codes.most_common(50):
    print(f"{count:6}  {code}")
print()
print("top files:")
for name, count in files.most_common(20):
    print(f"{count:6}  {name}")
'
