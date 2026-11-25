# X_eTaL-RSI tasks. Recipes call scripts/*.sh, which hold the logic and
# work without just too. `just` alone lists the recipes.

set positional-arguments

# List the recipes
default:
    @just --list

# The pre-commit gate
gate:
    scripts/gate.sh

# American spellings only (the checker checks itself first)
check-spelling:
    scripts/py scripts/check-spelling.py --self-test
    scripts/py scripts/check-spelling.py
