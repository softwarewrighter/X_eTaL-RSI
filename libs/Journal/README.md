# Journal

the append-only event history of an agent as columns, and the action sequences that repeat in it

```
"jr:" u_se< "Journal"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Journal.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo-lib Journal     # run its demos
just run-lib Journal      # run its test programs
just test-lib Journal     # check every baseline
```
