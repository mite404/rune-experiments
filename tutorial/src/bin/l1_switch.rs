//! Lesson 1: one piece of state, a view of it, and a switch that flips on a spring.
//! `cargo run --bin l1_switch` opens a window; `-- --snap on.png --scene on` renders it headless.

// @snip:l1
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, motion, type_};
use rune::{AppSpec, Cx, El};

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
fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true); // → Theme (dark colours)
    let label = rune::text(if state.on { "Lights on" } else { "Lights off" })
        .font(type_::TITLE)
        .color(theme.text); // → El
    let knob = rune::el()
        .key("knob")
        .size(18.0, 18.0)
        .radius(9.0)
        .fill(theme.knob)
        .absolute(knob_x(state.on), 2.0); // → El
    let switch = rune::el()
        .key("switch")
        .role("switch")
        .label("Lights")
        .checked(state.on)
        .focusable(true) // what a screen reader hears
        .size(38.0, 22.0)
        .radius(11.0)
        .fill(if state.on {
            tok::accent(0, true)
        } else {
            theme.control_off
        })
        .motion(motion::SNAPPY) // changes to this element and its children spring
        .on_press(|state: &mut State| state.on = !state.on) // the one Action: flip the bool
        .child(knob); // → El
    rune::col()
        .w_full()
        .h_full()
        .center()
        .gap(16.0)
        .fill(theme.bg)
        .child(label)
        .child(switch) // → El (the root)
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    rune::run(AppSpec::<State> {
        scenes: &[("on", |state| state.on = true)], // named starting states for headless snaps
        ..AppSpec::new("Lesson 1 - Switch", (360.0, 220.0), State::default, view)
    });
}
// @end
