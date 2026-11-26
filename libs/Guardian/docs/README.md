# Guardian

The layered guardian: checks a candidate policy must pass before it
replaces the running one, the version history and rollback.

```
"gd:" u_se< "Guardian"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `gd:` is the
recommended alias. It uses Policy and Journal.

Exo's guardian authorizes replacing the running harness; by default it
checks only that the replacement builds. This one is layered (plan
A8), cheapest first, stopping at the first layer that fails. A policy
is `(T, budget)`: a [Policy](../../Policy/docs/README.md) tool table and
the model calls a task may take. The criteria are arguments the policy
never sees.

| Layer | Name | The candidate fails when |
| ----- | ---- | ------------------------ |
| 1 | formed | T is not a matrix of op codes 0 to 9, each row its ops then 0s, at least 2 ops a tool |
| 2 | capable | a tool holds an op not in `allowed` |
| 3 | context | it has more than `most` tools |
| 4 | correct | a suite task is not done within its budget |
| 5 | regression | a held-out task is not done, or they cost more tokens than under the running policy |
| 6 | shadow | recent tasks do not cost strictly fewer tokens than under the running policy |

Layers 4 to 6 run episodes, so the review costs tokens; it reports how
many. When the held-out and recent tasks are the same, layer 6 reuses
layer 5's runs (the world is deterministic), so judging both on the
latest work costs no more than judging regressions alone.

## Functions

| Function | Type | What |
| -------- | ---- | ---- |
| `L gd:f_ormed T` | `Int -> a -> Int` | layer 1 |
| `allowed gd:c_apable T` | `a -> a -> b` | layer 2 |
| `(suite, held, recent, allowed, most, call, per, upto) gd:r_eview (cand, running)` | `(...) -> ((Int, Int), (Int, Int)) -> (e, Int)` | the first layer failed (0: none) and the review's tokens; layers past upto unchecked (1 is Exo's default) |
| `gd:b_egin (T, b)` | `(a, b) -> history` | a history of one running version |
| `h gd:p_ropose ((T, b), verdict)` | `history -> ((a, b), c) -> history` | the candidate kept, and running if its verdict is 0 |
| `gd:a_ctive h` | `history -> (a, b)` | the running policy |
| `gd:r_ollback h` | `history -> history` | running the last accepted version before the running one |

A history is `(tables, budgets, verdicts, active)`: every policy ever
proposed (tables boxed), every verdict, the running version's number.
Nothing is ever removed from it, rejected candidates included.

## Examples

From `../tests/basics.xtl`, one candidate made to fail each layer, then
the route accepted, and Exo's default accepting a budget cut it cannot
see:

```
(1, 0)
(2, 0)
(3, 0)
(4, 3120)
(5, 61600)
(6, 60000)
(0, 30400)
(0, 0)
```

## Demos

- [`demos/review.xtl`](../demos/review.xtl): one candidate reviewed to
  each depth: what each layer adds to the review's cost.

## Provenance

Written for X_eTaL-RSI after the Exo paper's guardian (Krentsel et al.,
arXiv 2611.00463) and the layered guardian proposed in
`docs/research.pdf`.
