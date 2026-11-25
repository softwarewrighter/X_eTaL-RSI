//! The page's model: the settings, the X_eTaL program written from them
//! (which the page shows, decorated, and runs in the browser), and what
//! it prints read back.

use microscope::run;

/// What the reader sets: tools are learned of 2 to `longest` ops, the
/// `count` worth the most calls; a hint costs `hint` tokens in every
/// prompt and a model given hints ignores them every `ignore`-th call
/// (0: never).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub longest: i64,
    pub count: i64,
    pub hint: i64,
    pub ignore: i64,
}

/// The command-line experiment's settings.
pub const DEFAULT: Params = Params { longest: 8, count: 3, hint: 60, ignore: 4 };

/// The groups, in the order the program prints them.
pub const GROUPS: [&str; 3] = ["A fixed harness", "B hints in the prompt", "C tools"];

/// Workshop's op names, by code.
pub const OPS: [&str; 9] = ["go-forest", "gather-wood", "go-mine", "gather-ore", "go-workshop", "make", "go-store", "deliver", "look"];

/// One group on one held-out set.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Row {
    pub done: i64,
    pub calls: i64,
    pub tokens: i64,
}

/// What the program printed.
#[derive(Clone, Debug, PartialEq)]
pub struct Results {
    pub train_calls: i64,
    pub train_tokens: i64,
    /// Each learned tool's op codes (padding dropped).
    pub tools: Vec<Vec<i64>>,
    /// Groups A, B, C on the same-recipe and the shifted held-out tasks.
    pub same: Vec<Row>,
    pub shift: Vec<Row>,
    /// Tokens B and C saved against A on all held-out tasks, after paying to learn.
    pub net: Vec<i64>,
}

/// The program for `p`: what the page runs and shows.
pub fn program(p: &Params) -> String {
    let (hi, k, hint, every) = (p.longest, p.count, p.hint, p.ignore);
    format!(
        r#"## Experiment 01 on this page: tools learned from a fixed harness's
## history, then three harnesses on held-out tasks. A deterministic
## simulation on the Workshop world; a model call costs 500 tokens.
"po:" u_se< "Policy"
"jr:" u_se< "Journal"
## Training: the fixed harness does 8 tasks (devices of 1 wood, 1 ore).
train := 8 3 r_eshape 1 1 1 3 1 1 2 1 1 5 1 1 1 1 1 4 1 1 2 1 1 3 1 1
(dt, jt) := (train, po:n_one 8, 0, 500) po:w_orkload 200
(po:c_alls jt) c_at jr:t_otal jt
## Learning: the {k} sequences of 2 to {hi} ops worth most calls saved.
tools := (2, {hi}, {k}, 8) po:s_ynthesize jt
t_ally tools
r_avel tools
## Held out: the same recipe, then 2 wood or 2 ore.
same := 4 3 r_eshape 2 1 1 4 1 1 6 1 1 3 1 1
shift := 4 3 r_eshape 2 2 1 3 1 2 2 2 2 4 1 3
## A tool's schema costs 20 tokens a prompt, a hint {hint}; hints are
## ignored every {every}th call (0: never).
u:r_esult := {{ (tasks, T, extra, every) ->
  (d, j) := (tasks, T, 1, 500, extra, every) po:w_orkloadWith 200
  d c_at (po:c_alls j) c_at jr:t_otal j
}}
u:g_roups := {{ tasks ->
  a := u:r_esult (tasks, po:n_one 8, 0, 0)
  b := u:r_esult (tasks, tools, {hint} * t_ally tools, {every})
  c := u:r_esult (tasks, tools, 20 * t_ally tools, 0)
  a c_at b c_at c
}}
s1 := u:g_roups same
s2 := u:g_roups shift
s1
s2
## Tokens saved against A on all 8 held-out tasks, less 1500 a tool or hint learned.
tok := (3 s_elect_2 3 3 r_eshape s1) + 3 s_elect_2 3 3 r_eshape s2
((f_irst tok) - 1 d_rop tok) - 1500 * t_ally tools
"#
    )
}

fn rows(v: &[i64]) -> Vec<Row> {
    v.chunks(3).map(|c| Row { done: c[0], calls: c[1], tokens: c[2] }).collect()
}

/// Run the program for `p` and read what it printed.
pub fn results(p: &Params) -> Result<Results, String> {
    let out = run::output(&program(p), 6)?;
    let train: Vec<i64> = run::numbers(&out[0], 2)?;
    let n = run::numbers::<i64>(&out[1], 1)?[0] as usize;
    let flat: Vec<i64> = if n == 0 { vec![] } else { run::numbers(&out[2], 8 * n)? };
    let tools = flat.chunks(8).map(|r| r.iter().copied().filter(|&o| o != 0).collect()).collect();
    Ok(Results {
        train_calls: train[0],
        train_tokens: train[1],
        tools,
        same: rows(&run::numbers(&out[3], 9)?),
        shift: rows(&run::numbers(&out[4], 9)?),
        net: run::numbers(&out[5], 2)?,
    })
}

/// A tool's ops by name.
pub fn named(tool: &[i64]) -> String {
    tool.iter().map(|&o| OPS.get((o - 1) as usize).copied().unwrap_or("?")).collect::<Vec<_>>().join(", ")
}
