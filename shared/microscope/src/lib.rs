//! The shared shell of the X_eTaL-RSI experiment pages: every page runs
//! an X_eTaL program in the browser (the pinned X_eTaL, compiled to
//! WebAssembly), reads back what it prints, and shows it beside the
//! program's decorated source. The core of X_eTaL-ML's microscope
//! (`shared/microscope` there), copied so the pages run this
//! repository's own pin (XETAL_COMMIT).
//!
//! - `run`: run a program, read printed arrays, write literals
//! - `source`: decorated source with a highlighted range; shapes
//! - `chrome`: header, panels, notices, footer
//! - `libs`: this repo's libraries, importable with `u_se<`
//!
//! Pages also link `microscope.css` (trunk: `rel="css"`).

pub mod chrome;
pub mod libs;
pub mod run;
pub mod source;
