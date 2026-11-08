//! Lesson 6: a GPU effect. A campfire whose flames grow each time you stoke it.
//! The view only marks out a slot; the compositor (`rune_tutorial::gfx`) paints fire into it.

// @snip:l6
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, El, col, el, row, text};
use rune_tutorial::gfx::{FIRE, Gfx};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    heat: u32, // 0 = embers, 3 = roaring
}

const W: f32 = 420.0;
const H: f32 = 320.0;
const LOG_Y: f32 = 230.0;

// ── 3. CALCULATION ──────────────────────────────────────────
// The numbers the fire shader reads, from how hot the fire is.
fn flames(heat: u32) -> [f32; 4] {
    let k = heat as f32 / 3.0; // → 0.0..=1.0
    [24.0 + 110.0 * k, 0.5 + 0.8 * k, 0.0, 0.0] // → [height pt, intensity, unused, unused]
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let fire = rune::effect(FIRE) // a box that draws nothing; the compositor paints here
        .effect_data(flames(s.heat)) // four numbers for the shader
        .effect_bleed(200.0) // flames rise past the slot's top edge
        .absolute(110.0, LOG_Y)
        .size(200.0, 24.0); // → El
    let log = el().absolute(120.0, LOG_Y + 6.0).size(180.0, 20.0).radius(10.0)
        .fill(rune::color::hex("#3b2416").unwrap()); // → El
    let stoke = row().key("stoke").h(32.0).px(16.0).radius(16.0).center().fill(t.control_off)
        .child(text(if s.heat < 3 { "Stoke the fire" } else { "Let it settle" }).font(type_::LABEL).color(t.text))
        .on_press(|s: &mut State| s.heat = (s.heat + 1) % 4); // → El
    el().w(W).h(H).fill(rune::color::hex("#0c0b0d").unwrap())
        .child(fire)
        .child(log) // drawn after the fire, so the log sits in front of the flames
        .child(col().absolute(0.0, 40.0).w(W).center().gap(10.0)
            .child(text(format!("Heat {} of 3", s.heat)).font(type_::BODY).color(t.text))
            .child(stoke)) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        scenes: &[("roaring", |s| s.heat = 3)],
        ..AppSpec::new("Lesson 6 - Campfire", (W, H), State::default, view)
    };
    rune::run_with(spec, Gfx::default()); // run, plus a compositor for the effect slots
}
// @end
