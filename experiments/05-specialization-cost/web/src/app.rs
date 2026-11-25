//! The page: sliders for the improvement, the cumulative cost curves of
//! three harnesses, the sweep over repetition, and the X_eTaL program
//! that computed them, drawn decorated.

use microscope::{chrome, source};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::micro::{costs, program, Costs, Params, DEFAULT, GROUPS, USES};

/// The page's state: the inputs, the last costs X_eTaL computed, and why
/// the last change was not shown, if it was not.
#[derive(Clone, PartialEq)]
struct State {
    params: Params,
    costs: Option<Costs>,
    notice: Option<String>,
}

impl State {
    fn with(params: Params, last: Option<Costs>) -> State {
        match costs(&params) {
            Ok(c) => State { params, costs: Some(c), notice: None },
            Err(e) => State { params, costs: last, notice: Some(format!("X_eTaL stopped: {e}")) },
        }
    }
}

const COLORS: [&str; 3] = ["#7a7a7a", "#e8730c", "#2f6fd0"];

/// The three cumulative curves, task 0 to n, with the break-even marked.
fn chart(c: &Costs, n: i64) -> Html {
    let (w, h, pad) = (640.0, 300.0, 44.0);
    let top = c.fixed.iter().chain(&c.eager).chain(&c.ski).copied().max().unwrap_or(1).max(1) as f64;
    let x = |t: f64| pad + (w - 2.0 * pad) * t / n as f64;
    let y = |v: f64| h - pad - (h - 2.0 * pad) * v / top;
    let line = |vals: &[i64], color: &str| {
        let pts: Vec<String> = std::iter::once((0.0, 0.0))
            .chain(vals.iter().enumerate().map(|(i, v)| ((i + 1) as f64, *v as f64)))
            .map(|(t, v)| format!("{:.1},{:.1}", x(t), y(v)))
            .collect();
        html! { <polyline points={pts.join(" ")} fill="none" stroke={color.to_string()} stroke-width="2.5" /> }
    };
    let be = (c.break_even > 0.0 && c.break_even < n as f64).then(|| {
        let bx = x(c.break_even);
        html! { <>
            <line x1={bx.to_string()} y1={pad.to_string()} x2={bx.to_string()} y2={(h - pad).to_string()} stroke="#999" stroke-dasharray="4 4" />
            <text x={(bx + 4.0).to_string()} y={(pad + 12.0).to_string()} class="axis">{ format!("break-even {:.2}", c.break_even) }</text>
        </> }
    });
    html! {
        <svg class="chart" viewBox={format!("0 0 {w} {h}")} role="img" aria-label="cumulative tokens by task">
            <line x1={pad.to_string()} y1={(h - pad).to_string()} x2={(w - pad).to_string()} y2={(h - pad).to_string()} stroke="#888" />
            <line x1={pad.to_string()} y1={pad.to_string()} x2={pad.to_string()} y2={(h - pad).to_string()} stroke="#888" />
            <text x={pad.to_string()} y={(pad - 8.0).to_string()} class="axis">{ format!("{} tokens", top as i64) }</text>
            <text x={(w - pad).to_string()} y={(h - pad + 18.0).to_string()} class="axis" text-anchor="end">{ format!("task {n}") }</text>
            { line(&c.fixed, COLORS[0]) }
            { line(&c.eager, COLORS[1]) }
            { line(&c.ski, COLORS[2]) }
            { for be }
        </svg>
    }
}

fn legend() -> Html {
    let item = |i: usize| html! { <span class="key"><span class="swatch" style={format!("background:{}", COLORS[i])}></span>{ GROUPS[i] }</span> };
    html! { <p class="legend">{ item(0) }{ item(1) }{ item(2) }</p> }
}

