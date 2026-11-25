# Guardian

the layered guardian: checks a candidate policy must pass before it replaces the running one, the version history and rollback

```
"gd:" u_se< "Guardian"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Guardian.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo-lib Guardian     # run its demos
just run-lib Guardian      # run its test programs
just test-lib Guardian     # check every baseline
```
