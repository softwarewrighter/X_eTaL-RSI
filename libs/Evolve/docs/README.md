# Evolve

The improvement loop as protected mechanism: an optimizer vector that
learns, reviews and swaps a harness's tools as the work goes on.

```
"ev:" u_se< "Evolve"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `ev:` is the
recommended alias. It uses Policy, Journal and Guardian.

Exo's executor rewrites its own harness as it works. Here the harness
is a [Policy](../../Policy/docs/README.md) tool table and a call budget,
and the optimizer that decides how it is rewritten is data too: an Int
vector

```
o = lo hi k window period
```

tools of `lo` to `hi` ops, the `k` worth the most calls saved, learned
from the journal of the last `window` tasks, a revision after every
`period` batches of work (0: never). This library is the fixed
interpreter of that vector. Before a batch that is due it:

1. synthesizes a candidate from the recent journal (Policy's
   `s_ynthesize`); an empty candidate, or one equal to the running
   table, is not proposed;
2. has the [Guardian](../../Guardian/docs/README.md) review it against
   the running policy, its held-out and recent tasks the batch just
   done;
3. records it and the verdict in the append-only history;

and then runs the batch with whatever policy is running. Every token is
counted: production, writing the candidate, the review.

The settings the optimizer cannot change are
`g = (suite, allowed, most, call, per, write, upto)`: the Guardian's
test suite, the ops a tool may hold, the most tools, a model call's
tokens, a tool's tokens in every prompt, the tokens to write one tool,
and the Guardian layers checked (1 is Exo's default, "it builds").

## Functions

| Function | Type | What |
| -------- | ---- | ---- |
| `g ev:e_volve (o, (T0, b), batches)` | `(...) -> (Int, (Int, Int), c) -> (history, journal, Int)` | batches (n by m by 3) worked and revised: the history, the journal (tasks numbered 1 to n*m) and a report row per batch |
| `o ev:c_andidate (j, L)` | `Int -> (journal, Int) -> Int` | the tool table optimizer o learns from journal j, rows of L ops |
| `i ev:d_ue period` | `Int -> Int -> a` | 1 if a revision comes before batch i |
| `n ev:r_ecent j` | `a -> journal -> journal` | the events of the last n tasks |
| `ev:t_okens report` | `a -> a` | production, writing and review tokens in all |

A report row is `verdict version done production writing review`: the
revision before the batch (-1 none proposed, 0 accepted, else the
Guardian layer that failed), the version that ran the batch, its tasks
done and the tokens of each part.

## Examples

From `../demos/adapt.xtl`: six batches of three tasks, the recipe
shifting from 1 to 2 wood a device at batch 4; three tools of 2 to 8
ops learned from the last 6 tasks before every batch, all six layers
checked:

| Harness | Production | Writing | Review |
| ------- | ---------- | ------- | ------ |
| never revised (period 0) | 148000 | 0 | 0 |
| revised before every batch | 66920 | 13500 | 93800 |

The version running each batch is `1 2 2 2 3 4`: tools accepted before
batch 2, no other accepted until the recipe shifted, a new set accepted
after the first batch of the new recipe and refined once more.

## Demos

- [`demos/adapt.xtl`](../demos/adapt.xtl): a harness adapting to a
  shifted recipe, against the fixed harness.

## Provenance

Written for X_eTaL-RSI.
