# 02 Context optimization

Which tools stay in the prompt: a library that retires idle tools,
against replacing the table and keeping everything.

A deterministic simulation on the [Workshop](../../libs/Workshop/docs/README.md)
world (the model is Workshop's expert); it shows the mechanics of
managing a harness's context, not that a model would.

## Question

Every tool a harness keeps costs context in every prompt, while a tool
dropped may have to be learned again. Exo's harnesses manage their own
context and memory (Krentsel et al.,
[arXiv 2611.00463](https://arxiv.org/abs/2611.00463)). Is a rule that
retires idle tools worth it, and how does the answer depend on what a
tool costs in the prompt?

## Method

Twenty-four batches of three tasks; recipes come back: 1 wood and 1
ore a device, 2 wood, 1-1 again, 2 wood again, 2 ore, 1-1 again, four
batches each. Costs: a call 500 tokens, writing a tool 1500 (only tools
not already running), and a tool's schema 20 tokens in every prompt,
or 200 (closer to a real tool's description).

Every group evolves with [Evolve](../../libs/Evolve/docs/README.md) and
the same optimizer: one tool of 2 to 8 ops learned from the last 3
tasks before every batch (experiment 04's best), each candidate
reviewed by all six [Guardian](../../libs/Guardian/docs/README.md)
layers (at most 6 tools). They differ in the optimizer's sixth number,
`idle`, what stays in the prompt: the tool just learned is added to
the running tools used (Policy's `u_ses`) in the last `idle` tasks.

## Groups

| Group | idle | In the prompt |
| ----- | ---- | ------------- |
| A | | no tools, never revised |
| replace | 0 | only the tool just learned |
| keep-all | 72 | every tool used at all (up to 6) |
| retire-3, -6, -12 | 3, 6, 12 | the tools used in the last 3, 6 or 12 tasks |

## Run

```bash
just run-exp 02        # the experiment and its checked claims
just show-exp 02       # each statement, then its output
just test-exp 02       # check its baseline
```

## Results

A tool's schema 20 tokens a prompt:

| Group | Production | Writing | Review | All | Accepted | Tools in prompt (sum) |
| ----- | ---------- | ------- | ------ | --- | -------- | --------------------- |
| A | 657000 | 0 | 0 | 657000 | 0 | 0 |
| replace | 270600 | 9000 | 286200 | 565800 | 6 | 23 |
| keep-all | 228480 | 4500 | 303720 | 536700 | 3 | 49 |
| retire-3 | 270600 | 9000 | 286200 | 565800 | 6 | 23 |
| retire-6 | 272100 | 9000 | 430620 | 711720 | 11 | 28 |
| retire-12 | 274800 | 9000 | 396000 | 679800 | 10 | 38 |

200 tokens a prompt:

| Group | Production | Writing | Review | All |
| ----- | ---------- | ------- | ------ | --- |
| A | 657000 | 0 | 0 | 657000 |
| replace | 357000 | 9000 | 378000 | 744000 |
| keep-all | 367800 | 4500 | 499200 | 871500 |
| retire-3 | 357000 | 9000 | 378000 | 744000 |
| retire-6 | 372000 | 9000 | 616200 | 997200 |
| retire-12 | 399000 | 9000 | 567000 | 975000 |

Every group did all 72 tasks at both prices.

What it shows:

- With cheap schemas, keep everything. Keeping every tool learned each
  route once (3 revisions accepted against 6), so the work spent least
  in production and the fewest tokens overall, though its prompt held
  up to 3 tools where the others held 1 or 2.
- With dear schemas, keep one. At 200 tokens a tool, replacing the
  table spends least of the evolving groups; the library's extra tools
  now cost more than relearning them, in production and in every
  review (a review runs the candidate with its schemas too).
- Retiring idle tools was the worst rule at both prices. Retiring a
  tool is itself a revision, reviewed at full cost, and a recipe that
  comes back means learning the tool again: retire-6 and retire-12
  accepted 10 and 11 revisions and spent the most. Retiring after 3
  tasks drops a tool as soon as a batch leaves it unused, which here is
  the same as replacing the table.
- What to keep in context is a setting whose best value depends on the
  price of context and on whether work recurs; it belongs to the
  optimizer, which experiment 04 shows can be tuned, at a cost.
- At 200 tokens a tool, no evolving group beat the fixed harness at
  this scale: reviews pay the price of context too.

## Limits

- One recurring pattern; with work that never comes back, retiring
  would lose less, and with more tools than the cap, keeping all would
  have to choose.
- Retiring is judged by the Guardian like any revision; a mechanism
  that dropped idle tools without a review would make retiring cheap
  (and unguarded).
- Context here is only tool schemas; the research's experiment 02 also
  filters and compresses memories and history, which this world does
  not have.
