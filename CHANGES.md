# Changes

Every commit, newest first, grouped by day. Times are Pacific
(UTC-08:00), as committed.

Categories: `feat` new capability, `fix` a bug or wrong behavior,
`refactor` structure without behavior change, `test` tests only,
`build` build and tooling, `exp` experiments, `docs` documentation,
`design` a decision recorded, `plan` saga planning and reordering,
`chore` agentrail bookkeeping (step complete, saga archive).

## 2026-11-25

- 14:17 `plan` Saga 1 (foundation) archived; Saga 2 (recursion) started: six steps, Evolve and experiment 03, the recursive optimizer (04), the third page, transfer (06), context optimization (02), a report.

## 2026-11-24

- 22:11 `docs` The second page is live (check-live: the catalog at f2f6061, both pages run X_eTaL there); the plan's step list matches the saga's numbering; Saga 1 done.
- 22:02 `feat` The second live page (step 12, the user): experiment 01 in the browser, the longest tool, tools learned, a hint's cost and how often hints are ignored on sliders, the tools learned by name, groups A, B, C on both held-out sets with token bars, the background (Exo, XSkill, agentrail-rs, docs/related.md) and the program; four page tests against the command-line experiment; the browser check waits up to 20 s of virtual time.
- 23:50 `chore` Step 11 (guardian) closed.
- 20:36 `exp` Experiment 07, the guardian (step 11): eight candidates, each with one defect, under Exo's default (it builds), the same with a circuit breaker, the layered guardian and the layered one judging regressions on recent work; the layers stop each defect at its own layer, the default lets through 3 unsupervised deliveries and a failed task, the breaker rolls back only after it, a fixed held-out set rejects the adaptation to drifted work while a recent one accepts it; reviews cost 35k to 40k tokens a candidate; 7 claims checked.
- 20:36 `feat` The Guardian library (step 11): r_eview (formed, capable, context, correct, regression, shadow; stops at the first failure, reports its tokens; upto 1 is Exo's default), f_ormed, c_apable, the append-only history (b_egin, p_ropose, a_ctive, r_ollback); 25 doc examples, basics, the review demo.
- 23:20 `chore` Step 10 (tool-synthesis) closed.
- 20:19 `exp` Experiment 01, tool synthesis (step 10): groups A, B (hints in the prompt), C (tools) and C-short on the Workshop, tools learned from the fixed harness's journal; on held-out tasks of the training recipe tools cut calls 98 to 44 with the same ops, hints match the calls at 21% more tokens; on shifted recipes the long route never fits and its schema costs 12% (hints 36%), while tools of at most 3 ops still cut calls 24%; 8 claim checks.
- 20:19 `feat` Policy gains e_pisodeWith and w_orkloadWith (a per-call context cost, a table ignored every k-th call) and s_ynthesize (the k sequences worth most calls saved, from a journal, as a tool table); 30 doc examples; the existing baselines unchanged, types reblessed after review (three new exports).
- 22:15 `chore` Step 9 (explain-05) closed.
- 20:07 `docs` Experiment 05 explains where its rules come from (step 9, the user asked): a page panel and a README Background section on the Exo paper (arXiv 2611.00463), the ablation groups, D-ski as group D with the ski-rental rent-or-buy rule and its 2x bound (Karlin, Manasse, Rudolph and Sleator 1988), and the hindsight yardstick; republished.
- 21:30 `chore` Step 8 (workshop) closed.
- 20:06 `feat` The Workshop world and the Policy interpreter (step 8): nine op codes over four places, pure s_tep with preconditions, d_one, the e_xpert that stands in for a model's choice, n_ame; tool tables as data (plan A1) run by po:e_pisode and po:w_orkload, one charged and journaled call per primitive op or per fitting tool (longest by lookahead), a step budget; 32 doc examples, basics, 14 checks (the expert's op counts, tools change calls but never outcomes or ops, 5 calls saved per device, a never-fitting tool is inert, the budget stops an episode); the fixed harness on six tasks: 120 calls, 55 with the route as a tool.
- 21:05 `chore` Step 7 (journal) closed.
- 19:57 `feat` The Journal library (step 7): an agent's history as five columns (task, version, op, cost, ok), append-only (e_mpty, r_ecord, l_ength, t_otal, v_ersion); n-grams of ops within a task as base-100 keys (k_eys, d_ecode) and each one's count, cost and failures ordered by calls a tool would save (s_equences); 21 doc examples, basics, seven checks against brute force, the repeats demo.
- 20:00 `chore` Step 6 (pages) closed.
- 19:53 `docs` The site is live (GitHub Pages from gh-pages, enabled by the user): just check-live finds the catalog at 761b797 and experiment 05's page running X_eTaL in headless Chrome (4 checks).
- 19:43 `feat` The first live page (step 6, the user): experiment 05 in the browser, the investment, baseline, saving and tasks on sliders, the three harnesses' cumulative curves with the break-even, the repetition table and the X_eTaL program that computed them (Econ, the pinned X_eTaL in WebAssembly); shared/microscope (the core of X_eTaL-ML's page shell on this repo's pin); build-pages, catalog, serve, browser-check (headless Chrome), publish to gh-pages (browser-checked first), check-live; the gate runs the pages' native tests and wasm32 check; four page tests against the command-line experiment and the formulas.
- 18:35 `chore` Step 5 (related) closed.
- 19:21 `docs` docs/related.md (step 5, the user): this repository and Exo beside ICL, ICRL, XSkill, RLM, the Ralph loop, agentrail-rs, all-together-now, the continuous-learning agent and the local-model plan-execute-review loop, by six questions (what changes, where kept, persists, checked by, does the improver improve, what is taken here), with links to the author's blog posts; linked from the README, the plan and CLAUDE.md; in the gate's markdown check.
- 17:45 `chore` Step 4 (econ) closed.
- 18:57 `exp` Experiment 05, specialization cost (step 4): repetition swept over 40 tasks and four workload shapes for the fixed harness, always improving, the cost-aware ski-rental rule, hindsight and a budget; always improving costs 3.45 times the fixed harness on one-off work, ski rental stays within 1.5 times hindsight, a budget with known reuse reaches it.
- 18:57 `feat` The Econ library (step 4): a_fter, f_ixed, e_ager, h_indsight, b_reakEven, s_kiRental, r_atio, a_llocate, n_et; 18 doc examples, basics, eight property checks, the break-even demo.
- 18:57 `plan` A pages step (5) inserted after econ: the first live page on GitHub Pages, pulled forward at the user's request.
- 17:20 `chore` Step 3 (layout) closed.
- 18:06 `build` Layout (step 3): libs/<Name>/ as in X_eTaL-libraries (templates/Library, libs.py, test-libs.sh, new-lib.sh, run-lib.sh, doc-test.sh) and experiments/NN-slug/ (templates/Experiment, experiments.sh, test-experiments.sh, run-experiment.sh, new-experiment.sh), reg-rs baselines, FAIL lines fail; just test, new-lib, new-exp, run-exp, show-exp, test-lib, test-exp, bless-lib, bless-exp, doc-test; the gate runs them all; xetal.sh keeps cargo's warnings for a failed build.
- 17:00 `chore` Step 2 (xetal-pin) closed.
- 16:54 `build` The pinned X_eTaL (step 2): XETAL_COMMIT c925f58 and XETAL_LIBRARIES_COMMIT c7e812e, cloned into work/ and built by scripts/xetal.sh (just xetal, xetal-pin, libs-pin, xetal-version); Check, Parsing, Lists, Strings, Format and Statistics from X_eTaL-libraries; scripts/xt (just eval) with every library on XETAL_PATH; check-xetal and check-asks (asks R1.. against X_eTaL's ledger) in the gate.
- 16:49 `plan` The manipulator arms dropped from the Workshop (the user: a request meant for another project); the Workshop is again the research's resource and manufacturing world.
- 16:22 `chore` Step 1 (scaffold) closed.
- 16:47 `build` Scaffold (step 1): the agentrail saga foundation (8 steps), CLAUDE.md with the repo rules and AGENTS.md a symlink to it, the README (the idea, experiments, libraries, copyright, license), docs/plan.md (the Exo paper, decisions A1-A8, the Workshop's manipulator arms, libraries, experiments, sagas), docs/xetal-asks.md, .gitignore, justfile and the gate (spelling, markdown).
- 15:27 `chore` The first commit: an empty README.
