# Econ

The economics of self-improvement: what an improvement costs, when it
pays, and which to buy with a budget.

```
"ec:" u_se< "Econ"
```

Put each library's `src/` directory on `XETAL_PATH` (`just path`
prints them; see the [README](../../../README.md)); `ec:` is the
recommended alias.

An improvement (a tool, a procedure, a better policy) costs an
investment `i` once and lowers each later task of its kind from `b`
(the baseline) to `a`. Over `c` tasks that is `c * b` without it and
`i + c * a` with it, so it pays only past the break-even `i / (b - a)`.
Deciding when to improve without knowing `c` is the ski-rental
problem: waiting until the break-even's worth of baseline tasks is
done never costs more than twice the best choice made in hindsight.
Costs are in any one unit (tokens, model calls), and every count may
be an array: over `r_ange n` a cost is the cumulative curve.

## Functions

| Function | Type | What |
| -------- | ---- | ---- |
| `(i, b, a, k) ec:a_fter c` | `(a, a, a, a) -> a -> a` | the cost of c tasks improving after the k-th (k = 0 at once, negative never) |
| `(i, b, a) ec:f_ixed c` | `(a, b, c) -> b -> b` | never improving: c * b |
| `(i, b, a) ec:e_ager c` | `(a, a, a) -> a -> a` | improving at once: i + c * a |
| `(i, b, a) ec:h_indsight c` | `(a, a, a) -> a -> a` | the least cost knowing c: the cheaper of the two |
| `ec:b_reakEven (i, b, a)` | `(a, a, a) -> Float` | i / (b - a); -1.0 when it never pays |
| `ec:s_kiRental (i, b, a)` | `(a, a, a) -> Int` | the break-even rounded up: improve after that many tasks; -1 never |
| `(i, b, a, k) ec:r_atio c` | `(a, a, a, a) -> a -> Float` | the cost of the rule over the hindsight cost |
| `m ec:a_llocate (i, s, n)` | `a -> (a, a, a) -> b` | which improvements a budget m buys (a mask), by return on investment |
| `(i, s, n) ec:n_et mask` | `(a, a, a) -> a -> a` | what the bought improvements save over their uses, less their cost |

## Examples

From `../tests/basics.xtl` (i = 3000, b = 1000, a = 450):

```
      s ec:h_indsight 0 1 5 6 7 10 40
0 1000 5000 5700 6150 7500 21000
      ec:s_kiRental s
6
      (3000, 1000, 450, 6) ec:a_fter 0 1 5 6 7 10 40
0 1000 5000 6000 9450 10800 24300
```

The properties (hindsight is never beaten, ski rental is within twice
it, the budget is never exceeded) are checked in `../tests/checks.xtl`.

## Demos

- [`demos/breakeven.xtl`](../demos/breakeven.xtl): the research's
  example, 3000 tokens to save 55% of a 1000-token task: break-even
  after 5.45 tasks.

## Provenance

Written for X_eTaL-RSI. The ski-rental bound is the classic result for
the rent-or-buy problem (Karlin, Manasse, Rudolph and Sleator, 1988).
