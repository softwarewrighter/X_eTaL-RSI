#!/usr/bin/env bash
# Test one experiment's built page in a real browser: headless Chrome
# loads pages/NN-slug/ (scripts/browser-dom.sh), whose WebAssembly runs
# the page's X_eTaL program with the pinned X_eTaL; then the rendered
# page must show no X_eTaL error and every line of the experiment's
# web/browser.txt (text the page shows only once X_eTaL has run).
# Prints one line per check, the same each run.
#   scripts/browser-check.sh NN|slug
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
e="$(basename "$("$root/scripts/experiments.sh" find "${1:?usage: browser-check.sh NN|slug}")")"
want="$root/experiments/$e/web/browser.txt"
[ -f "$want" ] || { echo "browser-check: no $want" >&2; exit 1; }
"$root/scripts/browser-dom.sh" "$e" "${BROWSER_MS:-20000}" | "$root/scripts/py" -c '
import html, re, sys
dom = sys.stdin.read()
want = [l.rstrip("\n") for l in open(sys.argv[1]) if l.strip() and not l.startswith("#")]
main = re.search(r"<main>(.*)</main>", dom, re.S)
text = html.unescape(re.sub(r"<[^>]+>", "", main.group(1))) if main else ""
ok = "</html>" in dom
print("page: loaded" if ok else "page: NOT LOADED")
errors = [m for m in ("X_eTaL stopped", "class=\"error\"", "panicked") if m in dom]
print("errors: " + (", ".join(errors) if errors else "none"))
ok &= not errors
for w in want:
    found = w in text
    print(("found: " if found else "MISSING: ") + w)
    ok &= found
sys.exit(0 if ok else 1)
' "$want"
