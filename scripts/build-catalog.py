# Run with scripts/py (the uv venv .venv), never a system python.
"""Write pages/index.html, the site's catalog: every experiment with a
web app (a live page) and every other experiment (run at the command
line), each with its title and summary from its README, and the commit
the site was built from (data-sha, and "sha XXXXXXX" in the footer,
which scripts/check-live.sh reads).

  scripts/py scripts/build-catalog.py
"""
import html
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = "https://github.com/softwarewrighter/X_eTaL-RSI"


def experiments():
    for d in sorted((ROOT / "experiments").glob("[0-9][0-9]-*/")):
        readme = (d / "README.md").read_text().splitlines()
        title = readme[0].lstrip("# ").strip() if readme else d.name
        summary = next((l.strip() for l in readme[1:] if l.strip()), "")
        yield d.name, title, summary, (d / "web" / "Cargo.toml").exists()


def main():
    sha = subprocess.run(["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, capture_output=True, text=True).stdout.strip()
    xetal = (ROOT / "XETAL_COMMIT").read_text().strip()[:7]
    cards = []
    for slug, title, summary, live in experiments():
        link = f'<a href="{slug}/">{html.escape(title)}</a>' if live else f'<a href="{REPO}/tree/main/experiments/{slug}">{html.escape(title)}</a>'
        kind = "live in your browser" if live else "at the command line"
        cards.append(f'<li>{link} <span class="kind">({kind})</span><br><span class="sum">{html.escape(summary)}</span></li>')
    page = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>X_eTaL-RSI</title>
<link rel="icon" href="favicon.ico">
<style>
body {{ font-family: system-ui, sans-serif; max-width: 52em; margin: 2em auto; padding: 0 1em; line-height: 1.45; }}
.brand {{ display: flex; align-items: center; gap: 0.8em; }}
.brand img {{ height: 56px; }}
li {{ margin: 0.8em 0; }} .kind {{ opacity: 0.7; }} .sum {{ opacity: 0.85; }}
footer {{ margin-top: 3em; font-size: 0.85em; opacity: 0.75; }}
@media (prefers-color-scheme: dark) {{ body {{ background: #161616; color: #ddd; }} a {{ color: #7ab0ff; }} }}
</style>
</head>
<body data-sha="{sha}">
<div class="brand"><img src="xetal-logo-orange.png" alt="X_eTaL"><h1>X_eTaL-RSI</h1></div>
<p>Recursive self-improvement in X_eTaL, the eXperimental Extensible Typed
Array Language: small, deterministic experiments in which a system improves
the harness it runs in, and then the procedure by which it improves it,
without changing a model's weights. Each experiment is a simulation with a
control; the live pages run X_eTaL in your browser (WebAssembly).</p>
<h2>Experiments</h2>
<ul>
{chr(10).join(cards)}
</ul>
<footer>Copyright (c) 2026 Michael A Wright &middot; MIT License &middot;
<a href="{REPO}">Repository</a> &middot; X_eTaL {xetal} &middot; sha {sha}</footer>
</body>
</html>
"""
    (ROOT / "pages" / "index.html").write_text(page)
    print(f"build-catalog: pages/index.html ({len(cards)} experiments, sha {sha})")


if __name__ == "__main__":
    main()
