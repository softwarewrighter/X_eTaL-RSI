//! The page: the starting optimizer and the meta-level's settings, the
//! cumulative tokens of the fixed harness, D and E batch by batch, the
//! totals, E's optimizer and both groups' revisions over time, where
//! the experiment comes from, and the X_eTaL program, drawn decorated.

use microscope::{chrome, source};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::micro::{program, results, Params, Results, DEFAULT, N};

#[derive(Clone, PartialEq)]
struct State {
    params: Params,
    results: Option<Results>,
    notice: Option<String>,
}

impl State {
    fn with(params: Params, last: Option<Results>) -> State {
        match results(&params) {
            Ok(r) => State { params, results: Some(r), notice: None },
            Err(e) => State { params, results: last, notice: Some(format!("X_eTaL stopped: {e}")) },
        }
    }
}

const COLORS: [&str; 3] = ["#7a7a7a", "#2f9e6f", "#2f6fd0"];
const GROUPS: [&str; 3] = ["A fixed harness", "D fixed optimizer", "E edits its optimizer"];
const RECIPES: [&str; 4] = ["1 wood, 1 ore", "2 wood", "2 ore", "2 of each"];

/// A labeled slider for one setting; the page recomputes on release.
fn slider(label: &str, value: i64, (lo, hi, step): (i64, i64, i64), unit: &str, set: Callback<i64>) -> Html {
    let unit = if value == 1 { unit.strip_suffix("es").unwrap_or(unit) } else { unit };
    let onchange = Callback::from(move |e: Event| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(lo);
        set.emit(v);
    });
    html! {
        <label class="slider">
            <span class="name">{ label }</span>
            <input type="range" min={lo.to_string()} max={hi.to_string()} step={step.to_string()} value={value.to_string()} {onchange} />
            <span class="value">{ format!("{value}{unit}") }</span>
        </label>
    }
}

/// Cumulative tokens by batch, a line per group, the recipe shifts dashed.
fn chart(r: &Results) -> Html {
    let (w, h, pad) = (640.0, 300.0, 44.0);
    let sums: Vec<Vec<i64>> = r.per_batch.iter().map(|v| v.iter().scan(0, |s, x| { *s += x; Some(*s) }).collect()).collect();
    let top = sums.iter().flatten().copied().max().unwrap_or(1).max(1) as f64;
    let x = |t: f64| pad + (w - 2.0 * pad) * t / N as f64;
    let y = |v: f64| h - pad - (h - 2.0 * pad) * v / top;
    let line = |vals: &[i64], color: &str| {
        let pts: Vec<String> = std::iter::once((0.0, 0.0))
            .chain(vals.iter().enumerate().map(|(i, v)| ((i + 1) as f64, *v as f64)))
            .map(|(t, v)| format!("{:.1},{:.1}", x(t), y(v)))
            .collect();
        html! { <polyline points={pts.join(" ")} fill="none" stroke={color.to_string()} stroke-width="2.5" /> }
    };
    let shift = |b: usize| {
        let bx = x(b as f64);
        html! { <line x1={bx.to_string()} y1={pad.to_string()} x2={bx.to_string()} y2={(h - pad).to_string()} stroke="#999" stroke-dasharray="4 4" /> }
    };
    html! {
        <svg class="chart" viewBox={format!("0 0 {w} {h}")} role="img" aria-label="cumulative tokens by batch">
            <line x1={pad.to_string()} y1={(h - pad).to_string()} x2={(w - pad).to_string()} y2={(h - pad).to_string()} stroke="#888" />
            <line x1={pad.to_string()} y1={pad.to_string()} x2={pad.to_string()} y2={(h - pad).to_string()} stroke="#888" />
            <text x={pad.to_string()} y={(pad - 8.0).to_string()} class="axis">{ format!("{} tokens", top as i64) }</text>
            <text x={(w - pad).to_string()} y={(h - pad + 18.0).to_string()} class="axis" text-anchor="end">{ format!("batch {N}") }</text>
            { for [6, 12, 18].into_iter().map(shift) }
            { for (0..3).map(|g| line(&sums[g], COLORS[g])) }
        </svg>
    }
}

