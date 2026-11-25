# X_eTaL-RSI -- Implementation Plan

Recursive self-improvement (RSI) in X_eTaL (the eXperimental
Extensible Typed Array Language, developed in `../X_eTaL`): small,
deterministic, measurable experiments in which a system improves the
harness it runs in, and then the procedure by which it improves that
harness, without changing a model's weights. The first architecture
studied is Exo (below); the repository is named for the broader
question so that later architectures fit.

| Repo | Answers |
| ---- | ------- |
| X_eTaL | what the language is |
| X_eTaL-ML | why array languages fit machine learning |
| X_eTaL-libraries | language-level extensibility (`.xtl`, `.xtlm`) |
| X_eTaL-extensions | system-level extensibility (native code) |
| **X_eTaL-RSI** | **whether an array language makes self-improvement cheaper, clearer and easier to verify** |

Source: `docs/research.pdf` (an analysis of the Exo paper with
proposed experiments, archival, not normative). This plan turns it
into sagas and steps; `docs/related.md` relates it to earlier work on
improving agents (ICL, ICRL, XSkill, RLM, the Ralph loop, agentrail-rs,
all-together-now). Development is driven by agentrail sagas (one
active saga in `.agentrail/`, finished sagas archived to
`.agentrail-archive/`), as in the sibling repos. Every step ends with
the gate (`just gate`), docs updated (`README.md`, `CHANGES.md`, this
plan, `docs/xetal-asks.md`, the library's or experiment's README), a
sane `.gitignore`, a detailed commit to `main` (with the `.agentrail/`
changes), `agentrail complete`, and a push.

## The paper: Exo

"Exo: An Architecture for Self-Improving Systems at Runtime"
(Krentsel, Ratnasamy, Stoica, Casado, Goyal; arXiv 2611.00463,
2026-10-30). A frozen LLM drives an agent harness (the *executor*);
the harness's own code, prompts, memory and tools are data the agent
may rewrite, and so is the policy that decides what to rewrite. A
protected *mechanism* layer the agent cannot edit holds an independent
*guardian* that authorizes replacing the executor, an append-only
history, versioned artifacts, the sandbox and credentials, and the
authority to roll back. The separation is the exokernel's: policy is
evolvable, mechanism is fixed.

Two ideas make the loop collapse into the system: *optimizer
symmetry* (ordinary work and policy revisions use the same action
interface) and *operational homoiconicity* (the harness's
implementation is manipulable data).

Reported results are mostly efficiency, not capability: AppWorld cost
per held-out task 0.359 to 0.141 dollars at unchanged 96.9% success;
SREGym diagnosis 81 to 53 seconds; coding cost per issue 0.507 to
0.436 at 100% resolved; in a RuneScape-like world, transfer runs cost
about a quarter of Codex's. Improvement is an investment that pays
only when reused, a smaller model benefited little (a capability
threshold), and the default guardian checks only that the replacement
builds.

### Three levels (a teaching frame, not the paper's)

1. Learn from experience: memory, retrieval, reflection.
2. Improve how work is done: tools, workflows, context policies.
3. Improve how improvement is done: the proposer, the evaluator and
   the schedule of reflection themselves.

## Guiding principle

Every claim is an experiment with a control. An experiment here is a
small X_eTaL program over a deterministic, simulated workload whose
cost is counted (model calls and tokens as units, never dollars), run
by every ablation group, with the result pinned by a reg-rs baseline.
Simulations are labeled as simulations: they show the mechanics of
RSI, they are not evidence that a model learns.

## Architecture decisions

- **A1** Policy is data, mechanism is code. X_eTaL deliberately has no
  run-time `execute` (its lang-choices B29: static types). So an
  evolvable policy is an *array program over a fixed, typed set of
  primitives* (op codes, parameter vectors, tables), interpreted by
  library code the policy cannot change; rewriting the policy is
  array manipulation, and the interpreter, the cost meter and the
  guardian are the protected mechanism. This is the research's
  "constrained policy representation" (its Strategy C) and gives
  operational homoiconicity without `execute`.
