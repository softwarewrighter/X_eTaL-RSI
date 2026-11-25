# Changes

Every commit, newest first, grouped by day. Times are Pacific
(UTC-08:00), as committed.

Categories: `feat` new capability, `fix` a bug or wrong behavior,
`refactor` structure without behavior change, `test` tests only,
`build` build and tooling, `exp` experiments, `docs` documentation,
`design` a decision recorded, `plan` saga planning and reordering,
`chore` agentrail bookkeeping (step complete, saga archive).

## 2026-11-24

- 18:06 `build` Layout (step 3): libs/<Name>/ as in X_eTaL-libraries (templates/Library, libs.py, test-libs.sh, new-lib.sh, run-lib.sh, doc-test.sh) and experiments/NN-slug/ (templates/Experiment, experiments.sh, test-experiments.sh, run-experiment.sh, new-experiment.sh), reg-rs baselines, FAIL lines fail; just test, new-lib, new-exp, run-exp, show-exp, test-lib, test-exp, bless-lib, bless-exp, doc-test; the gate runs them all; xetal.sh keeps cargo's warnings for a failed build.
- 17:00 `chore` Step 2 (xetal-pin) closed.
- 16:54 `build` The pinned X_eTaL (step 2): XETAL_COMMIT c925f58 and XETAL_LIBRARIES_COMMIT c7e812e, cloned into work/ and built by scripts/xetal.sh (just xetal, xetal-pin, libs-pin, xetal-version); Check, Parsing, Lists, Strings, Format and Statistics from X_eTaL-libraries; scripts/xt (just eval) with every library on XETAL_PATH; check-xetal and check-asks (asks R1.. against X_eTaL's ledger) in the gate.
- 16:49 `plan` The manipulator arms dropped from the Workshop (the user: a request meant for another project); the Workshop is again the research's resource and manufacturing world.
- 16:22 `chore` Step 1 (scaffold) closed.
- 16:47 `build` Scaffold (step 1): the agentrail saga foundation (8 steps), CLAUDE.md with the repo rules and AGENTS.md a symlink to it, the README (the idea, experiments, libraries, copyright, license), docs/plan.md (the Exo paper, decisions A1-A8, the Workshop's manipulator arms, libraries, experiments, sagas), docs/xetal-asks.md, .gitignore, justfile and the gate (spelling, markdown).
- 15:27 `chore` The first commit: an empty README.
