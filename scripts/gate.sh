#!/usr/bin/env bash
# The pre-commit gate: the pinned X_eTaL, the asks against its ledger,
# spelling (American), ASCII-only markdown.
#   scripts/gate.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
step() { printf '==> %s\n' "$*"; }
step "the pinned X_eTaL"
scripts/check-xetal.sh
step "the asks"
scripts/py scripts/check-asks.py --self-test
scripts/py scripts/check-asks.py
step "spelling"
scripts/py scripts/check-spelling.py --self-test
scripts/py scripts/check-spelling.py
step "markdown"
md=(README.md CHANGES.md docs/plan.md docs/xetal-asks.md)
for f in libs/*/README.md libs/*/docs/README.md experiments/*/README.md; do [ -e "$f" ] && md+=("$f"); done
for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; exit 1; }; done
echo "gate: ok"
