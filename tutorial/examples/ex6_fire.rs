//! Exercise 6 stub. Fill every TODO(you). `cargo run --example ex6_fire`.

// @snip:ex6
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, El, col, el, row, text};
use rune_tutorial::gfx::{FIRE, Gfx};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    heat: u32,
}

const W: f32 = 420.0;
const H: f32 = 320.0;
const LOG_Y: f32 = 230.0;

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - heat 0..=3 in, the shader's four numbers out.
fn flames(heat: u32) -> [f32; 4] { // in: u32 → out: [height pt, intensity, 0, 0]
    // TODO(you) 1: low embers at 0, a roaring fire at 3. Turn `heat` into a 0..1 fraction
    //   first (watch out: what does 1 / 3 equal in integer maths?), then scale two ranges.
    let _ = heat;
    todo!() // remove when implemented
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    // TODO(you) 2: an effect slot for FIRE, 200 x 24 pt, at (110, LOG_Y). Give it the numbers
    //   from `flames`. The flames rise far above the slot - what tells Rune to keep drawing
    //   outside the slot's box?
    let fire: El = el(); // replace with the slot
    let log = el().absolute(120.0, LOG_Y + 6.0).size(180.0, 20.0).radius(10.0)
        .fill(rune::color::hex("#3b2416").unwrap());
    let stoke = row().key("stoke").h(32.0).px(16.0).radius(16.0).center().fill(t.control_off)
        .child(text("Stoke the fire").font(type_::LABEL).color(t.text))
        .on_press(|s: &mut State| s.heat = (s.heat + 1) % 4);
    // TODO(you) 3: should the log come before or after the fire in this chain? Predict, run, check.
    el().w(W).h(H).fill(rune::color::hex("#0c0b0d").unwrap())
        .child(fire)
        .child(log)
        .child(col().absolute(0.0, 40.0).w(W).center().gap(10.0)
            .child(text(format!("Heat {} of 3", s.heat)).font(type_::BODY).color(t.text))
            .child(stoke))
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let _ = (FIRE, flames as fn(u32) -> [f32; 4]); // remove when both are used in `view`
    let spec = AppSpec::new("Exercise 6 - Campfire", (W, H), State::default, view);
    rune::run_with(spec, Gfx::default());
}
// @end
