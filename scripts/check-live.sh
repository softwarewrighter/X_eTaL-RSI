#!/usr/bin/env bash
# Verify the deployed site after just publish: wait until the catalog
# at https://softwarewrighter.github.io/X_eTaL-RSI/ reports this
# checkout's commit (GitHub Pages takes a minute), then load every
# experiment's live page in headless Chrome and check, as the local
# browser test does, that X_eTaL ran there.
#   scripts/check-live.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
base="https://softwarewrighter.github.io/X_eTaL-RSI"
head="$(git rev-parse --short HEAD)"
live=""
for _ in $(seq 1 40); do
  live="$(curl -fsS "$base/?t=$(date +%s)" 2>/dev/null | grep -o 'sha [0-9a-f]\{7,\}' | head -1 | cut -d' ' -f2 || true)"
  [ "$live" = "$head" ] && break
  sleep 6
done
[ "$live" = "$head" ] || { echo "check-live: the site reports commit '${live:-none}', not $head (just publish, or wait)" >&2; exit 1; }
echo "ok: catalog live at commit $head"
fail=0
while IFS= read -r e; do
  [ -f "experiments/$e/web/browser.txt" ] || continue
  if out="$(XETAL_LIVE=1 scripts/browser-check.sh "$e" 2>&1)"; then
    echo "ok: $base/$e/ ($(printf '%s\n' "$out" | grep -c '^found:') checks)"
  else
    echo "FAIL: $base/$e/"; printf '%s\n' "$out"; fail=1
  fi
done < <(scripts/experiments.sh list)
[ $fail = 0 ] && echo "check-live: ok" || { echo "check-live: FAILURES"; exit 1; }
