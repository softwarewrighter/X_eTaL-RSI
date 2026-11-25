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

# Every library and experiment checked against its baselines
test:
    scripts/test-libs.sh
    scripts/test-experiments.sh

# The libraries with their aliases and summaries
libs:
    @scripts/py scripts/libs.py table

# The libraries' src/ directories, for XETAL_PATH
path:
    @scripts/py scripts/libs.py path

# Start a library from templates/Library: just new-lib Econ ec: "the economics of specialization"
new-lib name alias summary:
    scripts/new-lib.sh "$1" "$2" "$3"

# Run a library's test programs (or one): just run-lib Econ basics
run-lib name prog="":
    @scripts/run-lib.sh "$1" ${2:+"$2"}

# Run a library's demos (or one): just demo-lib Econ
demo-lib name prog="":
    @scripts/run-lib.sh --demos "$1" ${2:+"$2"}

# Test one library's baselines
test-lib name:
    scripts/test-libs.sh "$1"

# Accept one library's current output as its baselines (review the diff!)
bless-lib name:
    XETAL_BLESS=1 scripts/test-libs.sh "$1"

# Run every library's ## >> doc examples (or the named ones)
doc-test *names:
    scripts/doc-test.sh "$@"

# The experiments, in order
experiments:
    @scripts/experiments.sh list

# Start an experiment from templates/Experiment: just new-exp 05 specialization-cost "Specialization cost" "when improvement pays"
new-exp num slug title summary:
    scripts/new-experiment.sh "$1" "$2" "$3" "$4"

# Run an experiment's programs (or one): just run-exp 05
run-exp exp prog="":
    @scripts/run-experiment.sh "$1" ${2:+"$2"}

# An experiment as a notebook: each statement, then its output
show-exp exp prog="":
    @scripts/run-experiment.sh --echo "$1" ${2:+"$2"}

# Test one experiment's baselines
test-exp exp:
    scripts/test-experiments.sh "$1"

# Accept one experiment's current output as its baselines (review the diff!)
bless-exp exp:
    XETAL_BLESS=1 scripts/test-experiments.sh "$1"
