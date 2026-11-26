# 06 Transfer

A harness evolved on one family of tasks, run on near and far ones,
frozen and still evolving.

A deterministic simulation on the [Workshop](../../libs/Workshop/docs/README.md)
world (the model is Workshop's expert); it shows the mechanics of
carrying learned tools to new work, not that a model would.

## Question

In Exo's game world, harnesses evolved on one set of tasks ran
transfer tasks for about a quarter of a baseline agent's cost
(Krentsel et al., [arXiv 2611.00463](https://arxiv.org/abs/2611.00463)).
Do tools learned on one family of tasks help on others, near and far,
and is a learned harness a better start than none (hypothesis H2:
learned policies transfer)?

## Method

The source: a harness evolves for 8 batches of 3 tasks, devices of 1
wood and 1 ore, with [Evolve](../../libs/Evolve/docs/README.md) and
the optimizer `2 8 1 3 1` (one tool of 2 to 8 ops from the last 3
tasks, revised every batch: the best fixed optimizer of experiment 04),
each candidate reviewed by all six [Guardian](../../libs/Guardian/docs/README.md)
layers. It ends with one tool, the whole route to make a device:
go-forest, gather-wood, go-mine, gather-ore, go-workshop, make. A
second source evolves with `2 3 3 3 1` (three tools of at most 3 ops)
and ends with go-forest, gather-wood, go-mine; gather-wood, go-mine,
gather-ore; go-mine, gather-ore, go-workshop.

Each target family is a fresh stream of 4 batches of 3 tasks (2 to 4
devices a task):

| Family | Devices of | Kind |
| ------ | ---------- | ---- |
| same | 1 wood, 1 ore | the source's (control) |
| 2 wood | 2 wood, 1 ore | near (2 ore is the same, by the world's symmetry) |
| 2 of each | 2 wood, 2 ore | near |
| wood only | 1 wood, no ore | far: a different route (ore only is the same) |

Costs: a call 500 tokens, a tool's schema 20 in every prompt, writing a
tool 1500.

## Groups

| Group | Starts with | Revised |
| ----- | ----------- | ------- |
| A | no tools | never |
| frozen | the source's route | never |
| continued | the source's route | every batch, as the source was |
| fresh | no tools | every batch |
| frozen-short | the short source's three tools | never |

## Run

```bash
just run-exp 06        # the experiment and its checked claims
just show-exp 06       # each statement, then its output
just test-exp 06       # check its baseline
```

## Results

All tokens on each family (production, writing and review):

| Family | A | frozen | continued | fresh | frozen-short |
| ------ | - | ------ | --------- | ----- | ------------ |
| same | 102000 | 28080 | 28080 | 90300 | 80640 |
| 2 wood | 117000 | 121680 | 97180 | 95260 | 97440 |
| 2 of each | 132000 | 137280 | 109660 | 107500 | 114240 |
| wood only | 72000 | 74880 | 87820 | 86620 | 80640 |

Every group did all 12 tasks on every family. Over the 8 source
batches the fixed harness spent 204000 tokens, the source 118380
(production 70920, writing 1500, review 45960) and the short source
226820.

What it shows:

- A specific tool pays only where it was learned. On its own recipe
  the route saves 72%; on every other family it never fits and its
  schema makes each call dearer, so frozen costs 4% more than no tools.
- General tools transfer to near work. The three short tools save 21%
  on their own recipe (far less than the route) but still 17% with 2
  wood and 13% with 2 of each, within 5% of a harness that keeps
  evolving there, with no learning and no reviews.
- Specific against general is the trade: the tool worth most on the
  source is worth nothing off it. Experiment 01 found the same on
  held-out tasks.
- A learned harness is no head start here. Off the source's recipe,
  continuing from its route never beat evolving from scratch: the route
  has to be replaced either way, and carrying it costs a little.
- Nothing transferred to the far family. Wood-only devices take a
  route the source never saw; every group cost more than the fixed
  harness, and four batches were too few for evolving there to repay
  its reviews (its production tokens did halve).
- H2 holds for general tools on near families, not for specific tools
  and not for far families.

## Limits

- One world: near and far are recipes of the same op set; a truly
  unrelated family (another world) would share nothing at all.
- Tools here are sequences of ops; parameterized tools (make n devices
  of any recipe) would transfer further, and the op-code policy of
  plan A1 cannot express them yet.
- Four batches a family: with longer work on a far family, evolving
  there would repay its reviews (experiment 03).