- **A2** Textual self-modification (a model writing new X_eTaL) goes
  through the mechanism outside the program: a host harness writes the
  candidate `.xtl`, and the guardian runs `xetal type`, the candidate's
  tests and held-out tasks with the pinned CLI. No step depends on it
  until the deterministic experiments are done (Saga 3).
- **A3** Deterministic first: every experiment runs with no model and
  no network, from fixed seeds (`--seed 1`), so its output is a
  baseline. A model ("oracle") is a cost-charging stand-in whose
  answers are scripted or computed; a real local LLM comes later
  behind the same interface.
- **A4** Cost is counted, not estimated: the interpreter charges each
  primitive executed, each oracle consultation (a "model call") and
  each improvement attempt, into an append-only journal (an array of
  events). Every metric (cumulative cost, break-even, accepted and
  rejected revisions) is an array expression over the journal.
- **A5** Ablation groups as in the research: A baseline (no memory),
  B memory, C tool learning, D self-improving harness, E recursive
  (the optimizer editable too). Each experiment runs the groups it can
  distinguish, on training and held-out tasks, and reports negative
  results too.
- **A6** Layout follows the siblings: `libs/<Name>/` as in
  X_eTaL-libraries (src, tests with reg-rs baselines and `types.rgt`,
  docs, demos), and `experiments/<NN-slug>/` (README.md, programs,
  reg/ baselines), one sub-project each, never reaching into another.
- **A7** X_eTaL is not tracked here: `XETAL_COMMIT` and
  `XETAL_LIBRARIES_COMMIT` pin known-good commits cloned into the
  gitignored `work/` (as X_eTaL-ML does). General libraries come from
  X_eTaL-libraries (Check for tests); RSI-specific ones live here;
  anything general we write is offered upstream.
- **A8** The guardian is layered (the research's proposal, beyond
  Exo): well-formed and typed, capability check (only allowed op
  codes), unit tests, held-out regression, resource budget, then
  shadow execution against the incumbent; a rejection keeps the old
  policy and records why. Its criteria are outside the policy's
  reach.

## Layout

```
libs/<Name>/            src/ tests/ docs/ demos/ README.md
experiments/NN-slug/    README.md, NN-slug.xtl, reg/ baselines
templates/              starting points for a library and an experiment
scripts/                the tooling (just calls these)
docs/                   plan.md, xetal-asks.md, research.pdf
work/                   gitignored: the X_eTaL clones, scratch
```

## The libraries (planned)

