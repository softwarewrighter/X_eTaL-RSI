//! The page's model: the inputs, the X_eTaL program written from them
//! (which the page shows, decorated, and runs in the browser), and what
//! it prints read back.

use microscope::run;

/// What the reader sets: an improvement costs `invest` tokens once and
/// cuts a task of its kind from `base` tokens by `saving` percent;
/// `tasks` is how many tasks the curves run to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub invest: i64,
    pub base: i64,
    pub saving: i64,
    pub tasks: i64,
}

/// The research's example: 3000 tokens to save 55% of a 1000-token task.
pub const DEFAULT: Params = Params { invest: 3000, base: 1000, saving: 55, tasks: 30 };

/// The uses per kind the sweep runs over, 40 tasks in all.
pub const USES: [i64; 8] = [1, 2, 4, 5, 8, 10, 20, 40];

/// The groups, in the order the program prints them.
pub const GROUPS: [&str; 4] = ["A fixed", "D always improves", "D-ski cost-aware", "best in hindsight"];

impl Params {
    /// The improved cost of a task, whole tokens (at least 0).
    pub fn improved(&self) -> i64 {
        (self.base * (100 - self.saving) / 100).max(0)
    }
}

/// What the program printed.
#[derive(Clone, Debug, PartialEq)]
pub struct Costs {
    /// Cumulative tokens after each task 1..=tasks, per group A, D, D-ski.
    pub fixed: Vec<i64>,
    pub eager: Vec<i64>,
    pub ski: Vec<i64>,
    /// i / (b - a), -1.0 when it never pays.
    pub break_even: f64,
    /// The cost-aware rule's k (-1: never improve).
    pub k: i64,
    /// Total tokens for 40 tasks, per group (rows) and uses per kind (columns).
    pub sweep: Vec<Vec<i64>>,
}

/// The program for `p`: what the page runs and shows.
pub fn program(p: &Params) -> String {
    let (i, b, a, n) = (p.invest, p.base, p.improved(), p.tasks);
    let uses: Vec<String> = USES.iter().map(i64::to_string).collect();
    format!(
        r#"## Experiment 05 on this page: one improvement costs i = {i} tokens and
## cuts a task from b = {b} to a = {a} tokens. A deterministic simulation.
"ec:" u_se< "Econ"
s := ({i}, {b}, {a})
k := ec:s_kiRental s
rule := ({i}, {b}, {a}, k)
## Cumulative tokens after each of {n} tasks: A fixed, D always, D-ski.
n := r_ange {n}
s ec:f_ixed n
s ec:e_ager n
rule ec:a_fter n
## The break-even, and after how many baseline tasks D-ski improves.
ec:b_reakEven s
k
## 40 tasks, r of each of 40 / r kinds: totals per group and r.
r := {uses}
kinds := 40 d_iv r
r_avel 4 {cols} r_eshape (kinds * s ec:f_ixed r) c_at (kinds * s ec:e_ager r) c_at (kinds * rule ec:a_fter r) c_at kinds * s ec:h_indsight r
"#,
        uses = uses.join(" "),
        cols = USES.len(),
    )
}

/// Run the program for `p` and read what it printed.
pub fn costs(p: &Params) -> Result<Costs, String> {
    let out = run::output(&program(p), 6)?;
    let n = p.tasks as usize;
    let cols = USES.len();
    let sweep: Vec<i64> = run::numbers(&out[5], 4 * cols)?;
    Ok(Costs {
        fixed: run::numbers(&out[0], n)?,
        eager: run::numbers(&out[1], n)?,
        ski: run::numbers(&out[2], n)?,
        break_even: run::numbers::<f64>(&out[3], 1)?[0],
        k: run::numbers::<i64>(&out[4], 1)?[0],
        sweep: sweep.chunks(cols).map(<[i64]>::to_vec).collect(),
    })
}
