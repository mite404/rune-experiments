//! Exercise 7 stub. Fill every TODO(you). `cargo run --example ex7_glass`.

// @snip:ex7
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, Drag, El, Phase, Tx};
use rune_tutorial::gfx::{Gfx, LENS};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    pull_x: f32,
    pull_y: f32,
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

const WIDTH: f32 = 420.0;
const HEIGHT: f32 = 320.0;
const PILL: Rect = Rect {
    x: 110.0,
    y: 130.0,
    width: 200.0,
    height: 56.0,
};
const DROP_RADIUS: f32 = 18.0;
const STRIPE_COLOURS: [&str; 4] = ["#ff5ca8", "#7c5cff", "#22d3ee", "#ffb547"];

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - the pull in, the drop's centre in window points out.
fn drop_centre(state: &State) -> (f32, f32) {
    // in: &State → out: (f32, f32)
    // TODO(you) 1: at rest the drop hangs just under the pill's right end, half inside it.
    //   Start from PILL's fields. Then add the pull.
    let _ = (state, PILL.x);
    todo!() // remove when implemented
}

fn stripes() -> El {
    let mut stripes = rune::row()
        .absolute(0.0, 0.0)
        .size(WIDTH, HEIGHT)
        .gap(12.0)
        .justify_center();
    for i in 0..22 {
        let colour = rune::color::hex(STRIPE_COLOURS[i % 4]).expect("valid hex colour");
        stripes = stripes.child(rune::el().w(8.0).h_full().fill(colour));
    }
    stripes
}

fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true);
    let (drop_x, drop_y) = drop_centre(state);
    // TODO(you) 2: the glass shows nothing behind it. A lens has to SEE the stripes to bend
    //   them - which call lets a slot read what was drawn before it?
    let glass = rune::effect(LENS)
        .effect_data([drop_x, drop_y, DROP_RADIUS, 0.1])
        .absolute(PILL.x, PILL.y)
        .size(PILL.width, PILL.height)
        .radius(PILL.height / 2.0);
    let handle = rune::el()
        .key("drop")
        .absolute(drop_x - DROP_RADIUS, drop_y - DROP_RADIUS)
        .size(2.0 * DROP_RADIUS, 2.0 * DROP_RADIUS)
        .radius(DROP_RADIUS)
        .on_drag(
            |state: &mut State, drag: &mut Drag, _: &mut Tx| match drag.phase {
                Phase::Down => {}
                // TODO(you) 3: follow the finger in both axes. Lesson 4 used one field of `drag`;
                //   this time you need two.
                Phase::Move => {
                    let _ = (state, drag);
                }
                // TODO(you) 4: let go and the drop goes home. Predict: will it glide or jump?
                //   (Hint: is `effect_data` an element property that springs, or raw numbers?)
                Phase::Up => {}
            },
        );
    let hint = rune::row()
        .h(30.0)
        .px(14.0)
        .radius(15.0)
        .center()
        .fill(theme.bg)
        .child(
            rune::text("Pull the drop out of the glass")
                .font(type_::BODY)
                .color(theme.text),
        );
    rune::el()
        .w(WIDTH)
        .h(HEIGHT)
        .fill(rune::color::hex("#0d0a14").expect("valid hex colour"))
        .child(stripes())
        .child(glass)
        .child(handle)
        .child(
            rune::row()
                .absolute(0.0, 40.0)
                .w(WIDTH)
                .center()
                .child(hint),
        )
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::new(
        "Exercise 7 - Liquid glass",
        (WIDTH, HEIGHT),
        State::default,
        view,
    );
    rune::run_with(spec, Gfx::default());
}
// @end
