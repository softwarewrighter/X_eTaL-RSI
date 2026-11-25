//! The page: settings for learning tools and for hints, what was
//! learned, the three harnesses on held-out tasks, where the experiment
//! comes from, and the X_eTaL program that computed it all, drawn
//! decorated.

use microscope::{chrome, source};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::micro::{named, program, results, Params, Results, Row, DEFAULT, GROUPS};

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

/// A labeled slider for one setting; the page recomputes on release.
fn slider(label: &str, value: i64, (lo, hi, step): (i64, i64, i64), unit: &str, set: Callback<i64>) -> Html {
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

/// One held-out set: a row per group, its calls and tokens, a bar of its
/// tokens against the fixed harness's.
fn table(rows: &[Row]) -> Html {
    let top = rows.iter().map(|r| r.tokens).max().unwrap_or(1).max(1) as f64;
    let base = rows[0].tokens.max(1) as f64;
    html! {
        <table class="sweep">
            <tr><th>{ "group" }</th><th>{ "tasks done" }</th><th>{ "model calls" }</th><th>{ "tokens" }</th><th>{ "against A" }</th><th></th></tr>
            { for rows.iter().enumerate().map(|(g, r)| {
                let pct = (100.0 * r.tokens as f64 / base).round() as i64;
                let width = format!("width:{:.1}%;background:{}", 100.0 * r.tokens as f64 / top, COLORS[g]);
                html! {
                    <tr>
                        <th>{ GROUPS[g] }</th><td>{ format!("{} of 4", r.done) }</td><td>{ r.calls }</td><td>{ r.tokens }</td>
                        <td>{ format!("{pct}%") }</td>
                        <td class="barcell"><div class="bar" style={width}></div></td>
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
                { "The first kind of self-improvement in the Exo paper: a system notices it keeps doing the same steps and builds itself a tool, so later work needs fewer model calls, without any weight changing (Krentsel, Ratnasamy, Stoica, Casado and Goyal, " }
                <a href="https://arxiv.org/abs/2611.00463" target="_blank">{ "Exo: An Architecture for Self-Improving Systems at Runtime" }</a>
                { ", arXiv 2611.00463, 2026)." }
            </p>
            <p>
                { "Group B keeps the same lessons as text in the prompt instead, as in-context memories do: XSkill's skills and experiences (" }
                <a href="https://arxiv.org/abs/2603.12056" target="_blank">{ "arXiv 2603.12056" }</a>
                { ") and agentrail-rs's injected trajectories (in-context reinforcement learning). Advice costs context on every call and a model may not follow it; a tool runs as written. " }
                <a href="https://github.com/softwarewrighter/X_eTaL-RSI/blob/main/docs/related.md" target="_blank">{ "docs/related.md" }</a>
                { " compares these approaches." }
            </p>
            <p>
                { "The groups are the research's ablation groups: A a fixed harness, B memory, C tools. Tools are learned from the fixed harness's own journal: every run of 2 or more ops inside a task is counted, and the sequences worth the most calls saved (how often they occur times their length less one) become tools. The model is the Workshop's expert, always right, so the groups differ only in how often it is called and what a call costs; the costs are assumptions, shown in the program." }
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
            { chrome::header("01 Tool synthesis", "An agent that keeps doing the same steps can build itself a tool from its own history. Does the tool cut model calls without changing what gets done, is it better than the same lesson given as advice in the prompt, and does it still help when the work changes? Every number below is computed in your browser by X_eTaL (the Policy, Journal and Workshop libraries), from the program at the bottom.") }
            <p class="sim">{ "A deterministic simulation on a small manufacturing world: it shows the mechanics of learning tools from a history; it is not evidence that a model learns." }</p>
            <section class="panel">
                <h2>{ "Settings" }</h2>
                { slider("longest tool", p.longest, (2, 8, 1), " ops", setter(|p, v| p.longest = v)) }
                { slider("tools learned", p.count, (1, 5, 1), "", setter(|p, v| p.count = v)) }
                { slider("a hint's text", p.hint, (0, 200, 10), " tokens a prompt", setter(|p, v| p.hint = v)) }
                { slider("hints ignored", p.ignore, (0, 8, 1), if p.ignore == 0 { " (never)" } else { "th call" }, setter(|p, v| p.ignore = v)) }
                <p class="note">{ "A model call costs 500 tokens, a tool's schema 20 in every prompt, writing a tool or a hint 1500 once. The page recomputes when a slider is released (about a second)." }</p>
            </section>
            { chrome::notice(&state.notice) }
            { for state.results.as_ref().map(|r| html! { <>
                <section class="panel">
                    <h2>{ format!("Learned from {} model calls of training", r.train_calls) }</h2>
                    <p class="note">{ "The fixed harness did 8 tasks of devices made from 1 wood and 1 ore; its journal is the history. The tools learned, most calls saved first:" }</p>
                    <ol class="tools">{ for r.tools.iter().map(|t| html! { <li>{ named(t) }</li> }) }</ol>
                </section>
                <section class="panel">
                    <h2>{ "Held-out tasks, same recipe" }</h2>
                    <p class="note">{ "4 tasks of 2 to 6 devices of the training recipe." }</p>
                    { table(&r.same) }
                </section>
                <section class="panel">
                    <h2>{ "Held-out tasks, shifted recipes" }</h2>
                    <p class="note">{ "4 tasks whose devices need 2 wood, 2 ore or both: a long route learned on the old recipe no longer fits, and its schema still costs every prompt. Shorten the longest tool to 3 and see what transfers." }</p>
                    { table(&r.shift) }
                    <p class="note">{ format!("On all 8 held-out tasks, after paying {} tokens to learn: B {} and C {}.", 1500 * r.tools.len(), against(r.net[0]), against(r.net[1])) }</p>
                </section>
            </> }) }
            { background() }
            <section class="panel">
                <h2>{ "The X_eTaL program this page runs" }</h2>
                <p class="note">{ "Rewritten from the settings and run on every change; the libraries are in libs/Policy, libs/Journal and libs/Workshop." }</p>
                { source::block(&src, source::NONE) }
            </section>
            { chrome::footer() }
        </main>
    }
}

/// Net tokens against A, in words.
fn against(net: i64) -> String {
    match net {
        n if n > 0 => format!("saves {n} tokens against A"),
        0 => "breaks even with A".into(),
        n => format!("costs {} tokens more than A", -n),
    }
}
