#!/usr/bin/env bash
# Run an experiment's programs (all, or the one named) as its tests do:
# from its directory, every library on XETAL_PATH, --seed 1. --echo
# shows each statement before its output.
#   scripts/run-experiment.sh [--echo] NN|slug [program]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo=()
[ "${1:-}" = --echo ] && { echo=(--echo); shift; }
d="$root/$("$root/scripts/experiments.sh" find "${1:?usage: run-experiment.sh [--echo] NN|slug [program]}")"
if [ -n "${2:-}" ]; then progs=("${2%.xtl}.xtl"); else progs=(); for p in "$d"/*.xtl; do progs+=("$(basename "$p")"); done; fi
for p in "${progs[@]}"; do
  [ ${#progs[@]} -gt 1 ] && echo "== $(basename "$d")/$p"
  (cd "$d" && "$root/scripts/xt" run ${echo[@]+"${echo[@]}"} "$p")
done
