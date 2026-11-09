//! Exercise 3 stub. Fill every TODO(you). `cargo run --example ex3_springs`.

// @snip:ex3
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, Theme, motion, type_};
use rune::{AppSpec, Cx, El, Spring, Transition};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    right: bool,
}

const TRACK_W: f32 = 300.0;
const BALL: f32 = 28.0;

// TODO(you) 1: a third lane with a spring you design. Spring::new takes a duration in seconds
//   and a bounce from 0 up to (not including) 1. Pick numbers you expect to look slow and wobbly, then check.
const LANES: [(&str, Spring); 2] = [("SNAPPY", motion::SNAPPY), ("PLAYFUL", motion::PLAYFUL)];

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - which end of the track, in points.
fn ball_x(right: bool) -> f32 {
    // in: bool → out: f32
    // TODO(you) 2: the ball's left edge at each end. At the right end, how much of the track
    //   does the ball itself use up?
    let _ = right;
    todo!() // remove when implemented
}

fn lane(name: &str, spring: Spring, right: bool, theme: Theme) -> El {
    // TODO(you) 3: the ball jumps instead of gliding. You never write a "move from A to B"
    //   animation in Rune - so what do you tell the element about HOW its changes should land?
    let _ = spring;
    let ball = rune::el()
        .key(format!("ball-{name}"))
        .size(BALL, BALL)
        .radius(BALL / 2.0)
        .fill(tok::accent(0, true))
        .absolute(ball_x(right), 0.0);
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
        )
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
        .on_press(|state: &mut State| state.right = !state.right);
    let mut footer = rune::row().h(34.0).gap(14.0).center_y().child(go);
    if state.right {
        // TODO(you) 4: "Wheee" pops in and out. Make it rise 10 pt while fading in, and do the
        //   reverse on the way out. Which builder describes an element's entrance and exit,
        //   and how would you combine two of them?
        footer = footer.child(
            rune::text("Wheee")
                .key("wheee")
                .font(type_::BODY)
                .color(theme.text),
        );
    }
    let _ = Transition::opacity; // a hint that you will need this type
    rune::col()
        .w_full()
        .h_full()
        .center()
        .gap(20.0)
        .fill(theme.bg)
        .child(lanes)
        .child(footer)
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    rune::run(AppSpec::new(
        "Exercise 3 - Springs",
        (400.0, 300.0),
        State::default,
        view,
    ));
}
// @end
