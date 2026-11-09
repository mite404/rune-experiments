//! Lesson 7: liquid glass that reads the backdrop. Drag the drop out of the glass pill and
//! watch them stay joined like water until you pull far enough. Let go and it snaps home.

// @snip:l7
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, Drag, El, Phase, Tx};
use rune_tutorial::gfx::{Gfx, LENS};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    pull_x: f32, // how far the drop is dragged from home, pt
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
// Where the drop's centre is: just under the pill's right end, plus the pull.
fn drop_centre(state: &State) -> (f32, f32) {
    let x = PILL.x + PILL.width - 40.0 + state.pull_x; // → window points
    let y = PILL.y + PILL.height - 6.0 + state.pull_y;
    (x, y)
}

// Something behind the glass for it to bend.
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
    stripes // → El
}

fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true);
    let (drop_x, drop_y) = drop_centre(state);
    let glass = rune::effect(LENS)
        .effect_data([drop_x, drop_y, DROP_RADIUS, 0.1]) // the drop, gooey-unioned to the pill
        .effect_backdrop(true) // this slot reads what was drawn before it
        .absolute(PILL.x, PILL.y)
        .size(PILL.width, PILL.height)
        .radius(PILL.height / 2.0); // → El
    // An invisible handle over the drop, to grab it with.
    let handle = rune::el()
        .key("drop")
        .absolute(drop_x - DROP_RADIUS, drop_y - DROP_RADIUS)
        .size(2.0 * DROP_RADIUS, 2.0 * DROP_RADIUS)
        .radius(DROP_RADIUS)
        .on_drag(
            |state: &mut State, drag: &mut Drag, _: &mut Tx| match drag.phase {
                Phase::Down => {}
                Phase::Move => {
                    state.pull_x = drag.dx; // drag moves already land instantly
                    state.pull_y = drag.dy;
                }
                Phase::Up => *state = State::default(), // effect data has no spring: it snaps
            },
        ); // → El
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
        ) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        scenes: &[("pulled", |state| {
            state.pull_x = 40.0;
            state.pull_y = 70.0;
        })],
        ..AppSpec::new(
            "Lesson 7 - Liquid glass",
            (WIDTH, HEIGHT),
            State::default,
            view,
        )
    };
    rune::run_with(spec, Gfx::default());
}
// @end
