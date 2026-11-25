# Asks for X_eTaL

Features the RSI experiments and libraries need that X_eTaL does not
have yet, and bugs they uncovered. This repo does not change X_eTaL:
each ask is filed here (and taken to `../X_eTaL`), the experiment or
library uses the workaround noted below or waits, and the workaround
is removed when the ask lands in the X_eTaL commit this repo pins
(`XETAL_COMMIT`).

Each entry: status, kind (feature, bug or speed), which experiments or
libraries need it, why, a minimal repro or example, and the workaround
in use. The status is X_eTaL's own word for it, from its ledger
(`docs/asks.toml` in X_eTaL, read from the pinned copy in
`work/xetal`): **landed**, **partly landed**, **planned**,
**declined**, **new** (taken to X_eTaL, not yet triaged), or here only
**not yet taken** (filed here, still to be added to X_eTaL's ledger).
`scripts/check-asks.py` (in the gate) keeps this table and the ledger
in step. Ids are R1, R2, ...

| # | Status | Kind | Ask | Experiments, libraries | Workaround |
| - | ------ | ---- | --- | ---------------------- | ---------- |

None yet.

## Decided, not asked

- Run-time `execute` (APL's execute, evaluating program text while a
  program runs): left out of X_eTaL on purpose (its lang-choices B29,
  static types). This repository does not ask for it: policies are
  data interpreted by fixed library code (plan A1), and new X_eTaL
  text written by a model is checked outside the program by the pinned
  CLI (plan A2).
