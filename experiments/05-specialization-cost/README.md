# 05 Specialization cost

When an improvement pays for itself, when it does not, and a rule that
is never far from the best.

A deterministic simulation: it shows the economics of
self-improvement on scripted workloads; it is not evidence that a
model learns. Built on the [Econ](../../libs/Econ/docs/README.md)
library.

## Question

Self-improvement is an investment: Exo's own runs cost more at first
and pay back only when what was learned is reused. Given what an
improvement costs and saves, how much repetition does a workload need
before improving pays, and can a harness decide when to improve without
knowing the future? (Research hypotheses H1 and the "economics of
specialization" experiment.)

The model: each task kind can be improved once for i = 3000 tokens,
cutting its cost from b = 1000 to a = 450 tokens (the research's
example, a 55% saving). Break-even: 3000 / 550 = 5.45 tasks of a kind.

## Groups

| Group | Memory | Tools | Harness | Optimizer | Rule |
| ----- | ------ | ----- | ------- | --------- | ---- |
| A baseline | no | no | no | no | never improves |
| D always | yes | yes | yes | no | improves every kind before its first task |
| D-ski cost-aware | yes | yes | yes | no | improves a kind after k = 6 baseline tasks of it (ski rental) |
| best (yardstick) | - | - | - | - | per kind, the cheaper choice knowing its count |
| budgeted | yes | yes | yes | no | knows each kind's expected uses; spends at most 9000 tokens by return on investment |

## Run

```bash
just run-exp 05        # the experiment
just show-exp 05       # each statement, then its output
just test-exp 05       # check its baseline
```

## Results

40 tasks in all, r tasks of each of 40 / r kinds (tokens):

| uses per kind | 1 | 2 | 4 | 5 | 8 | 10 | 20 | 40 |
| ------------- | - | - | - | - | - | -- | -- | -- |
| A fixed | 40000 | 40000 | 40000 | 40000 | 40000 | 40000 | 40000 | 40000 |
| D always | 138000 | 78000 | 48000 | 42000 | 33000 | 30000 | 24000 | 21000 |
| D-ski | 40000 | 40000 | 40000 | 40000 | 49500 | 43200 | 30600 | 24300 |
| best | 40000 | 40000 | 40000 | 40000 | 33000 | 30000 | 24000 | 21000 |

Workload shapes (A, D, D-ski, best):

| workload | A | D | D-ski | best |
| -------- | - | - | ----- | ---- |
| one-off, 40 kinds once | 40000 | 138000 | 40000 | 40000 |
| routine, 4 kinds x 10 | 40000 | 30000 | 43200 | 30000 |
| heavy tail, 15 kinds, 1 to 21 uses | 62000 | 72900 | 57800 | 47900 |
| drifting, 8 kinds x 5 (just under break-even) | 40000 | 42000 | 40000 | 40000 |

With a 9000-token budget and the heavy tail's uses known, the three
most-used kinds are improved: 47900 tokens, the hindsight cost.

What it shows:

- Improving always is a bad bet on unrepeated work: 3.45 times the
  fixed harness when nothing recurs, still 1.05 times at 5 uses per
  kind; it wins only past the break-even, and then by up to 47%.
- The cost-aware rule never loses to the fixed harness on rare work
  and stays within twice the best in hindsight (here at most 1.5
  times), but on routine work it pays the baseline for the first six
  tasks of each kind, so it trails "always improve" there (43200
  against 30000).
- Knowing reuse is worth the most: the budget, spent where it returns
  most, reaches the hindsight cost. Predicting reuse is therefore
  itself something a self-improving system should learn (a later
  experiment).

## Limits

- One improvement per kind, with a known cost and saving; real
  improvements have uncertain savings, can fail, and interact.
- No model: costs are given, not measured.
- Kinds are independent; transfer of one improvement to several
  kinds (experiment 06) is not modeled.
