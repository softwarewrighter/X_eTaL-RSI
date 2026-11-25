# Policy

Policies as data: a harness's tools as an op-code table, run by an
interpreter the policy cannot change.

```
"po:" u_se< "Policy"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `po:` is the
recommended alias. It uses Workshop and Journal.

X_eTaL has no run-time execute (its lang-choices B29), so what a
harness learns is data: a tool table T, one row per tool, each row a
sequence of Workshop op codes padded with 0. An empty table is the
fixed harness. This library is the protected mechanism: it runs
episodes, charges model calls and journals every op; a policy can
change only T.

One model call: the model (Workshop's expert) looks ahead as many ops
as T has columns and uses the longest tool whose ops are exactly what it
would do next, or does one op itself. The chosen ops run until one
fails. Each op is journaled with the call's cost on the call's first op
and 0 on the rest, so the calls are the events with a cost.

## Functions

| Function | Type | What |
| -------- | ---- | ---- |
| `po:n_one L` | `Int -> a` | the empty tool table, rows of L ops |
| `(task, s) po:l_ookahead n` | `((a, b, c), (d, b, c, a, a)) -> Int -> e` | the expert's next n ops |
| `T po:c_hoose la` | `a -> a -> Int` | the longest tool that is the start of la (row from 1), or 0 |
| `task po:r_un (s, seq)` | `(a, b, c) -> ((...), f) -> ((...), f, g)` | run seq until an op fails: (state, ops, oks) |
| `(task, T, id, ver, call) po:e_pisode budget` | `(...) -> Int -> ((...), journal)` | one task, at most budget calls: (state, journal) |
| `(tasks, T, ver, call) po:w_orkload budget` | `(a, Int, Int, Int) -> Int -> (Int, journal)` | each row (q, w, r) of tasks in turn: (tasks done, journal) |
| `(task, T, id, ver, call, extra, every) po:e_pisodeWith budget` | `(...) -> Int -> ((...), journal)` | as e_pisode, each call costing call + extra, the table ignored on every every-th call (0: never) |
| `(tasks, T, ver, call, extra, every) po:w_orkloadWith budget` | `(...) -> Int -> (Int, journal)` | as w_orkload, by e_pisodeWith |
| `(lo, hi, k, L) po:s_ynthesize j` | `(Int, Int, Int, Int) -> journal -> Int` | the k sequences of lo to hi ops in j worth most calls saved, as a tool table of rows of L ops |
| `po:c_alls j` | `journal -> Int` | the model calls in a journal |

## Examples

From `../demos/baseline.xtl`, six tasks, 500 tokens a call:

| Harness | Tasks done | Ops | Calls | Tokens |
| ------- | ---------- | --- | ----- | ------ |
| fixed (no tools) | 6 | 120 | 120 | 60000 |
| the make-a-device route as a tool | 6 | 120 | 55 | 27500 |

`../tests/checks.xtl` checks that tools change the calls and never the
outcome or the ops done, that the saving is 5 calls per device made
with the tool, that a tool that never fits changes nothing, and that
the budget stops an episode.

## Demos

- [`demos/baseline.xtl`](../demos/baseline.xtl): the fixed harness and
  the same harness with one tool.

## Provenance

Written for X_eTaL-RSI (plan A1: policy is data, mechanism is code).
