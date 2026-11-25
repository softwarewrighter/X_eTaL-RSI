# 01 Tool synthesis

Repeated action sequences found in the journal become tools; model
calls fall, and what transfers.

A deterministic simulation on the [Workshop](../../libs/Workshop/docs/README.md)
world: it shows the mechanics of learning tools from a history; it is
not evidence that a model learns. The model is Workshop's expert
(always right), so the groups differ only in how often the model is
called and what each call costs.

## Question

Exo's first kind of improvement: the system notices it keeps doing the
same steps and builds itself a tool, so later work needs fewer model
calls (Krentsel et al., [arXiv 2611.00463](https://arxiv.org/abs/2611.00463)).
Does a tool learned from the journal cut calls without changing what
gets done (hypothesis H1), is it better as a tool than as advice in the
prompt (the in-context memory of XSkill and agentrail-rs, see
[docs/related.md](../../docs/related.md)), and does it transfer to
tasks that differ (H2)?

## Method

1. Training: the fixed harness does 8 tasks (1 to 5 devices of 1 wood
   and 1 ore); its journal is the history (142 calls).
2. Learning ([Policy](../../libs/Policy/docs/README.md)'s
   `po:s_ynthesize`, over [Journal](../../libs/Journal/docs/README.md)'s
   n-grams): the 3 op sequences of 2 to 8 ops worth the most calls
   saved, count * (n - 1); and, for C-short, the best 3 of 2 or 3 ops.
3. Held out: 4 tasks of the same recipe and other sizes, and 4 tasks of
   shifted recipes (2 wood, 2 ore or both).

Assumed costs, in tokens: a model call 500; writing a tool or a hint
1500, once; each tool's schema adds 20 to every prompt, each hint's
text 60 (a hint is a sequence spelled out for the model to follow); a
model given hints ignores them on every 4th call (agents "ignore rules
even when they're present"). These numbers are assumptions; the checks
below hold for the direction of each result, not the exact values.

## Groups

| Group | Memory | Tools | Harness | Optimizer | What it is given |
| ----- | ------ | ----- | ------- | --------- | ---------------- |
| A baseline | no | no | no | no | nothing: one call per op |
| B memory | yes | no | no | no | the 3 learned sequences as hints in the prompt |
| C tools | yes | yes | no | no | the same 3 sequences as tools the harness runs |
| C-short | yes | yes | no | no | 3 tools of at most 3 ops |

## Run

```bash
just run-exp 01        # the experiment, and its checks
just show-exp 01       # each statement, then its output
just test-exp 01       # check its baselines
```

## Results

Learned: the make-a-device route `go-forest gather-wood go-mine
gather-ore go-workshop make` (1 2 3 4 5 6), and two 8-op sequences
that run from one device into the next. At most 3 ops: 1 2 3, 2 3 4,
3 4 5.

| Held out | Group | Tasks done | Calls | Tokens |
| -------- | ----- | ---------- | ----- | ------ |
| same recipe | A | 4 | 98 | 49000 |
| same recipe | B | 4 | 44 | 29920 |
| same recipe | C | 4 | 44 | 24640 |
| same recipe | C-short | 4 | 68 | 38080 |
| shifted | A | 4 | 91 | 45500 |
| shifted | B | 4 | 91 | 61880 |
| shifted | C | 4 | 91 | 50960 |
| shifted | C-short | 4 | 69 | 38640 |

On all 8 held-out tasks, after paying 4500 tokens to learn: B saves
-1800 tokens against A (a loss), C 14400, C-short 13280.

What it shows:

- Tools cut calls by more than half on work like the training (98 to
  44) and never change what is done: every group finishes every task
  with the same ops (H1, in this world).
- The same knowledge as hints reached the same call count here,
  because the synthesized 8-op sequences let a model that ignored a
  hint pick the route up again, but paid for it in context on every
  call: B costs 21% more tokens than C on the same recipe.
- The long route is over-specialized: with 2 wood or 2 ore it never
  fits, saves nothing, and its schema makes every call dearer (C pays
  12% more than A, B 36% more). Short tools transfer: C-short still
  cuts the shifted tasks' calls by 24%. Overall C and C-short come out
  close, from opposite strengths (H2: what transfers is the short,
  general step, not the whole procedure).
- Both points (`checks.xtl`) argue for a harness that tracks how often
  each tool fits and retires ones that stopped paying: experiment 05's
  economics applied per tool, a step toward experiments 03 and 06.

## Limits

- The model never errs and never forgets a tool; real models do both.
- Costs are assumed, not measured (Saga 3 measures them with a local
  model).
- One learning pass from one history; the harness does not keep
  learning on held-out work (experiment 03).
