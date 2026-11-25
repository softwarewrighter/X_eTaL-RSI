#!/usr/bin/env bash
# Build the live site into pages/, which is not tracked: just publish
# (scripts/publish-pages.sh) makes it the gh-pages branch's only
# commit, which GitHub Pages serves (nothing is built on GitHub).
#   - every experiment with a web app (experiments/NN-slug/web/) is
#     built with trunk into pages/NN-slug/, served under
#     /X_eTaL-RSI/NN-slug/;
#   - pages/index.html, the catalog (scripts/build-catalog.py).
# With arguments (experiments, by number or slug) only those are built.
#   scripts/build-pages.sh [NN|slug ...]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
base="/X_eTaL-RSI"
"$root/scripts/xetal.sh" >/dev/null   # the web apps build on work/xetal
mkdir -p "$root/pages"
touch "$root/pages/.nojekyll"
want=()
if [ $# -gt 0 ]; then for a in "$@"; do want+=("$(basename "$("$root/scripts/experiments.sh" find "$a")")"); done
else while IFS= read -r e; do [ -n "$e" ] && want+=("$e"); done < <("$root/scripts/experiments.sh" list); fi
for e in ${want[@]+"${want[@]}"}; do
  web="$root/experiments/$e/web"
  [ -f "$web/Cargo.toml" ] || continue
  dist="$root/target/pages-dist/$e"
  echo "==> trunk build $e"
  (cd "$web" && trunk build --release --public-url "$base/$e/" --dist "$dist" 2>&1 | grep -E 'error|ERROR' || true)
  [ -f "$dist/index.html" ] || { echo "build-pages: trunk built nothing for $e" >&2; exit 1; }
  rsync -a --delete "$dist/" "$root/pages/$e/"
done
# A full build drops pages/NN-slug/ of experiments without a web app.
if [ $# -eq 0 ]; then
  for d in "$root"/pages/*/; do
    [ -d "$d" ] || continue
    [ -f "$root/experiments/$(basename "$d")/web/Cargo.toml" ] || { echo "removing pages/$(basename "$d")/"; rm -rf "$d"; }
  done
fi
cp "$root/images/xetal-logo-orange.png" "$root/images/favicon.ico" "$root/pages/"
"$root/scripts/py" "$root/scripts/build-catalog.py"
echo "pages/ built; just publish puts it live (after committing your changes)."
