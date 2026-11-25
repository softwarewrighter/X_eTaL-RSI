#!/usr/bin/env bash
# The experiments, experiments/NN-slug/ (README.md, *.xtl, reg/): list
# them, find one by its number or slug, or check the layout.
#   scripts/experiments.sh list          # NN-slug, one per line
#   scripts/experiments.sh find 05       # experiments/05-specialization-cost
#   scripts/experiments.sh check         # each is complete
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
list() { for d in "$root"/experiments/[0-9][0-9]-*/; do [ -d "$d" ] && basename "$d"; done; return 0; }
case "${1:-list}" in
  list) list ;;
  find)
    want="${2:?usage: experiments.sh find NN|slug}"
    hit="$(list | grep -E "^($want-|[0-9][0-9]-$want$|$want$)" || true)"
    [ "$(printf '%s' "$hit" | grep -c . || true)" = 1 ] || { echo "experiments: no single experiment '$want'" >&2; exit 1; }
    echo "experiments/$hit" ;;
  check)
    fail=0
    for e in $(list); do
      d="$root/experiments/$e"
      [[ "$e" =~ ^[0-9][0-9]-[a-z0-9-]+$ ]] || { echo "experiments: $e: a name is NN-slug" >&2; fail=1; }
      [ -f "$d/README.md" ] || { echo "experiments: $e/README.md missing" >&2; fail=1; }
      [ -f "$d/$e.xtl" ] || { echo "experiments: $e/$e.xtl missing" >&2; fail=1; }
      grep -q -i 'simulation' "$d/README.md" 2>/dev/null || { echo "experiments: $e/README.md must say whether it is a simulation" >&2; fail=1; }
    done
    exit $fail ;;
  *) echo "usage: experiments.sh list|find|check" >&2; exit 2 ;;
esac