fn legend() -> Html {
    let item = |i: usize| html! { <span class="key"><span class="swatch" style={format!("background:{}", COLORS[i])}></span>{ GROUPS[i] }</span> };
    html! { <p class="legend">{ item(0) }{ item(1) }{ item(2) }{ " (dashed: the recipe shifts)" }</p> }
}

fn totals(r: &Results) -> Html {
    let top = r.totals.iter().map(|t| t.all()).max().unwrap_or(1).max(1) as f64;
    let base = r.totals[0].all().max(1) as f64;
    html! {
        <table class="sweep">
            <tr><th>{ "group" }</th><th>{ "production" }</th><th>{ "writing tools" }</th><th>{ "reviews" }</th><th>{ "meta-level" }</th><th>{ "all" }</th><th>{ "against A" }</th><th></th></tr>
            { for r.totals.iter().enumerate().map(|(g, t)| {
                let width = format!("width:{:.1}%;background:{}", 100.0 * t.all() as f64 / top, COLORS[g]);
                html! {
                    <tr>
                        <th>{ GROUPS[g] }</th><td>{ t.production }</td><td>{ t.writing }</td><td>{ t.review }</td><td>{ t.meta }</td><td>{ t.all() }</td>
                        <td>{ format!("{}%", (100.0 * t.all() as f64 / base).round() as i64) }</td>
                        <td class="barcell"><div class="bar" style={width}></div></td>
                    </tr>
                }
            }) }
        </table>
    }
}

fn verdict(v: i64) -> String {
    match v {
        -1 => "".into(),
        0 => "accepted".into(),
        n => format!("rejected ({n})"),
    }
}

/// A row per batch: the recipe, D's and E's revisions, E's optimizer
/// (changes marked).
fn timeline(r: &Results) -> Html {
    html! {
        <table class="sweep">
            <tr><th>{ "batch" }</th><th>{ "recipe" }</th><th>{ "D's revision" }</th><th>{ "E's revision" }</th><th>{ "E's hi" }</th><th>{ "k" }</th><th>{ "window" }</th><th>{ "period" }</th></tr>
            { for (0..N).map(|b| {
                let o = r.optimizer[b];
                let before = if b == 0 { o } else { r.optimizer[b - 1] };
                html! {
                    <tr>
                        <th>{ b + 1 }</th><td>{ RECIPES[b / 6] }</td>
                        <td>{ verdict(r.verdicts[0][b]) }</td><td>{ verdict(r.verdicts[1][b]) }</td>
                        { for (0..4).map(|i| html! { <td class={classes!((o[i] != before[i]).then_some("best"))}>{ o[i] }</td> }) }
                    </tr>
                }
            }) }
        </table>
    }
}

fn background() -> Html {
    html! {
        <section class="panel">
            <h2>{ "Where this comes from" }</h2>
            <p>
                { "Exo lets a system rewrite not only its harness but the policy that decides how the harness is rewritten (Krentsel, Ratnasamy, Stoica, Casado and Goyal, " }
                <a href="https://arxiv.org/abs/2611.00463" target="_blank">{ "Exo: An Architecture for Self-Improving Systems at Runtime" }</a>
                { ", arXiv 2611.00463, 2026). This page asks whether that second level pays: group D improves its tools with a fixed optimizer, group E also edits the optimizer (the research's ablation groups D and E; hypothesis H3)." }
            </p>
            <p>
                { "The optimizer is a vector of five numbers, interpreted by fixed library code (Evolve): learn tools of 2 to hi ops, keep the k worth the most model calls saved, learn from the last window tasks, revise every period batches. Every candidate is reviewed by a layered guardian on the batch just done (Guardian), and every token is counted: the work, writing the tools, the reviews." }
            </p>
            <p>
                { "E's meta-level is fixed, so the tower of improvers stops there: before every few batches it replays the recent batches under its optimizer and each one a step away and adopts the cheapest. The replay needs no model, because tools change how many calls a task takes, never which steps it runs; it pays for the history it reads. Experiment 03 shows one optimizer at work; " }
                <a href="https://github.com/softwarewrighter/X_eTaL-RSI/blob/main/experiments/04-recursive-optimizer/README.md" target="_blank">{ "experiment 04's README" }</a>
                { " has the command-line results, among them the best of 180 fixed optimizers on this stream." }
            </p>
        </section>
    }
}

