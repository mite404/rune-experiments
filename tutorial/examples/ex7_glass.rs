//! Exercise 7 stub. Fill every TODO(you). `cargo run --example ex7_glass`.

// @snip:ex7
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, Drag, El, Phase, Tx, el, row, text};
use rune_tutorial::gfx::{Gfx, LENS};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    pull: (f32, f32),
}

const W: f32 = 420.0;
const H: f32 = 320.0;
const PILL: (f32, f32, f32, f32) = (110.0, 130.0, 200.0, 56.0);
const DROP_R: f32 = 18.0;

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - the pull in, the drop's centre in window points out.
fn drop_centre(pull: (f32, f32)) -> (f32, f32) { // in: (f32, f32) → out: (f32, f32)
    // TODO(you) 1: at rest the drop hangs just under the pill's right end, half inside it.
    //   Start from PILL's x, y, w, h. Then add the pull.
    let _ = pull;
    todo!() // remove when implemented
}

fn stripes() -> El {
    let mut r = row().absolute(0.0, 0.0).size(W, H).gap(12.0).justify_center();
    for i in 0..22 {
        let hex = ["#ff5ca8", "#7c5cff", "#22d3ee", "#ffb547"][i % 4];
        r = r.child(el().w(8.0).h_full().fill(rune::color::hex(hex).unwrap()));
    }
    r
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let (x, y, w, h) = PILL;
    let (cx, cy) = drop_centre(s.pull);
    // TODO(you) 2: the glass shows nothing behind it. A lens has to SEE the stripes to bend
    //   them - which call lets a slot read what was drawn before it?
    let glass = rune::effect(LENS)
        .effect_data([cx, cy, DROP_R, 0.1])
        .absolute(x, y).size(w, h).radius(h / 2.0);
    let handle = el().key("drop").absolute(cx - DROP_R, cy - DROP_R).size(2.0 * DROP_R, 2.0 * DROP_R)
        .radius(DROP_R)
        .on_drag(|s: &mut State, d: &mut Drag, tx: &mut Tx| match d.phase {
            Phase::Down => {}
            // TODO(you) 3: follow the finger in both axes, with no spring lag. Which `Tx`
            //   call lands a change at once? What happens if you leave it out?
            Phase::Move => { let _ = (s, tx); }
            // TODO(you) 4: let go and the drop springs home.
            Phase::Up => {}
        });
    el().w(W).h(H).fill(rune::color::hex("#0d0a14").unwrap())
        .child(stripes())
        .child(glass)
        .child(handle)
        .child(row().absolute(0.0, 40.0).w(W).center()
            .child(row().h(30.0).px(14.0).radius(15.0).center().fill(t.bg)
                .child(text("Pull the drop out of the glass").font(type_::BODY).color(t.text))))
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::new("Exercise 7 - Liquid glass", (W, H), State::default, view);
    rune::run_with(spec, Gfx::default());
}
// @end
