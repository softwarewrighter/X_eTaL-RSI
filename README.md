<h1>
  <img src="images/xetal-logo-orange.png" alt="X_eTaL logo" height="60" align="absmiddle">
  X_eTaL-RSI
</h1>

<p align="center">
  <b><a href="https://softwarewrighter.github.io/X_eTaL-RSI/">The live experiments</a></b>
  -- X_eTaL running in your browser (WebAssembly)
</p>

Recursive self-improvement (RSI) in
[X_eTaL](https://github.com/softwarewrighter/X_eTaL), the eXperimental
Extensible Typed Array Language: small, deterministic experiments in
which a system improves the harness it runs in, and then the procedure
by which it improves that harness, without changing a model's weights.

## The idea

The first architecture studied is Exo ("Exo: An Architecture for
Self-Improving Systems at Runtime", arXiv 2611.00463). A frozen model
drives an agent harness whose code, memory, tools and prompts are data
the agent may rewrite, and so is the policy that decides what to
rewrite. A protected layer the agent cannot change holds an
independent guardian, an append-only history and the authority to roll
back. The gains it reports are mostly efficiency: work done again
costs less because the system has built itself tools and procedures.

The question this repository asks is whether an array language makes
that loop cheaper, clearer and easier to verify:

- a policy is a small array program over a fixed, typed set of
  operations, so rewriting it is array manipulation, and the
  interpreter, the cost meter and the guardian stay out of its reach;
- the execution history is a table, so "which action sequences repeat,
  what do they cost, which fail" is an array expression, not a prompt;
- every change is a diff of a short program, checked by types and
  tests before it runs.

## Experiments

Each experiment runs against a control, with the ablation groups it
can tell apart (baseline, memory, tool learning, self-improving,
recursive), on deterministic simulated workloads. Simulations show
the mechanics of RSI; they are not evidence that a model learns, and
each one says so.

| # | Experiment | Status |
| - | ---------- | ------ |
| 01 | [tool synthesis](experiments/01-tool-synthesis/README.md): repeated action sequences become tools; tools beat hints, short tools transfer | done, [live](https://softwarewrighter.github.io/X_eTaL-RSI/01-tool-synthesis/) |
| 03 | [policy evolution](experiments/03-policy-evolution/README.md): a harness revising its tools as the work shifts; reviews cost more than they save at small scale | done |
| 04 | [recursive optimizer](experiments/04-recursive-optimizer/README.md): an optimizer that edits its own settings rescues a badly tuned one, but tuning matters more | done, [live](https://softwarewrighter.github.io/X_eTaL-RSI/04-recursive-optimizer/) |
| 05 | [specialization cost](experiments/05-specialization-cost/README.md): when improvement pays, and a bounded budget | done, [live](https://softwarewrighter.github.io/X_eTaL-RSI/05-specialization-cost/) |
| 07 | [guardian](experiments/07-guardian/README.md): defective revisions rejected or rolled back; what verifying costs | done |

The full list, the hypotheses and the roadmap are in
[docs/plan.md](docs/plan.md).

## Related work

[docs/related.md](docs/related.md) sets this beside earlier ways of
improving agents without retraining: in-context learning, in-context
reinforcement learning, XSkill, Recursive Language Models, the Ralph
loop, agentrail-rs, all-together-now, and a continuous-learning
architecture, by what each changes, where it is kept, who checks it and
whether the improver itself improves.

## Libraries

| Library | What | Status |
| ------- | ---- | ------ |
| [Econ](libs/Econ/docs/README.md) | the economics of specialization: break-even, ski rental, budgets | done |
| [Journal](libs/Journal/docs/README.md) | the event history as columns; repeated sequences and their cost | done |
| [Workshop](libs/Workshop/docs/README.md) | a deterministic resource and manufacturing world (gather, carry, make, deliver) | done |
| [Policy](libs/Policy/docs/README.md) | policies as op-code tables run by a fixed interpreter: episodes, tools, calls | done |
| [Guardian](libs/Guardian/docs/README.md) | layered acceptance checks, versions and rollback | done |
| [Evolve](libs/Evolve/docs/README.md) | the improvement loop: an optimizer vector that learns, reviews and swaps tools as work goes on | done |

## Development

X_eTaL is not part of this repository: `XETAL_COMMIT` and
`XETAL_LIBRARIES_COMMIT` pin the known-good commits, and `just xetal`
clones X_eTaL and X_eTaL-libraries into `work/` (gitignored), builds the
CLI and links `bin/xetal`. It needs Rust (cargo), and
[uv](https://docs.astral.sh/uv/) for the scripts. `just eval EXPR`
evaluates an expression with every library on the search path.

Each library is its own directory, `libs/<Name>/` (`src/`, `tests/`
with reg-rs baselines and its exports' types, `docs/`, `demos/`), and
each experiment is `experiments/NN-slug/` (README, programs, `reg/`
baselines). `just new-lib` and `just new-exp` start one from
`templates/`; `just test` checks every baseline, `just run-exp NN` runs
an experiment, and `just bless-lib` or `just bless-exp` accepts new
output after its diff is reviewed.

An experiment with a `web/` app has a live page: `just pages` builds the
site into `pages/` (not tracked) with trunk, `just serve` serves it
locally, and `just publish` makes it the `gh-pages` branch that GitHub
Pages serves (each page is checked in headless Chrome first). The pages
share `shared/microscope`, which runs the pinned X_eTaL in the browser.

`just` lists the tasks; `just gate` is the pre-commit gate. Work is
planned as agentrail sagas (`agentrail status`); the decisions are in
[docs/plan.md](docs/plan.md), and what this repository needs from
X_eTaL is in [docs/xetal-asks.md](docs/xetal-asks.md). Changes are
listed in [CHANGES.md](CHANGES.md).

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).
