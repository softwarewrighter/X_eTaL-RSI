# Econ

the economics of self-improvement: what an improvement costs, when it pays, and which to buy with a budget

```
"ec:" u_se< "Econ"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Econ.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo-lib Econ     # run its demos
just run-lib Econ      # run its test programs
just test-lib Econ     # check every baseline
```