/// The sweep: totals for 40 tasks, a row per group, a column per uses per kind.
fn sweep(c: &Costs) -> Html {
    let best = &c.sweep[3];
    html! {
        <table class="sweep">
            <tr><th>{ "uses per kind" }</th>{ for USES.iter().map(|u| html! { <th>{ u }</th> }) }</tr>
            { for c.sweep.iter().enumerate().map(|(g, row)| html! {
                <tr><th>{ GROUPS[g] }</th>{ for row.iter().zip(best).map(|(v, b)| html! {
                    <td class={classes!((g < 3 && v == b).then_some("best"))}>{ v }</td>
                }) }</tr>
            }) }
        </table>
    }
}

/// A labeled slider for one input.
fn slider(label: &str, value: i64, (lo, hi, step): (i64, i64, i64), unit: &str, set: Callback<i64>) -> Html {
    let oninput = Callback::from(move |e: InputEvent| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(lo);
        set.emit(v);
    });
    html! {
        <label class="slider">
            <span class="name">{ label }</span>
            <input type="range" min={lo.to_string()} max={hi.to_string()} step={step.to_string()} value={value.to_string()} {oninput} />
            <span class="value">{ format!("{value}{unit}") }</span>
        </label>
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
            state.set(State::with(np, state.costs.clone()));
        })
    };
    let src = program(&p);
    html! {
        <main>
            { chrome::header("05 Specialization cost", "Self-improvement is an investment: an agent that builds itself a tool pays for it once and saves on every later task of that kind. When does it pay, and can a harness decide when to improve without knowing the future? Every number below is computed in your browser by X_eTaL (the Econ library), from the program at the bottom.") }
            <p class="sim">{ "A deterministic simulation: it shows the economics of self-improvement on scripted workloads; it is not evidence that a model learns." }</p>
            <section class="panel">
                <h2>{ "The improvement" }</h2>
                { slider("investment i", p.invest, (500, 10000, 100), " tokens", setter(|p, v| p.invest = v)) }
                { slider("baseline b", p.base, (100, 3000, 50), " tokens a task", setter(|p, v| p.base = v)) }
                { slider("saving", p.saving, (5, 95, 1), "%", setter(|p, v| p.saving = v)) }
                { slider("tasks shown", p.tasks, (10, 80, 1), "", setter(|p, v| p.tasks = v)) }
                <p class="note">{ format!("Improved cost a = {} tokens a task.", p.improved()) }</p>
            </section>
            { chrome::notice(&state.notice) }
            { for state.costs.as_ref().map(|c| html! { <>
                <section class="panel">
                    <h2>{ format!("Cumulative tokens after {} tasks", c.fixed.len()) }</h2>
                    <p class="note">
                        { if c.break_even > 0.0 { format!("Break-even after {:.2} tasks. ", c.break_even) } else { "This improvement never pays. ".into() } }
                        { if c.k >= 0 { format!("The cost-aware rule improves after {} baseline tasks (ski rental: never more than twice the best choice in hindsight).", c.k) } else { "The cost-aware rule never improves.".into() } }
                    </p>
                    { legend() }
                    { chart(c, p.tasks) }
                    <p class="note">{ format!("After {} tasks: fixed {}, always improving {}, cost-aware {}.", c.fixed.len(), c.fixed.last().unwrap_or(&0), c.eager.last().unwrap_or(&0), c.ski.last().unwrap_or(&0)) }</p>
                </section>
                <section class="panel">
                    <h2>{ "Totals for 40 tasks by uses per kind" }</h2>
                    <p class="note">{ "40 tasks made of 40 / r kinds, each used r times: from all different (1) to all the same (40). Shaded: a group matching the best in hindsight. Always improving loses badly on rare work; the cost-aware rule never does, but trails on routine work." }</p>
                    { sweep(c) }
                </section>
            </> }) }
            <section class="panel">
                <h2>{ "The X_eTaL program this page runs" }</h2>
                <p class="note">{ "Rewritten from the sliders and run on every change; the Econ library is in libs/Econ." }</p>
                { source::block(&src, source::NONE) }
            </section>
            { chrome::footer() }
        </main>
    }
}
