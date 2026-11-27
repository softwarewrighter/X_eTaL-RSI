#!/usr/bin/env bash
# The pre-commit gate: the pinned X_eTaL, the asks against its ledger,
# every library (baselines, doc examples) and experiment (baselines),
# the pages' shell and web apps (native tests, wasm32; just pages and
# just browser-check put them in a real browser, as just publish does),
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
step "the libraries"
scripts/test-libs.sh
scripts/doc-test.sh
step "the experiments"
scripts/test-experiments.sh
step "the pages: the shared shell and each web app (native tests, wasm32)"
(cd shared/microscope && cargo test -q >/dev/null 2>&1 && cargo check -q --target wasm32-unknown-unknown) || { echo "FAIL: shared/microscope"; exit 1; }
for w in experiments/*/web; do
  [ -f "$w/Cargo.toml" ] || continue
  (cd "$w" && cargo test -q >/dev/null 2>&1 && cargo check -q --target wasm32-unknown-unknown) || { (cd "$w" && cargo test -q); echo "FAIL: $w"; exit 1; }
  echo "ok: $w"
done
step "spelling"
scripts/py scripts/check-spelling.py --self-test
scripts/py scripts/check-spelling.py
step "markdown"
md=(README.md CHANGES.md docs/plan.md docs/xetal-asks.md docs/related.md docs/report.md shared/microscope/README.md)
for f in libs/*/README.md libs/*/docs/README.md experiments/*/README.md; do [ -e "$f" ] && md+=("$f"); done
for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; exit 1; }; done
echo "gate: ok"
