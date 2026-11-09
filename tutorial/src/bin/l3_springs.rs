//! Lesson 3: springs. Three balls race to the same place on three different springs, and a
//! label enters and leaves with a transition. Press "Go" to send them back and forth.

// @snip:l3
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, Theme, motion, type_};
use rune::{AppSpec, Cx, El, Spring, Transition};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    right: bool, // which end of the track the balls are heading for
}

const TRACK_W: f32 = 300.0;
const BALL: f32 = 28.0;

// Name, spring. `Spring::new(duration in seconds, bounce from 0 up to, not including, 1)`.
const LANES: [(&str, Spring); 3] = [
    ("SNAPPY", motion::SNAPPY),         // 0.25 s, no bounce
    ("PLAYFUL", motion::PLAYFUL),       // 0.25 s, a little bounce
    ("custom", Spring::new(0.8, 0.45)), // slow and bouncy
];

// ── 3. CALCULATION ──────────────────────────────────────────
fn ball_x(right: bool) -> f32 {
    if right { TRACK_W - BALL } else { 0.0 } // → points inside the track
}

fn lane(name: &str, spring: Spring, right: bool, theme: Theme) -> El {
    let ball = rune::el()
        .key(format!("ball-{name}"))
        .size(BALL, BALL)
        .radius(BALL / 2.0)
        .fill(tok::accent(0, true))
        .absolute(ball_x(right), 0.0)
        .motion(spring); // this element's own spring → El
    rune::col()
        .gap(6.0)
        .child(
            rune::text(name)
                .font(type_::LABEL)
                .color(theme.text_secondary),
        )
        .child(
            rune::el()
                .w(TRACK_W)
                .h(BALL)
                .radius(BALL / 2.0)
                .fill(theme.control_off)
                .child(ball),
        ) // → El
}

fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true);
    let mut lanes = rune::col().gap(14.0);
    for (name, spring) in LANES {
        lanes = lanes.child(lane(name, spring, state.right, theme));
    }
    let go = rune::row()
        .key("go")
        .h(34.0)
        .px(18.0)
        .radius(17.0)
        .center()
        .fill(theme.text)
        .press_scale(0.96)
        .child(rune::text("Go").font(type_::BODY).color(theme.bg))
        .on_press(|state: &mut State| state.right = !state.right); // → El
    let mut footer = rune::row().h(34.0).gap(14.0).center_y().child(go);
    if state.right {
        // Mounted only while `right` is true: it enters and exits instead of popping.
        footer = footer.child(
            rune::text("Wheee")
                .key("wheee")
                .font(type_::BODY)
                .color(theme.text)
                .transition(Transition::offset(0.0, 10.0).combined(Transition::opacity())),
        );
    }
    rune::col()
        .w_full()
        .h_full()
        .center()
        .gap(20.0)
        .fill(theme.bg)
        .child(lanes)
        .child(footer) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    rune::run(AppSpec::<State> {
        scenes: &[("right", |state| state.right = true)],
        ..AppSpec::new("Lesson 3 - Springs", (400.0, 300.0), State::default, view)
    });
}
// @end
