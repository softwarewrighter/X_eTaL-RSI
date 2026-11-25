# Policy

policies as data: a harness's tools as an op-code table, run by an interpreter the policy cannot change

```
"po:" u_se< "Policy"
```

| Directory | What |
| --------- | ---- |
| [`src/`](src/) | the library, `Policy.xtl` |
| [`docs/`](docs/README.md) | the reference: every function, its type, examples, provenance |
| [`demos/`](demos/) | programs that use it |
| [`tests/`](tests/) | reg-rs baselines: the test programs, the pinned export types (`types.rgt`) and the demos (`demo-*.rgt`) |

```bash
just demo-lib Policy     # run its demos
just run-lib Policy      # run its test programs
just test-lib Policy     # check every baseline
```
