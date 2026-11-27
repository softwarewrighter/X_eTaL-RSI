# Related work: this repository beside earlier ways of improving agents

Exo, and this repository's experiments on it, are one more way for an
agent to get better without retraining. This note sets them beside the
techniques and tools the author has studied and built before: in-context
learning (ICL), in-context reinforcement learning (ICRL), XSkill,
Recursive Language Models (RLM), the Ralph loop, agentrail-rs,
all-together-now, the continuous-learning architecture of the "How AI
Learns" series, and the local-model plan-execute-review loop. Blog posts
are on [Software Wrighter Lab](https://blog.softwarewrighter.com/).

## Six questions

Each approach is placed by six questions:

1. What changes: weights, the context, a memory, code, or the process?
2. Where is what was learned kept?
3. Does it persist past the session?
4. Who or what checks it before it is used?
5. Does the procedure that improves things itself improve (the
   research's level 3)?
6. What this repository takes from it, or tests about it.

| Approach | What changes | Kept in | Persists | Checked by | Improver improves |
| -------- | ------------ | ------- | -------- | ---------- | ----------------- |
| ICL | the prompt's examples | the context | no | nothing | no |
| ICRL | (state, action, reward) traces in the prompt | the context, from a store | if stored | the reward | no |
| RLM | what part of the data is in view | a workspace outside the prompt | no | a sandbox for its code | no |
| XSkill | skills (plans) and experiences (tips), as text | a memory, retrieved | yes | nothing explicit | no |
| Ralph loop | the code base, one fresh-context pass at a time | files and git | yes | tests, and the human who edits the prompt | the human does |
| agentrail-rs | trajectories and distilled skills; the saga | `.agentrail/`, domain repos | yes | validation steps, audit | no (fixed engine) |
| all-together-now | nothing learned: coordination | a wiki, an event timeline | yes | compare-and-swap | no |
| Continuous-learning agent | adapters (and memory) | adapter files, a memory | yes | retention, transfer, regression gates | no (human-designed loop) |
| SIM | the optimizer's record of attempts, by problem | a JSON record per run, carried | yes | the selector, on evolve-set feedback only | no (fixed search; the record informs it) |
| Exo | the harness: its code, tools, memory, prompts | versioned artifacts | yes | a guardian (by default: it builds) | yes |
| This repository | a policy as an array program, and the optimizer's own parameters | versioned arrays, an append-only journal | yes | a layered guardian (plan A8) | yes (group E) |

## The time scales, and where Exo sits

"How AI Learns" (Part 1, [Four Time Scales of
Learning](https://blog.softwarewrighter.com/2026/02/24/how-ai-learns-part1-time-scales/);
Part 7, [Designing a Continuous Learning
Agent](https://blog.softwarewrighter.com/2026/03/02/how-ai-learns-part7-full-architecture/))
puts learning in layers of permanence: core weights (months or never),
adapters (weekly to monthly), external memory (continuous) and the
context (per call). None of those layers is the harness itself: its
code, its tools and the policy that decides how it works are, in that
picture, fixed and designed by people.

Exo adds that layer. What it learns is executable and versioned like
code, persistent like memory, cheap to change like context, and
checked before use like an adapter. The research's three levels (learn
from experience; improve how work is done; improve how improvement is
done) cut across the time scales: ICL, ICRL and XSkill are level 1,
tool synthesis and Exo's harness edits are level 2, and level 3 is
what none of the earlier systems does on its own.

## Each in turn

### ICL

Examples in the prompt change what the model does for one call
([In-Context Learning
Revisited](https://blog.softwarewrighter.com/2026/02/22/icl-revisited-from-mystery-to-engineering/)).
It is "working memory, not learning" (Part 5): nothing persists, and
appending everything rots the context. In Exo, ICL is how the frozen
model works inside one step; it is not what improves. X_eTaL-ML's
in-context-learning saga takes the mechanism apart (a hand-built
circuit, arXiv 2611.05344); this repository does not repeat that.

### ICRL

A transformer infers a policy from (state, action, reward) traces in
its prompt ([ML Frontier #02: In-Context Reinforcement
Learning](https://blog.softwarewrighter.com/2026/03/16/mlf-02-icrl-in-context-reinforcement-learning/);
Decision Transformer, arXiv 2106.01345; Reflexion, 2303.11366;
Voyager, 2305.16291). agentrail-rs applies it: successful trajectories
are injected into the next step's prompt. Its limit, in the post's
words: "Agents ignore rules even when they're present."

Here: group B (memory) is the ICRL baseline: the journal's successful
action sequences replayed as hints, which the policy may or may not
follow. Group C makes the same sequences into tools that run as
written. The difference between B and C measures how much is lost when
a lesson is advice rather than code.

### RLM

Recursive Language Models (Zhang, Kraska and Khattab, arXiv
2512.24601; [RLM: Recursive Language Models for Massive
Context](https://blog.softwarewrighter.com/2026/02/13/rlm-recursive-language-models/))
"expand the workspace, not the context": the data stays outside the
prompt and the model looks at it through tools (slice, find, regex,
count, a sub-model call), recursively. The author's Rust version has
four levels, "each level trades off safety for capability": built-in
commands, model-written Rust run as WASM, native binaries, and
delegation to sub-models. Nothing it writes is kept; the WASM sandbox
is the only check.

Here: RLM's level 1, a fixed command language the model composes, is
plan decision A1 (a policy is a program over a fixed op-code set). Its
levels map to the guardian's layers: the cheaper and safer the
representation, the less checking a change needs. The journal library
answers questions about the execution history as array expressions
rather than by reading the log into a prompt, which is RLM's idea
applied to an agent's own traces. Experiment 02 (context optimization)
is closest to RLM's context rebuilding. One difference matters: RLM's
code is thrown away after the query, Exo's is kept and reused, which is
why Exo needs a guardian and RLM a sandbox.

Note: Parts 2, 5 and 7 of "How AI Learns" cite RLM as arXiv 2412.04619
(Zhou et al. 2024); the RLM post cites 2512.24601 (Zhang, Kraska,
Khattab). This note follows the RLM post.

### XSkill

Two persistent memories, skills (workflow documents) and experiences
(tactical tips with triggers and failures), built from batches of
rollouts and retrieved into the system prompt; "not parameter updates"
([AI Tools #1:
XSkill](https://blog.softwarewrighter.com/2026/03/17/ai-tools-xskill-memory-layer-multimodal-agents/);
arXiv 2603.12056). It reports syntax errors down from 20.3% to 11.4%
and tool-name errors from 2.85% to 0.32%, and an ablation showing both
memories are needed.

Here: experiment 01 (tool synthesis) is XSkill's accumulation phase
with the skill compiled into code: a repeated sequence becomes a tool
that runs, not a document a model reads. XSkill has no gate between
accumulating a skill and using it; experiment 07 tests one.

### SIM (self-improving memory)

Google's SIM (Yu, Xia, Pan et al., "Self-Improvement Memory Enables
Continual Recursive Self-Improvement",
[arXiv 2611.18856](https://arxiv.org/abs/2611.18856)) gives a harness
optimizer a memory of its own work: each edit is filed under the
recurring problem it addressed (a root-cause hypothesis such as
"premature completion without execution verification"), with what it
changed, the tasks it was predicted to fix (written before evaluation),
those it fixed and missed, and the setting it was evaluated in. The
whole record is read every round and never pruned; problems with the
same text are merged. Carried into a new benchmark or a new policy
model, it beats carrying the harness alone (2.36 points on Terminal
Bench over ten paired runs; 3.60 to 11.33 points across models). The
optimizer's search stays fixed: the memory makes its choices better,
not its code. Its own negatives: two selected edits fixed none of
their predicted tasks, and a stale entry kept steering after the
harness changed.

Here: Saga 5 (plan) makes the record data, as the journal is: a
Memory library read by Evolve, and experiment 11 with the paper's
arms. Unlike the paper, every read of the record is charged, so its
growth has a price, and what the record saves is measured in reviews
not rerun, the dominant cost of experiments 03 and 02.

### The Ralph loop

Geoffrey Huntley's [Ralph](https://ghuntley.com/ralph/): run a coding
agent in a shell loop on the same prompt, `while :; do cat PROMPT.md |
agent; done`. Each pass starts from a fresh context. The state is the
files and git history, so progress persists even though each run does
not remember the last. When the same mistake recurs, the person edits
the prompt. The author demonstrates a variant for sequential and
parallel Claude Code tasks in
[try-ralphy](https://github.com/softwarewrighter/try-ralphy).

Here: the Ralph loop is Exo with the human as the optimizer. The prompt
is the policy, editing it after a failure is a policy revision, and
the tests are the only guardian. Exo makes that edit part of the run,
and level 3 asks whether the loop can learn when and how to edit
itself. Group A (fixed harness, fresh context every task) is Ralph
with the prompt left as it is.

### agentrail-rs

The saga process this repository is developed with
([agentrail-rs](https://github.com/sw-vibe-coding/agentrail-rs);
[Saw #2](https://blog.softwarewrighter.com/2026/03/15/saw-reg-rs-avoid-compaction-agentrail/),
[Saw #3](https://blog.softwarewrighter.com/2026/03/22/saw-agentrail-rs-icrl-dual-memory/),
[AI Tools #2](https://blog.softwarewrighter.com/2026/04/23/ai-tools-agentrail-rs-mid-saga-flexibility/)).
It records each step's trajectory (state, action, result, reward),
injects successful ones into the next session (ICRL), distills batches
of them into XSkill-style skills (`agentrail distill`), and runs
deterministic and validation steps without the model ("validation
steps create the reward signals"). Its problem statement is Exo's: an
agent that is "not forgetting, but *improvising*" on procedures it has
done before.

It already has much of Exo's protected layer: an append-only saga
("the past is immutable; the future is negotiable"), `audit` to
reconcile the saga with git, `snapshot` into the git object store, and
domain repos of skills, executors and validators. What it lacks, by
design, is self-modification: the engine, the reward and the distill
procedure are fixed by people.

Here: the Journal library is agentrail's trajectory store as columns.
A later experiment could make agentrail's own policy (which
trajectories to inject, when to distill) the editable optimizer of
group E. This repository's own development, recorded in `.agentrail/`,
is a real log of a person and an agent improving a process step by
step.

### all-together-now

A program manager for cooperating agents
([all-together-now](https://github.com/sw-vibe-coding/all-together-now);
[Saw #4](https://blog.softwarewrighter.com/2026/03/29/saw-pjmai-reg-emacs-all-together-now/),
[Saw #6](https://blog.softwarewrighter.com/2026/04/12/saw-agent-coordination-fuzzit-vendoring/)):
it owns the agents' terminals, streams them to a dashboard, and keeps a
wiki with ETag compare-and-swap and an append-only event timeline. It
does not learn. Its value here is as mechanism: the event timeline is
the kind of history Exo protects, and running several executors at once
is how a guardian would shadow-run a candidate beside the incumbent
(plan A8, experiment 07).

### The continuous-learning agent

[Part 6](https://blog.softwarewrighter.com/2026/03/01/how-ai-learns-part6-continuous-learning/)
and Part 7 of "How AI Learns" design a loop of collecting traces,
scoring them, clustering failures, training an adapter, validating
retention and transfer, and deploying "with rollback capability":
"periodic consolidation", not real-time learning. "Continuous learning
without continuous evaluation is dangerous." Sleepy Coder (Saw #3)
found LoRA fine-tuning did worse than the base model with good
prompts, which argues for improving the harness, not the weights.

Here: the same loop with a different artifact. Exo's revisions are
code, not adapters, so they can be read and diffed. The gates the
series asks for (retention tests, forward transfer, regression
detection, rollback) are the guardian's layers in plan A8.

### The plan-execute-review loop with local models

A Rust harness around stateless model calls, plan then execute then
review, where "Rust owns the cursor + history", inspired by RLM
([AI Tools #6: Evaluating Local Models in a Plan-Execute-Review
Loop](https://blog.softwarewrighter.com/2026/09/09/ai-tools-local-llm-loop-model-evaluation/)).
It ranked models by speed only (gpt-oss-20b fastest at 1m13s on a 5060
Ti; some 27B models failed to plan at all) and says "a prompt or
tool-policy change could move any of these results".

Here: that harness is the natural host for Saga 3 (models), and its
note about prompt and tool-policy changes is exactly what experiment 10
measures. Exo found that small models gained little from adapting their
harness; experiment 10 asks whether constrained X_eTaL policies lower
that threshold.

## What this repository adds

- The harness and the procedure that improves it are both data, as
  small typed array programs, so level 3 can be tested directly:
  group D against group E.
- Every improvement is charged: experiment 05 shows how often
  improving is a bad bet, which none of the systems above measures.
- Every lesson can be code rather than advice (group C against B), so
  "agents ignore rules" can be measured, not just observed.
- A guardian whose criteria the policy cannot edit, with rollback and
  an intact history, tested against defective revisions on purpose.
