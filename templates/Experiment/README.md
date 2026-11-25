# __NUM__ __TITLE__

__SUMMARY__

A deterministic simulation: it shows the mechanics of
self-improvement on a scripted workload; it is not evidence that a
model learns.

## Question

What the experiment asks, and the hypothesis it tests (docs/plan.md).

## Groups

| Group | Memory | Tools | Harness | Optimizer |
| ----- | ------ | ----- | ------- | --------- |
| A baseline | no | no | no | no |

## Run

```bash
just run-exp __SLUG__        # the experiment's programs
just test-exp __SLUG__       # check its baselines
```

## Results

What the pinned run (`reg/`) shows.

## Limits

What it does not show.
