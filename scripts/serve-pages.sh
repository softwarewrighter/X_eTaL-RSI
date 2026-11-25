#!/usr/bin/env bash
# Serve the built pages/ locally as GitHub Pages will, under
# /X_eTaL-RSI/: http://127.0.0.1:PORT/X_eTaL-RSI/ (default 8435).
#   scripts/serve-pages.sh [PORT]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
port="${1:-8435}"
site="$root/target/serve-pages"
mkdir -p "$site"
ln -sfn "$root/pages" "$site/X_eTaL-RSI"
echo "serving http://127.0.0.1:$port/X_eTaL-RSI/ (Ctrl-C stops)"
exec "$root/scripts/py" -m http.server "$port" --bind 127.0.0.1 --directory "$site"
