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

# Get and build xetal at the known-good commit in XETAL_COMMIT (clone in work/xetal, binary bin/xetal), and the libraries used from X_eTaL-libraries
xetal:
    @scripts/xetal.sh

# Pin a newer X_eTaL: a committed ref of ../X_eTaL (default HEAD) into XETAL_COMMIT, built; then test and commit
xetal-pin ref="HEAD":
    scripts/xetal-pin.sh "$1"

# Pin a newer X_eTaL-libraries: a committed ref of ../X_eTaL-libraries into XETAL_LIBRARIES_COMMIT
libs-pin ref="HEAD":
    scripts/xetal-libraries.sh --pin "$1"

# The pinned X_eTaL: the commits in XETAL_COMMIT and XETAL_LIBRARIES_COMMIT, and the binary's own report
xetal-version:
    @echo "XETAL_COMMIT $(cat XETAL_COMMIT)"
    @echo "XETAL_LIBRARIES_COMMIT $(cat XETAL_LIBRARIES_COMMIT)"
    @"$(scripts/xetal.sh)" --version

# Evaluate an expression with the pinned xetal, every library on XETAL_PATH: just eval "'+ r_/ 1 2 3"
eval expr:
    @scripts/xt eval -e "$1"

# Check the pinned X_eTaL: the CLI builds, answers, reports its commit; Check loads
check-xetal:
    scripts/check-xetal.sh

# Our asks in step with X_eTaL's ledger at the pin
check-asks:
    scripts/py scripts/check-asks.py --self-test
    scripts/py scripts/check-asks.py
