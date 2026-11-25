# Workshop

A deterministic resource and manufacturing world: places, gathering,
making and delivering, as op codes.

```
"ws:" u_se< "Workshop"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `ws:` is the
recommended alias.

The world of the experiments on tools and policies (the research's
"resource-gathering and manufacturing game", far simpler than the
RuneScape-like world of the Exo paper). Four places (1 forest, 2 mine,
3 workshop, 4 store); a state is `(pos, wood, ore, made, delivered)`;
a task `(q, w, r)` is to deliver q devices, each made from w wood and r
ore. Nine op codes:

| Op | Does | Precondition |
| -- | ---- | ------------ |
| 1 | go to the forest | none |
| 2 | gather wood | at the forest |
| 3 | go to the mine | none |
| 4 | gather ore | at the mine |
| 5 | go to the workshop | none |
| 6 | make a device (uses w wood, r ore) | at the workshop, enough of both |
| 7 | go to the store | none |
| 8 | deliver what was made | at the store, something made |
| 9 | look | none (changes nothing) |

An op whose precondition fails changes nothing and reports 0. The
world is pure, so every episode is reproducible.

## Functions

| Function | Type | What |
| -------- | ---- | ---- |
| `ws:s_tart @` | `Unit -> (a, b, c, d, e)` | at the store with nothing |
| `task ws:s_tep (state, op)` | `(a, b, c) -> ((d, b, c, e, e), f) -> ((d, b, c, e, e), g)` | the next state and 1, or the same state and 0 |
| `task ws:d_one state` | `(a, b, c) -> (d, e, f, g, a) -> h` | 1 once q devices are delivered |
| `task ws:e_xpert state` | `(a, b, c) -> (d, b, c, a, a) -> e` | the op a competent agent does next (0 when done) |
| `ws:n_ame op` | `a -> Char` | the op's name |

`ws:e_xpert` stands in for a model's choice of action: deterministic and
always right, so the experiments measure what the harness does with the
calls, not how good the model is. It makes the devices one at a time,
w + r + 4 ops each, then goes to the store and delivers all of them.

## Examples

From `../tests/basics.xtl`:

```
      m_ix '{ op -> (_, ok) := task ws:s_tep (s0, op); ok } m_ap 1 2 3 4 5 6 7 8 9
1 0 1 0 1 0 1 0 1
      (1, 2, 1) ws:e_xpert (1, 1, 0, 0, 0)
2
```

`../tests/checks.xtl` checks that the expert finishes tasks of several
sizes and recipes in exactly q (w + r + 4) + 2 ops with no failure.

## Demos

- [`demos/route.xtl`](../demos/route.xtl): three devices, every op the
  expert takes.

## Provenance

Written for X_eTaL-RSI.
