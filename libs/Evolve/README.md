# Evolve

The improvement loop as protected mechanism: an optimizer vector that
learns, reviews and swaps a harness's tools as the work goes on.

```
"ev:" u_se< "Evolve"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Evolve.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo-lib Evolve     # run its demos
just run-lib Evolve      # run its test programs
just test-lib Evolve     # check every baseline
```