| Library | Alias | What |
| ------- | ----- | ---- |
| Econ | `ec:` | the economics of specialization: fixed versus improving cost, break-even, a bounded improvement budget |
| Journal | `jr:` | the append-only event history as columns; frequency of action sequences (n-grams), cost by sequence |
| Workshop | `ws:` | the deterministic task world: gather, carry, make, deliver (the research's resource and manufacturing world), with an op-code action set and its cost meter |
| Policy | `po:` | policies as op-code programs: interpret, compose, define a tool (macro op), diff two versions |
| Guardian | `gd:` | the layered acceptance checks, version history and rollback |

## The experiments (from the research, renumbered as built)

| # | Experiment | Shows | Complexity |
| - | ---------- | ----- | ---------- |
| 01 | tool synthesis | repeated action sequences found in the journal and made into tools; model calls fall | low |
| 02 | context optimization | context filtered and compressed by a learned policy; prompt size against retention | low |
| 03 | policy evolution | accepted and rejected revisions with their measurements | medium |
| 04 | recursive optimizer | group D against group E: does editing the optimizer pay? | high |
| 05 | specialization cost | when improvement pays and when it does not; a bounded budget | low |
| 06 | transfer | a harness evolved on one task family, run on unseen ones | medium |
| 07 | guardian | defective revisions rejected or rolled back, history intact | medium |
| 08 | incident diagnosis | runbooks learned in a simulated service graph | high |
| 09 | coding harness | checks and context learned from compiler and test failures | high |
| 10 | small models | a capability threshold, with constrained policies | high |

Hypotheses (from the research): H1 harness evolution cuts cost
without cutting correctness; H2 learned policies transfer; H3 editable
optimizers beat fixed ones; H4 compact X_eTaL policies take fewer
tokens to inspect and change; H5 types and purity cut defective
revisions; H6 stronger guardians pay for their cost; H7
specialization partly makes up for a weaker model; H8 structured
policies lower the capability needed to self-improve.

## Saga 1 -- foundation  [DONE 2026-11-24]

1. scaffold [DONE]: agentrail, CLAUDE.md/AGENTS.md, README, CHANGES, license
   files, this plan, the asks file, `.gitignore`, justfile, the gate.
2. xetal-pin [DONE]: the pinned X_eTaL and X_eTaL-libraries (clones in
   `work/`), `scripts/xt`, `just eval`, the X_eTaL check and the asks
   check in the gate.
3. layout [DONE]: `libs/` and `experiments/` with templates, runners (reg-rs),
   `just new-lib`, `just new-experiment`, `just test`.
4. econ [DONE]: the Econ library and experiment 05 (specialization cost):
   break-even over workloads of varying repetition, and a bounded
   budget allocated by expected reuse.
5. related [DONE]: docs/related.md (the user): this repository beside
   ICL, ICRL, XSkill, RLM, the Ralph loop, agentrail-rs,
   all-together-now and the continuous-learning architecture of the
   author's blog.
6. pages [DONE]: the first live page, on GitHub Pages (pulled forward at the
   user's request): X_eTaL-ML's page shell and publishing, and
   experiment 05 as the break-even simulator, computed in X_eTaL in the
   browser.
7. journal [DONE]: the Journal library: events as columns, n-gram counts of
   action sequences, cost per sequence, the top candidates for a tool.
8. workshop [DONE]: the Workshop world and the Policy interpreter: op codes,
   a fixed-harness baseline (group A) that consults the oracle for
   every action.
9. explain-05 [DONE]: experiment 05's page and README explain where its
   rules come from (the user asked), with references.
10. tool-synthesis [DONE]: experiment 01: groups A, B, C on the Workshop,
   tools synthesized from the journal's n-grams, model calls and
   correctness compared, held-out tasks.
11. guardian [DONE]: the Guardian library and experiment 07: layered checks,
   deliberately defective candidates, rollback with history intact.

12. page-01 [DONE]: the second live page (the user asked when): experiment 01
    in the browser.

## Saga 2 -- recursion  [PROPOSED]

Experiments 03 (policy evolution), 04 (recursive optimizer: groups D
and E, the optimizer's own parameters as an editable policy), 06
(transfer to unseen and unrelated task families), and 02 (context
optimization). A written report of results, negative ones included.

## Saga 3 -- models  [PROPOSED]

A host harness (Rust, protected) that asks a local model (llama.cpp or
Ollama) for candidate tools and policies as X_eTaL text, checks them
with the guardian (A2), and records the trajectories; experiment 10
across model sizes on the user's GPUs; the native parts belong in
`../X_eTaL-extensions`.

## Saga 4 -- explorer  [PROPOSED]

The RSI Explorer page: a fixed and a self-improving agent side by side
in the browser (WASM, the deterministic mode, clearly labeled a
simulation), the policy timeline, each version's code, pause and
replay; published as X_eTaL-ML's pages are.

## Open questions

- Q1 The cost units: model calls and a token proxy per call; whether
  to weigh oracle calls by context size (experiment 02 needs it).
- Q2 Whether the Workshop should be shared with X_eTaL-games (a game
  world the user may already have) or stay a minimal world here.
- Q3 Saga 3's model host: which local models, and where the harness
  lives (here, or X_eTaL-extensions).
