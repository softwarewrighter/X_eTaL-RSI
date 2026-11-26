//! The page's model: the settings, the X_eTaL program written from them
//! (which the page shows, decorated, and runs in the browser), and what
//! it prints read back.

use microscope::run;

/// What the reader sets: the optimizer D runs and E starts from (tools
/// of 2 to `hi` ops, the `k` worth most calls saved, learned from the
/// last `window` tasks, revised every `period` batches), and E's
/// meta-level: it replays the last `replay` batches before every
/// `every`-th batch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub hi: i64,
    pub k: i64,
    pub window: i64,
    pub period: i64,
    pub replay: i64,
    pub every: i64,
}

/// The command-line experiment's first start: experiment 03's optimizer.
pub const DEFAULT: Params = Params { hi: 8, k: 3, window: 6, period: 1, replay: 3, every: 3 };

/// The stream: 24 batches of 3 tasks (q w r), the recipe shifting every 6.
pub const BATCHES: &str = "2 1 1 3 1 1 1 1 1 4 1 1 2 1 1 3 1 1 3 1 1 1 1 1 2 1 1 2 1 1 4 1 1 3 1 1 3 1 1 2 1 1 1 1 1 1 1 1 3 1 1 2 1 1 2 2 1 3 2 1 1 2 1 4 2 1 2 2 1 3 2 1 2 2 1 1 2 1 3 2 1 3 2 1 2 2 1 4 2 1 1 2 1 3 2 1 2 2 1 3 2 1 2 2 1 2 2 1 3 1 2 2 1 2 1 1 2 4 1 2 2 1 2 3 1 2 2 1 2 1 1 2 3 1 2 2 1 2 3 1 2 4 1 2 3 1 2 1 1 2 2 1 2 2 1 2 2 1 2 3 1 2 2 2 2 3 2 2 1 2 2 3 2 2 1 2 2 2 2 2 1 2 2 2 2 2 3 2 2 3 2 2 2 2 2 2 2 2 2 2 2 3 2 2 1 2 2 2 2 2 1 2 2 3 2 2";

/// The number of batches.
pub const N: usize = 24;

/// One group's tokens in all.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Totals {
    pub production: i64,
    pub writing: i64,
    pub review: i64,
    pub meta: i64,
}

impl Totals {
    pub fn all(&self) -> i64 {
        self.production + self.writing + self.review + self.meta
    }
}

/// What the program printed.
#[derive(Clone, Debug, PartialEq)]
pub struct Results {
    /// Every token each batch spent (production, writing, review, meta),
    /// for A, D and E.
    pub per_batch: [Vec<i64>; 3],
    /// D's and E's revision verdict before each batch (-1 none, 0
    /// accepted, else the Guardian layer that failed).
    pub verdicts: [Vec<i64>; 2],
    /// The optimizer E ran each batch: hi, k, window, period.
    pub optimizer: Vec<[i64; 4]>,
    /// A, D, E.
    pub totals: [Totals; 3],
}

/// The program for `p`: what the page runs and shows.
pub fn program(p: &Params) -> String {
    let Params { hi, k, window, period, replay, every } = *p;
    format!(
        r#"## Experiment 04 on this page: the fixed harness, D (a fixed
## optimizer) and E (the same optimizer, retuned by a meta-level) on
## 24 batches whose recipe shifts every 6. A deterministic simulation
## on the Workshop world; a model call costs 500 tokens.
"po:" u_se< "Policy"
"ev:" u_se< "Evolve"
batches := 24 3 3 r_eshape {BATCHES}
## (suite, allowed ops, most tools, call, per tool, write a tool, layers)
g := (3 3 r_eshape 2 1 1 2 2 1 2 1 2, 1 2 3 4 5 6 7, 6, 500, 20, 1500, 6)
P := (po:n_one 8, 60)
## The optimizer: lo hi k window period.
o := 2 {hi} {k} {window} {period}
(ha, ja, ra) := g ev:e_volve (2 8 3 6 0, P, batches)
(hd, jd, rd) := g ev:e_volve (o, P, batches)
## E: replay the last {replay} batches before every {every}th batch,
## 5 tokens for each journal event read.
(he, je, re) := (g, {replay}, {every}, 5) ev:r_ecursive (o, P, batches)
## Each batch's tokens: production, writing and review (and E's reads).
u:b_atch := {{ rep -> '+ r_/_2 4 5 6 s_elect_2 rep }}
u:b_atch ra
u:b_atch rd
(u:b_atch re) + 7 s_elect_2 re
1 s_elect_2 rd
1 s_elect_2 re
r_avel o_\ 10 11 12 s_elect_2 re
r_avel 9 s_elect_2 re
(ev:t_okens ra) c_at (ev:t_okens rd) c_at (ev:t_okens re) c_at '+ r_/ 7 s_elect_2 re
"#
    )
}

fn totals(v: &[i64]) -> [Totals; 3] {
    let t = |s: &[i64]| Totals { production: s[0], writing: s[1], review: s[2], meta: 0 };
    let mut e = t(&v[6..9]);
    e.meta = v[9];
    [t(&v[0..3]), t(&v[3..6]), e]
}

/// Run the program for `p` and read what it printed.
pub fn results(p: &Params) -> Result<Results, String> {
    let out = run::output(&program(p), 8)?;
    let num = |i: usize, n: usize| run::numbers::<i64>(&out[i], n);
    let rest = num(5, 3 * N)?;
    let hi = num(6, N)?;
    let optimizer = (0..N).map(|b| [hi[b], rest[b], rest[N + b], rest[2 * N + b]]).collect();
    Ok(Results {
        per_batch: [num(0, N)?, num(1, N)?, num(2, N)?],
        verdicts: [num(3, N)?, num(4, N)?],
        optimizer,
        totals: totals(&num(7, 10)?),
    })
}
