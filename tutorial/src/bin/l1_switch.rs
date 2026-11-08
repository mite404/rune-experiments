//! Lesson 1: one piece of state, a view of it, and a switch that flips on a spring.
//! `cargo run --bin l1_switch` opens a window; `-- --snap on.png --scene on` renders it headless.

// @snip:l1
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, motion, type_};
use rune::{AppSpec, Cx, El, col, el, text};

// ── 2. DATA ─────────────────────────────────────────────────
// Everything the app knows. Nothing else is stored anywhere.
#[derive(Default)]
struct State {
    on: bool,
}

// ── 3. CALCULATION ──────────────────────────────────────────
// Where the knob sits for a given switch position.
fn knob_x(on: bool) -> f32 {
    if on { 18.0 } else { 2.0 } // → points from the track's left edge
}

// The whole screen, described from the state. Rune calls this again after every change.
fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true); // → Theme (dark colours)
    let label = text(if s.on { "Lights on" } else { "Lights off" })
        .font(type_::TITLE)
        .color(t.text); // → El
    let knob = el().key("knob").size(18.0, 18.0).radius(9.0).fill(t.knob)
        .absolute(knob_x(s.on), 2.0); // → El
    let switch = el().key("switch")
        .role("switch").label("Lights").checked(s.on).focusable(true) // what a screen reader hears
        .size(38.0, 22.0).radius(11.0)
        .fill(if s.on { tok::accent(0, true) } else { t.control_off })
        .motion(motion::SNAPPY) // changes to this element and its children spring
        .on_press(|s: &mut State| s.on = !s.on) // the one Action: flip the bool
        .child(knob); // → El
    col().w_full().h_full().center().gap(16.0).fill(t.bg)
        .child(label)
        .child(switch) // → El (the root)
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    rune::run(AppSpec::<State> {
        scenes: &[("on", |s| s.on = true)], // named starting states for headless snaps
        ..AppSpec::new("Lesson 1 - Switch", (360.0, 220.0), State::default, view)
    });
}
// @end
