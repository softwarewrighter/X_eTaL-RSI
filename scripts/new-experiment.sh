#!/usr/bin/env bash
# Start an experiment from templates/Experiment: experiments/NN-slug/
# with its README, program and reg-rs baseline.
#   scripts/new-experiment.sh NN slug "Title" "what it shows"
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
usage='usage: new-experiment.sh NN slug "Title" "what it shows"'
num="${1:?$usage}"; slug="${2:?$usage}"; title="${3:?$usage}"; summary="${4:?$usage}"
[[ "$num" =~ ^[0-9][0-9]$ ]] || { echo "new-experiment: NN is two digits" >&2; exit 1; }
[[ "$slug" =~ ^[a-z0-9-]+$ ]] || { echo "new-experiment: a slug is lowercase, digits and -" >&2; exit 1; }
full="$num-$slug"; d="$root/experiments/$full"
[ ! -e "$d" ] || { echo "new-experiment: experiments/$full exists" >&2; exit 1; }
mkdir -p "$root/experiments"
cp -R "$root/templates/Experiment" "$d"
mv "$d/__SLUG__.xtl" "$d/$full.xtl"; rm -f "$d/reg/.keep"
esc() { printf '%s' "$1" | sed 's/[&|\\]/\\&/g'; }
find "$d" -type f | while read -r f; do
  sed -i.bak -e "s|__SLUG__|$full|g" -e "s|__NUM__|$num|g" -e "s|__TITLE__|$(esc "$title")|g" -e "s|__SUMMARY__|$(esc "$summary")|g" "$f" && rm -f "$f.bak"
done
chmod +x "$d"/*.xtl
XETAL_BLESS=1 "$root/scripts/test-experiments.sh" "$full" >/dev/null
echo "new experiment: experiments/$full/ (just run-exp $num; just test-exp $num)"
