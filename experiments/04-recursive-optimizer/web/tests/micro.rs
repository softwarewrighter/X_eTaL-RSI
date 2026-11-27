//! The page's program, run natively with the same pinned X_eTaL the
//! browser runs: its answers against the command-line experiment's.

use recursive_optimizer_web::micro::{program, results, Params, DEFAULT};

#[test]
fn from_experiment_03s_optimizer() {
    // experiments/04-recursive-optimizer/README.md, Results.
    let r = results(&DEFAULT).unwrap();
    let all: Vec<i64> = r.totals.iter().map(|t| t.all()).collect();
    assert_eq!(all, vec![647000, 930940, 736680]);
    assert_eq!(r.totals[2].meta, 44240);
    // E moved hi 8 to 7 before batch 4 and k 3 to 2 before batch 16.
    assert_eq!(r.optimizer[2], [8, 3, 6, 1]);
    assert_eq!(r.optimizer[3], [7, 3, 6, 1]);
    assert_eq!(r.optimizer[15], [7, 2, 6, 1]);
    for g in 0..3 {
        assert_eq!(r.per_batch[g].iter().sum::<i64>(), all[g]);
    }
}

#[test]
fn from_the_bad_optimizer() {
    let r = results(&Params { hi: 3, k: 3, window: 3, ..DEFAULT }).unwrap();
    let all: Vec<i64> = r.totals.iter().map(|t| t.all()).collect();
    assert_eq!(all, vec![647000, 1775220, 870240]);
    assert_eq!(r.optimizer[9], [3, 4, 3, 1]);
}

#[test]
fn the_simulation_notice() {
    assert!(program(&DEFAULT).contains("simulation"));
}
