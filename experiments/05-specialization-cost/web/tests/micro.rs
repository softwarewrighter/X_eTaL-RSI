//! The page's program, run natively with the same pinned X_eTaL the
//! browser runs: its answers against the command-line experiment's and
//! against the arithmetic.

use specialization_cost_web::micro::{costs, program, Params, DEFAULT};

#[test]
fn the_research_example() {
    let c = costs(&DEFAULT).unwrap();
    assert!((c.break_even - 3000.0 / 550.0).abs() < 1e-12);
    assert_eq!(c.k, 6);
    assert_eq!(c.fixed.len(), 30);
    assert_eq!((c.fixed[29], c.eager[29], c.ski[29]), (30000, 16500, 19800));
    assert_eq!(c.ski[5], 6000);
    assert_eq!(c.ski[6], 9450);
}

#[test]
fn the_sweep_is_the_command_line_experiments() {
    // experiments/05-specialization-cost/README.md, the repetition table.
    let c = costs(&DEFAULT).unwrap();
    assert_eq!(c.sweep[0], vec![40000; 8]);
    assert_eq!(c.sweep[1], vec![138000, 78000, 48000, 42000, 33000, 30000, 24000, 21000]);
    assert_eq!(c.sweep[2], vec![40000, 40000, 40000, 40000, 49500, 43200, 30600, 24300]);
    assert_eq!(c.sweep[3], vec![40000, 40000, 40000, 40000, 33000, 30000, 24000, 21000]);
}

#[test]
fn the_curves_are_the_formulas() {
    let p = Params { invest: 5000, base: 800, saving: 25, tasks: 50 };
    let c = costs(&p).unwrap();
    let (i, b, a) = (5000, 800, 600);
    let k = (5000.0_f64 / 200.0).ceil() as i64;
    assert_eq!(c.k, k);
    for t in 1..=50_i64 {
        let u = (t - 1) as usize;
        assert_eq!(c.fixed[u], t * b);
        assert_eq!(c.eager[u], i + t * a);
        let ski = if t <= k { t * b } else { k * b + i + (t - k) * a };
        assert_eq!(c.ski[u], ski);
        let best = c.fixed[u].min(c.eager[u]);
        assert!(c.ski[u] <= 2 * best, "ski rental within twice hindsight at {t}");
    }
}

#[test]
fn the_program_uses_econ_and_says_it_is_a_simulation() {
    let src = program(&DEFAULT);
    assert!(src.contains("u_se< \"Econ\""));
    assert!(src.contains("simulation"));
}
