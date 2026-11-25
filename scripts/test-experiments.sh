#!/usr/bin/env bash
# Test the experiments (all, or those named by number or slug) with
# reg-rs: each experiment's reg/ is its reg-rs data directory, and
# NAME.rgt runs NAME.xtl from there with scripts/xt (the pinned xetal,
# every library on XETAL_PATH, --seed 1 --ascii). A baseline missing,
# or left over from a program that is gone, fails; so does any FAIL
# line (a failed Check) in a baseline's output. XETAL_BLESS=1 creates
# missing baselines and rebases the rest (review the diff!).
#   scripts/test-experiments.sh [NN|slug ...]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
command -v reg-rs >/dev/null || { echo "test-experiments: reg-rs not found on PATH" >&2; exit 127; }
"$root/scripts/xetal.sh" >/dev/null
bless="${XETAL_BLESS:-}"
[ "$bless" = 1 ] || "$root/scripts/experiments.sh" check
dirs=()
if [ $# -gt 0 ]; then for a in "$@"; do dirs+=("$("$root/scripts/experiments.sh" find "$a")"); done
else while IFS= read -r e; do [ -n "$e" ] && dirs+=("experiments/$e"); done < <("$root/scripts/experiments.sh" list); fi
fail=0; n=0
for rel in ${dirs[@]+"${dirs[@]}"}; do
  d="$root/$rel"; e="$(basename "$d")"; n=$((n + 1))
  mkdir -p "$d/reg"
  export REG_RS_DATA_DIR="$d/reg"
  cd "$d/reg"
  xt="$("$root/scripts/py" -c 'import os,sys; print(os.path.relpath(sys.argv[1]))' "$root/scripts/xt")"
  wanted=()
  for p in "$d"/*.xtl; do [ -e "$p" ] && wanted+=("$(basename "$p" .xtl)"); done
  for t in ${wanted[@]+"${wanted[@]}"}; do
    cmd="$xt run ../$t.xtl"
    if [ ! -f "$t.rgt" ]; then
      if [ "$bless" = 1 ]; then rm -f "$t".tdb*; reg-rs create -t "$t" -c "$cmd" >/dev/null; echo "created: $e/$t"
      else echo "FAIL: $e/$t: no baseline reg/$t.rgt (just bless-exp $e)"; fail=1; continue; fi
    elif ! grep -qF "command = \"$cmd\"" "$t.rgt"; then
      echo "FAIL: $e/$t: $t.rgt does not run '$cmd'"; fail=1
    fi
  done
  for r in *.rgt; do
    [ -e "$r" ] || continue
    printf '%s\n' ${wanted[@]+"${wanted[@]}"} | grep -qx "${r%.rgt}" || { echo "FAIL: $e/${r%.rgt}: $r has no program (remove it)"; fail=1; }
  done
  if [ "$bless" = 1 ]; then
    reg-rs run -q -p .rgt >/dev/null 2>&1 || true
    reg-rs rebase -p .rgt >/dev/null 2>&1
    echo "blessed: $e"
  elif reg-rs run -q -p .rgt >/dev/null 2>&1; then
    echo "ok: $e ($(ls *.rgt | wc -l | tr -d ' ') baselines)"
  else
    echo "FAIL: $e"; reg-rs run -vv -p .rgt || true; fail=1
  fi
  for t in ${wanted[@]+"${wanted[@]}"}; do
    [ -f "$t.out" ] && grep -q '^FAIL' "$t.out" || continue
    echo "FAIL: $e/$t: a check failed:"; grep '^FAIL' "$t.out"; fail=1
  done
  cd "$root"
done
echo "test-experiments: $n experiment(s)$([ $fail = 0 ] && echo ', all passed' || echo ', FAILURES')"
exit $fail
