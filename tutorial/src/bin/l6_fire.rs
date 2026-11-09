//! Lesson 6: a GPU effect. A campfire whose flames grow each time you stoke it.
//! The view only marks out a slot; the compositor (`rune_tutorial::gfx`) paints fire into it.

// @snip:l6
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, El};
use rune_tutorial::gfx::{FIRE, Gfx};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Default)]
struct State {
    heat: u32, // 0 = embers, 3 = roaring
}

const WIDTH: f32 = 420.0;
const HEIGHT: f32 = 320.0;
const LOG_Y: f32 = 230.0;

// ── 3. CALCULATION ──────────────────────────────────────────
// The numbers the fire shader reads, from how hot the fire is.
fn flames(heat: u32) -> [f32; 4] {
    let fraction = heat as f32 / 3.0; // → 0.0..=1.0
    [24.0 + 110.0 * fraction, 0.5 + 0.8 * fraction, 0.0, 0.0] // → [height pt, intensity, unused, unused]
}

fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true);
    let fire = rune::effect(FIRE) // a box that draws nothing; the compositor paints here
        .effect_data(flames(state.heat)) // four numbers for the shader
        .effect_bleed(200.0) // flames rise past the slot's top edge
        .absolute(110.0, LOG_Y)
        .size(200.0, 24.0); // → El
    let log = rune::el()
        .absolute(120.0, LOG_Y + 6.0)
        .size(180.0, 20.0)
        .radius(10.0)
        .fill(rune::color::hex("#3b2416").expect("valid hex colour")); // → El
    let stoke = rune::row()
        .key("stoke")
        .h(32.0)
        .px(16.0)
        .radius(16.0)
        .center()
        .fill(theme.control_off)
        .child(
            rune::text(if state.heat < 3 {
                "Stoke the fire"
            } else {
                "Let it settle"
            })
            .font(type_::LABEL)
            .color(theme.text),
        )
        .on_press(|state: &mut State| state.heat = (state.heat + 1) % 4); // → El
    rune::el()
        .w(WIDTH)
        .h(HEIGHT)
        .fill(rune::color::hex("#0c0b0d").expect("valid hex colour"))
        .child(fire)
        .child(log) // drawn after the fire, so the log sits in front of the flames
        .child(
            rune::col()
                .absolute(0.0, 40.0)
                .w(WIDTH)
                .center()
                .gap(10.0)
                .child(
                    rune::text(format!("Heat {} of 3", state.heat))
                        .font(type_::BODY)
                        .color(theme.text),
                )
                .child(stoke),
        ) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        scenes: &[("roaring", |state| state.heat = 3)],
        ..AppSpec::new("Lesson 6 - Campfire", (WIDTH, HEIGHT), State::default, view)
    };
    rune::run_with(spec, Gfx::default()); // run, plus a compositor for the effect slots
}
// @end
