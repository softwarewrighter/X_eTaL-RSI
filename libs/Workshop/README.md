# Workshop

a deterministic resource and manufacturing world: places, gathering, making and delivering, as op codes

```
"ws:" u_se< "Workshop"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Workshop.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo-lib Workshop     # run its demos
just run-lib Workshop      # run its test programs
just test-lib Workshop     # check every baseline
```
