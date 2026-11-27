# Report: experiments 01 to 07

What the deterministic experiments in this repository show about
self-improving harnesses, hypothesis by hypothesis, negative results
included. Each experiment's README has its method, numbers and limits;
this page draws them together. The architecture studied is Exo
(Krentsel, Ratnasamy, Stoica, Casado, Goyal; "Exo: An Architecture
for Self-Improving Systems at Runtime",
[arXiv 2611.00463](https://arxiv.org/abs/2611.00463)); the hypotheses
are the research's, listed in [plan.md](plan.md).

## What was run

Every experiment is a small X_eTaL program over a deterministic world,
the Workshop: gather wood and ore, make devices, deliver them, nine op
codes. The "model" is a scripted expert that is always right and is
charged for every call (500 tokens). A harness is data: a table of
tools, each a sequence of op codes, run by a fixed interpreter (plan
A1; X_eTaL has no run-time `execute`). What changes the harness is data
too: an optimizer vector read by fixed library code (Evolve), and every
change is reviewed by a layered guardian (Guardian) before it runs.
Costs are counted in tokens: production (the work), writing tools,
reviews, and in experiment 04 a meta-level's reads.

These are simulations. They show the mechanics and the economics of
self-improvement in a world where they can be measured exactly; they
are not evidence that a model would propose or benefit from such
changes.

| # | Experiment | Its finding in one line |
| - | ---------- | ----------------------- |
| 01 | [tool synthesis](../experiments/01-tool-synthesis/README.md) | tools learned from history cut calls by more than half on similar work; the same lesson as a hint costs 21% more; long tools stop fitting when work shifts |
| 02 | [context optimization](../experiments/02-context-optimization/README.md) | which tools to keep in the prompt depends on the price of context; retiring idle tools churns |
| 03 | [policy evolution](../experiments/03-policy-evolution/README.md) | evolving halves production, but reviewing every revision costs more than it saves at this scale |
| 04 | [recursive optimizer](../experiments/04-recursive-optimizer/README.md) | tuning the optimizer matters more than evolving; editing it rescues a bad one, not a good one |
| 05 | [specialization cost](../experiments/05-specialization-cost/README.md) | improvement pays only past a break-even in reuse; a rent-or-buy rule bounds the loss |
| 06 | [transfer](../experiments/06-transfer/README.md) | general tools transfer to near work, specific ones nowhere, nothing to far work |
| 07 | [guardian](../experiments/07-guardian/README.md) | layered checks stop each defect at its own layer; "it builds" lets them through |

## Hypotheses

### H1 Harness evolution cuts cost without cutting correctness

**Holds for production; not, at this scale, once verification is
paid.** Learned tools cut model calls by more than half on work like
their history (01: 98 to 44 calls) and never changed what was done:
every group of every experiment finished every task, with the same
ops. A harness that keeps revising halves production tokens on a
shifting stream (03: 165480 against 336000). But a guarded revision
before every batch costs more in reviews than it saves (03: 318360
review tokens; 509340 in all against 336000), and only some optimizer
settings come out ahead of never evolving (04: the best of 180 saves
32%; experiment 03's settings spend 44% more). Improvement is an
investment that pays only when reused (05: always improving costs 3.45
times the fixed harness on work that never recurs).

### H2 Learned policies transfer

**Holds for general tools on near work only.** A tool that is a whole
procedure saves 72% on the work it was learned from and is dead weight
everywhere else (06: 4% more than no tools); short, general tools save
less at home (21%) but still 13% to 17% on near families, within 5% of
a harness that keeps evolving there. Nothing transferred to a family
with a different route, and a learned harness was never a better start
than none off its own recipe. Experiment 01 found the same split on
held-out tasks.

### H3 Editable optimizers beat fixed ones

**Holds only against a badly tuned fixed optimizer.** A meta-level
that replays recent work under neighboring optimizer settings and
adopts the cheapest halved the cost of a badly tuned optimizer and cut
experiment 03's settings by 21%, for 4% to 6% of its tokens (04). It
never moved from well-tuned settings (pure overhead, 6%), never
reached the fixed harness from a poor start, and its own settings
(how much to replay, how often) mattered as much: fixing them stops
the tower of improvers but does not remove the choice.

### H4 Compact X_eTaL policies take fewer tokens to inspect and change

**Not measured.** The policies here are small integer arrays (a tool
table, a five- or six-number optimizer), and every metric is an array
expression over the journal, but no model read or edited them, so
there is nothing to count yet (Saga 3).

### H5 Types and purity cut defective revisions

**Not measured as a rate.** Policies as data cannot be ill-typed by
construction, and the guardian's first layers stop malformed and
over-capable tables before anything runs (07: an unknown op code at
layer 1, an unsupervised delivery at layer 2). Whether types reduce
how often a model proposes defects needs a model (Saga 3).

### H6 Stronger guardians pay for their cost

**Holds for correctness, not for tokens, at this scale.** The layered
guardian stopped every defective candidate (07), where Exo's default,
"it builds", let through a failed task and three unsupervised
deliveries, and a circuit breaker rolled back only after the damage.
In tokens, reviews cost more than the production they protected (07:
27,000 to 35,000 tokens a candidate; 03: unguarded evolution spent the
fewest tokens). A guardian can also be wrong the other way: judged on
a fixed held-out set it rejected the adaptation to shifted work (07).
The review's cost relative to the work it guards is what decides H6
in tokens; here the work between revisions is small.

### H7 Specialization partly makes up for a weaker model; H8 structured policies lower the capability needed to self-improve

**Not tested.** Both need models of different sizes (experiment 10,
Saga 3).

## Across the experiments

- **The economics decide.** Every improvement is a cost now for a
  saving later. Whether it pays depends on reuse (05), on how close new
  work is to old (06), on the price of context (02) and on the cost of
  checking (03, 07), more than on whether the harness can improve.
- **How to improve is the main lever.** The same loop with different
  settings ranged from 32% cheaper than not evolving to 2.7 times dearer
  (04). Settings that suit one stream fail on another (02, 06).
- **Verification is the dominant cost at small scale.** Reviews rerun
  work; reusing runs the world makes redundant helped (the Guardian's
  shadow layer reuses its regression runs), and caching the running
  policy's measured cost would help more.
- **Policy as data works without `execute`.** Tools, optimizers and
  context rules were all arrays read by fixed code; the guardian, the
  journal and the cost meter stayed out of the policy's reach (plan
  A1, A8).

## Limits

- One small, deterministic world, a scripted model that is always
  right, and costs that are assumptions (shown in each program).
- Candidates come from a fixed synthesizer, not a model; the defective
  ones in experiment 07 were written by hand.
- At this scale a review costs as much as several batches of work, so
  results about verification would shift with more work between
  revisions.
- H4, H5, H7 and H8 need a model in the loop: Saga 3.

## Live pages

Experiments 01, 04 and 05 run in the browser, computed by the same
X_eTaL programs: <https://softwarewrighter.github.io/X_eTaL-RSI/>.
