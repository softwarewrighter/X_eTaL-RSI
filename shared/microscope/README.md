# The microscope: the experiment pages' shared shell

Every live experiment page runs an X_eTaL program in the browser (the
pinned X_eTaL, compiled to WebAssembly), reads back what it prints and
draws it beside the program's decorated source. This crate is the core
of X_eTaL-ML's shell of the same name, copied so the pages run this
repository's own pin (`XETAL_COMMIT`) rather than X_eTaL-ML's.

| Module | What it gives a page |
| ------ | -------------------- |
| `run` | `output(src, lines)` runs a program and returns its printed lines, or the X_eTaL error; `numbers(line, n)` reads one printed array; `lit(x)` writes a Float literal; `section` cuts part of a program |
| `source` | `code(src)` and `block(src, range)` draw X_eTaL decorated and colored, a range highlighted |
| `chrome` | `header`, `panel`, `notice` and `footer` (copyright, license, repository, the pinned X_eTaL commit, the build's host, commit and time) |
| `libs` | this repository's libraries and those used from X_eTaL-libraries, embedded at build time so a page's program imports them with `u_se<` |

`microscope.css` is the shared stylesheet; a page links it with
`<link data-trunk rel="css" href="../../../shared/microscope/microscope.css" />`.

## A new experiment page

1. `experiments/NN-slug/web/`, a Cargo workspace like
   `experiments/05-specialization-cost/web/`: `Cargo.toml` depending on
   `microscope = { path = "../../../shared/microscope" }`, `index.html`
   with the trunk links, and `browser.txt`, lines the page shows only
   once X_eTaL has run.
2. `src/micro.rs`: the program written from the page's inputs, run and
   read back, tested natively in `tests/` against the command-line
   experiment and the arithmetic. `src/app.rs`: the page (Yew).
3. `just pages NN`, `just browser-check NN`, `just serve`; commit, then
   `just publish` and `just check-live`.

Conventions: the page shows the whole program it runs, decorated; an
X_eTaL error keeps the last good result and shows a notice; a page
that simulates says so.
