# 03 Policy evolution

A harness that keeps revising its tools as the work shifts, every
revision reviewed and recorded.

A deterministic simulation on the [Workshop](../../libs/Workshop/docs/README.md)
world (the model is Workshop's expert); it shows the mechanics of a
harness that rewrites itself as it works, not that a model proposes
such revisions.

## Question

Exo's executor improves its own harness during ordinary work, each
replacement authorized by a guardian (Krentsel et al.,
[arXiv 2611.00463](https://arxiv.org/abs/2611.00463)). When the work
shifts, does a harness that keeps revising its tools beat one whose
tools were learned once, and what do the revisions and their reviews
cost (hypothesis H1: harness evolution cuts cost without cutting
correctness)?

## Method

Twelve batches of three tasks. The recipe shifts twice: 1 wood and 1
ore a device in batches 1 to 4, 2 wood in batches 5 to 8, 2 ore in
batches 9 to 12. Costs: a call 500 tokens, a tool's schema 20 in every
prompt, writing a tool 1500.

The loop is [Evolve](../../libs/Evolve/docs/README.md)'s: an optimizer
vector `lo hi k window period` decides what is learned and when.
Before a batch that is due, three tools of 2 to 8 ops are synthesized
from the journal of the last 6 tasks; the [Guardian](../../libs/Guardian/docs/README.md)
reviews them against the running policy, its held-out and recent tasks
the batch just done; the batch then runs with whatever is running.

## Groups

| Group | Tools | Revised | Guardian |
| ----- | ----- | ------- | -------- |
| A | none | never | |
| C | three learned once from a training journal (as in experiment 01) | never | |
| D | learned from the last 6 tasks | before every batch | all six layers |
| D-unguarded | the same | before every batch | layer 1 (Exo's default, "it builds") |
| D-every-4 | the same | every fourth batch | all six layers |

## Run

```bash
just run-exp 03        # the experiment and its checked claims
just show-exp 03       # each statement, then its output
just test-exp 03       # check its baseline
```

## Results

| Group | Production | Writing | Review | All tokens | Tasks done | Accepted |
| ----- | ---------- | ------- | ------ | ---------- | ---------- | -------- |
| A | 336000 | 0 | 0 | 336000 | 36 | 0 |
| C | 312480 | 0 | 0 | 312480 | 36 | 0 |
| D | 165480 | 25500 | 318360 | 509340 | 36 | 4 |
| D-unguarded | 189000 | 21000 | 0 | 210000 | 36 | 5 |
| D-every-4 | 364080 | 9000 | 133040 | 506120 | 36 | 2 |

D's timeline (verdict: -1 nothing proposed, 0 accepted, else the
Guardian layer that failed):

| Batch | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
| ----- | - | - | - | - | - | - | - | - | - | -- | -- | -- |
| recipe | 1,1 | 1,1 | 1,1 | 1,1 | 2,1 | 2,1 | 2,1 | 2,1 | 1,2 | 1,2 | 1,2 | 1,2 |
| verdict | -1 | 0 | 5 | 5 | 5 | 0 | 0 | -1 | -1 | -1 | 0 | -1 |
| version | 1 | 2 | 3 | 3 | 3 | 4 | 5 | 5 | 5 | 5 | 6 | 6 |
| production | 21000 | 8400 | 6720 | 8400 | 13440 | 11760 | 6720 | 8400 | 26880 | 38640 | 6720 | 8400 |

What it shows:

- Evolving halves production. D's production tokens are less than half
  of the fixed harness's; tools learned once (C) save under 10%, as
  their route stops fitting at the first shift and its schema still
  costs every prompt.
- D adapts with a lag. The batch after each shift runs on old tools
  until a revision fits the new recipe, and after the second shift it
  ran stale tools for two batches (9 and 10 cost more than the fixed
  harness): a window of 6 tasks still held mostly the old recipe, so
  the candidate learned was the running table again and nothing was
  proposed. The window is the optimizer's to tune (experiment 04).
- Reviewing every revision costs more than evolving saves. D's reviews
  (318360 tokens) cost more than its production saved; at this scale,
  three tasks a batch, a review that reruns the suite and the last
  batch under both policies outweighs the batch it guards. Unguarded
  evolution spent the fewest tokens of all.
- The guardian still earns something: it kept worse candidates out of
  production (D's production is below the unguarded one's), and with
  synthesis from the journal no defective candidate arose here, so its
  safety layers had nothing to stop (experiment 07 shows what they
  stop).
- Revising on a schedule that ignores the work can be worse than not
  revising. D-every-4 revised before batches 5 and 9, the first batches
  of a new recipe, from journals of the old one; the guardian, judging
  on the batch just done, accepted tools that no longer fit, and that
  group spent more in production than the fixed harness.
- The history is whole: every candidate, accepted or rejected, and
  every verdict stays in it.

## Limits

- The world is tiny: a review costs about as much as several batches of
  work. With more production between revisions a review is a smaller
  share; how often to revise is itself a setting (experiment 04).
- Candidates come from one fixed synthesizer; a model would propose
  others, and some could be defective (Saga 3).
- Each review reruns the running policy on the last batch, though
  production has just measured it; with that reuse a review would cost
  less.
- One stream, one shift pattern.
