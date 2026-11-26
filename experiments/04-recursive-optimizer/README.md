# 04 Recursive optimizer

A harness that revises its tools and also how it revises them, against
fixed optimizers well and badly tuned.

A deterministic simulation on the [Workshop](../../libs/Workshop/docs/README.md)
world (the model is Workshop's expert); it shows the mechanics of an
optimizer that edits itself, not that a model would.

## Question

Exo lets the policy that decides what to rewrite be rewritten too
(Krentsel et al., [arXiv 2611.00463](https://arxiv.org/abs/2611.00463)).
Does an optimizer that edits its own settings (group E) beat one whose
settings are fixed (group D), what does the editing cost, and when
does it pay (hypothesis H3: editable optimizers beat fixed ones)?

## Method

Twenty-four batches of three tasks; the recipe shifts every six
batches: 1 wood and 1 ore a device, then 2 wood, then 2 ore, then 2 of
each. Costs: a call 500 tokens, a tool's schema 20 in every prompt,
writing a tool 1500, the meta-level 5 for each journal event it reads.

The optimizer is [Evolve](../../libs/Evolve/docs/README.md)'s vector
`lo hi k window period`: tools of `lo` to `hi` ops, the `k` worth the
most calls saved, learned from the last `window` tasks, a revision
after every `period` batches, each reviewed by the layered
[Guardian](../../libs/Guardian/docs/README.md) on the batch just done
(as in experiment 03).

Group E's meta-level, before every third batch, replays the last 3
batches under the running optimizer and each one a step from it (`hi`,
`k`, `period` plus or minus 1, `window` plus or minus 3) and adopts the
one whose replay spent the fewest tokens. The replay calls no model:
tools change how many calls a task takes, never which ops it runs, so
it needs only the recorded ops, and it is charged for the journal
events it reads. The meta-level and the Guardian's settings are fixed:
nothing improves the improver of the optimizer (no tower).

## Groups

| Group | Optimizer | Edits it |
| ----- | --------- | -------- |
| A | none: the fixed harness | |
| D from 03's | `2 8 3 6 1`, experiment 03's settings | no |
| D bad | `2 3 3 3 1`: short tools from a short window | no |
| D best | `2 8 1 3 1`: the best of 180 fixed optimizers on this very stream, found afterwards (no harness could know it in advance) | no |
| E from each | the same three starts | every third batch |

## Run

```bash
just run-exp 04        # the experiment and its checked claims (about 20 s)
just show-exp 04       # each statement, then its output
just test-exp 04       # check its baseline
```

## Results

All tokens (production, writing, review, and E's meta-level):

| Start | D (fixed) | E (edits itself) | E's meta-level |
| ----- | --------- | ---------------- | -------------- |
| experiment 03's settings | 948940 | 745680 | 44240 |
| bad | 1809720 | 880740 | 39340 |
| best (hindsight) | 442680 | 470780 | 28100 |

The fixed harness, which never evolves, spends 647000.

The optimizer E ran: from 03's settings it moved `hi` 8 to 7 (batch 4)
and `k` 3 to 2 (batch 16); from the bad one only `k` 3 to 4 (batch
10); from the best one it never moved.

E from the bad start, with other meta-level settings:

| Replay R batches, retune every M | 3, 3 | 6, 3 | 3, 6 |
| -------------------------------- | ---- | ---- | ---- |
| all tokens | 880740 | 1062535 | 1094360 |

What it shows:

- How the optimizer is tuned matters more than whether the harness
  evolves. The best fixed optimizer saves 32% against the fixed
  harness; experiment 03's settings and the bad ones spend more than
  never evolving at all (one tool from a short window beats three: a
  smaller candidate is cheaper to write and to carry in every prompt).
- Editing the optimizer pays when it starts badly tuned. E halved what
  the bad optimizer spends, with a single change, and cut experiment
  03's settings by 21%. Its meta-level reads cost 4% to 6% of its
  tokens.
- But it is no substitute for good settings. From neither poor start
  did E reach the fixed harness, let alone the best optimizer: one step
  at a time, judged on 3 batches of the past, it moves slowly and only
  where a short replay shows a difference.
- Started tuned, E is pure overhead: it never moved, and paid 6% for
  checking.
- The meta-level's own settings matter as much: replaying longer, or
  retuning less often, did worse from the bad start. Choosing them is
  the level above, and fixing it (as here) only stops the tower; it
  does not remove the choice.
- H3 holds conditionally: an editable optimizer beats a fixed one that
  is badly tuned, not one that is well tuned, and here it never beat
  not evolving at all.

## Limits

- The meta-level is one local search: one step in one setting, judged
  on a replay of 3 batches that starts from an empty journal, so a
  setting whose effect shows only over a longer history (the window)
  is hard for it to see. Wider moves, longer replays or a model
  proposing settings are untested (Saga 3).
- The replay is exact because the world is deterministic and the
  model always right; a real replay of recorded traces is an estimate.
- The best optimizer was found by trying 180 on this stream: it is a
  yardstick, not a group a harness could run.
- One stream, one world, and the Guardian reviews every candidate at
  full cost (experiment 03): with cheaper reviews the ranking of the
  optimizers could change.
