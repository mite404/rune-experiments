//! Lesson 7: liquid glass that reads the backdrop. Drag the drop out of the glass pill and
//! watch them stay joined like water until you pull far enough; let go and it springs home.

// @snip:l7
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, Drag, El, Phase, Tx, el, row, text};
use rune_tutorial::gfx::{Gfx, LENS};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    pull: (f32, f32), // how far the drop is dragged from home, pt
}

const W: f32 = 420.0;
const H: f32 = 320.0;
const PILL: (f32, f32, f32, f32) = (110.0, 130.0, 200.0, 56.0); // x, y, w, h
const DROP_R: f32 = 18.0;

// ── 3. CALCULATION ──────────────────────────────────────────
// Where the drop's centre is: just under the pill's right end, plus the pull.
fn drop_centre(pull: (f32, f32)) -> (f32, f32) {
    let (x, y, w, h) = PILL;
    (x + w - 40.0 + pull.0, y + h - 6.0 + pull.1) // → window points
}

fn stripes() -> El {
    let mut r = row().absolute(0.0, 0.0).size(W, H).gap(12.0).justify_center();
    for i in 0..22 {
        let hex = ["#ff5ca8", "#7c5cff", "#22d3ee", "#ffb547"][i % 4];
        r = r.child(el().w(8.0).h_full().fill(rune::color::hex(hex).unwrap()));
    }
    r // → El: something for the glass to bend
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let (x, y, w, h) = PILL;
    let (cx, cy) = drop_centre(s.pull);
    let glass = rune::effect(LENS)
        .effect_data([cx, cy, DROP_R, 0.1]) // the drop: a circle gooey-unioned to the pill
        .effect_backdrop(true) // this slot reads what was drawn before it
        .absolute(x, y).size(w, h).radius(h / 2.0); // → El
    // An invisible handle over the drop, to grab it with.
    let handle = el().key("drop").absolute(cx - DROP_R, cy - DROP_R).size(2.0 * DROP_R, 2.0 * DROP_R)
        .radius(DROP_R)
        .on_drag(|s: &mut State, d: &mut Drag, tx: &mut Tx| match d.phase {
            Phase::Down => {}
            Phase::Move => {
                s.pull = (d.dx, d.dy);
                tx.instant();
            }
            Phase::Up => s.pull = (0.0, 0.0),
        }); // → El
    el().w(W).h(H).fill(rune::color::hex("#0d0a14").unwrap())
        .child(stripes())
        .child(glass)
        .child(handle)
        .child(row().absolute(0.0, 40.0).w(W).center()
            .child(row().h(30.0).px(14.0).radius(15.0).center().fill(t.bg)
                .child(text("Pull the drop out of the glass").font(type_::BODY).color(t.text)))) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        scenes: &[("pulled", |s| s.pull = (40.0, 70.0))],
        ..AppSpec::new("Lesson 7 - Liquid glass", (W, H), State::default, view)
    };
    rune::run_with(spec, Gfx::default());
}
// @end