#[function_component]
pub fn App() -> Html {
    let state = use_state(|| State::with(DEFAULT, None));
    let p = state.params;
    let setter = |f: fn(&mut Params, i64)| {
        let state = state.clone();
        Callback::from(move |v: i64| {
            let mut np = state.params;
            f(&mut np, v);
            state.set(State::with(np, state.results.clone()));
        })
    };
    let src = program(&p);
    html! {
        <main>
            { chrome::header("04 Recursive optimizer", "A harness can improve its tools; can it also improve how it improves them? D runs a fixed optimizer, E starts from the same one and edits it as it works. Every number below is computed in your browser by X_eTaL (the Evolve, Guardian and Policy libraries), from the program at the bottom.") }
            <p class="sim">{ "A deterministic simulation on a small manufacturing world: it shows the mechanics of an optimizer that edits itself; it is not evidence that a model would." }</p>
            <section class="panel">
                <h2>{ "Settings" }</h2>
                <p class="note">{ "The optimizer D runs and E starts from (experiment 03's: 8, 3, 6, 1; a bad one: 3, 3, 3, 1; the best found afterwards: 8, 1, 3, 1):" }</p>
                { slider("longest tool", p.hi, (2, 8, 1), " ops", setter(|p, v| p.hi = v)) }
                { slider("tools kept", p.k, (1, 5, 1), "", setter(|p, v| p.k = v)) }
                { slider("learn from", p.window, (3, 12, 3), " recent tasks", setter(|p, v| p.window = v)) }
                { slider("revise every", p.period, (1, 6, 1), " batches", setter(|p, v| p.period = v)) }
                <p class="note">{ "E's meta-level, which nothing edits:" }</p>
                { slider("replay", p.replay, (2, 6, 1), " batches", setter(|p, v| p.replay = v)) }
                { slider("retune every", p.every, (2, 6, 1), " batches", setter(|p, v| p.every = v)) }
                <p class="note">{ "A model call costs 500 tokens, a tool's schema 20 in every prompt, writing a tool 1500, the meta-level 5 for each event of history it reads. The page recomputes when a slider is released; it runs dozens of simulated harnesses, so allow several seconds." }</p>
            </section>
            { chrome::notice(&state.notice) }
            { for state.results.as_ref().map(|r| html! { <>
                <section class="panel">
                    <h2>{ "Tokens over 24 batches" }</h2>
                    <p class="note">{ "Three tasks a batch; the recipe shifts every 6 batches. Cumulative tokens of everything each group spends:" }</p>
                    { chart(r) }
                    { legend() }
                    { totals(r) }
                </section>
                <section class="panel">
                    <h2>{ "Revisions and E's optimizer, batch by batch" }</h2>
                    <p class="note">{ "A revision is proposed before a batch that is due, unless it would change nothing; the guardian accepts it or names the layer it failed (5: costlier on the last batch than the running tools, 6: not cheaper). E's settings are marked where its meta-level changed them." }</p>
                    { timeline(r) }
                </section>
            </> }) }
            { background() }
            <section class="panel">
                <h2>{ "The X_eTaL program this page runs" }</h2>
                <p class="note">{ "Rewritten from the settings and run on every change; the libraries are in libs/Evolve, libs/Guardian and libs/Policy." }</p>
                { source::block(&src, source::NONE) }
            </section>
            { chrome::footer() }
        </main>
    }
}
