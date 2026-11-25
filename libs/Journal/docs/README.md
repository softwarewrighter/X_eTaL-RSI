# Journal

The append-only event history of an agent as columns, and the action
sequences that repeat in it.

```
"jr:" u_se< "Journal"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `jr:` is the
recommended alias.

A journal is five Int columns of one length, as a tuple
`(task, version, op, cost, ok)`: per event in order, its task, the
policy version that acted, the operation (an op code from 1 to 99),
its cost and whether it succeeded (1) or failed (0). It only grows:
`jr:r_ecord` gives a new journal with the new events after the old.
The questions an optimizer asks of a history (which op sequences
repeat, what they cost, which fail) are array expressions over the
columns, answered without reading the log into a prompt.

An n-gram is n consecutive ops of one task, numbered as one key: the
op codes as base-100 digits (`3 4 5` is 30405). `jr:k_eys` gives the
key starting at each event, 0 where the n events would run past the
end or into another task.

## Functions

| Function | Type | What |
| -------- | ---- | ---- |
| `jr:e_mpty @` | `Unit -> (a, a, a, a, a)` | a journal with no events |
| `j jr:r_ecord e` | `(a, b, c, d, e) -> (a, b, c, d, e) -> ...` | j with the events e after its own |
| `jr:l_ength j` | `(...) -> Int` | the number of events |
| `jr:t_otal j` | `(...) -> d` | the cost of all events |
| `ver jr:v_ersion j` | `a -> (b, a, c, d, e) -> ...` | the events of one policy version |
| `n jr:k_eys j` | `Int -> (...) -> Int` | the n-gram key starting at each event (0: none) |
| `n jr:s_equences j` | `Int -> (...) -> (Int, Int, Int, Int)` | each distinct n-gram: (key, count, cost, failed), most calls saved first |
| `n jr:d_ecode keys` | `Int -> Int -> Int` | the op codes of keys, a row each |

`jr:s_equences` orders the n-grams by `count * (n - 1)`: the model
calls a tool for the sequence would save, if each op otherwise costs
one call and the tool one.

## Examples

From `../tests/basics.xtl` (three tasks of ops 1 2 3 4, 1 2 3 4 and
1 2 3; task 2's last event fails):

```
      3 jr:k_eys j
10203 20304 0 0 10203 20304 0 0 10203 0 0
      (key, count, cost, failed) := 3 jr:s_equences j
      key
10203 20304
      count
3 2
      failed
0 1
```

`../tests/checks.xtl` checks every count against a brute-force count,
that the counts add up to the windows inside tasks, and that no
n-gram crosses a task.

## Demos

- [`demos/repeats.xtl`](../demos/repeats.xtl): eight manufacturing
  tasks, every action a model call; the most valuable 4-op sequences,
  and the whole 8-op route occurring 8 times.

## Provenance

Written for X_eTaL-RSI. The event columns follow the trajectories of
agentrail-rs (state, action, result, reward), kept as arrays.
