//! The page's program, run natively with the same pinned X_eTaL the
//! browser runs: its answers against the command-line experiment's.

use tool_synthesis_web::micro::{named, program, results, Params, DEFAULT};

#[test]
fn the_command_line_experiment() {
    // experiments/01-tool-synthesis/README.md, Results.
    let r = results(&DEFAULT).unwrap();
    assert_eq!((r.train_calls, r.train_tokens), (142, 71000));
    assert_eq!(r.tools.len(), 3);
    assert_eq!(r.tools[0], vec![1, 2, 3, 4, 5, 6]);
    let calls = |v: &[tool_synthesis_web::micro::Row]| v.iter().map(|x| x.calls).collect::<Vec<_>>();
    let tokens = |v: &[tool_synthesis_web::micro::Row]| v.iter().map(|x| x.tokens).collect::<Vec<_>>();
    assert_eq!(calls(&r.same), vec![98, 44, 44]);
    assert_eq!(tokens(&r.same), vec![49000, 29920, 24640]);
    assert_eq!(calls(&r.shift), vec![91, 91, 91]);
    assert_eq!(tokens(&r.shift), vec![45500, 61880, 50960]);
    assert_eq!(r.net, vec![-1800, 14400]);
    assert!(r.same.iter().chain(&r.shift).all(|x| x.done == 4));
}

#[test]
fn short_tools_transfer() {
    // C-short in the command-line experiment: 3 tools of at most 3 ops.
    let r = results(&Params { longest: 3, ..DEFAULT }).unwrap();
    assert_eq!(r.tools, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
    assert_eq!((r.same[2].calls, r.same[2].tokens), (68, 38080));
    assert_eq!((r.shift[2].calls, r.shift[2].tokens), (69, 38640));
}

#[test]
fn hints_always_followed_cost_only_their_context() {
    let r = results(&Params { ignore: 0, ..DEFAULT }).unwrap();
    assert_eq!(r.same[1].calls, r.same[2].calls);
    assert!(r.same[1].tokens > r.same[2].tokens);
}

#[test]
fn names_and_the_simulation_notice() {
    assert_eq!(named(&[1, 2, 6]), "go-forest, gather-wood, make");
    assert!(program(&DEFAULT).contains("simulation"));
}
