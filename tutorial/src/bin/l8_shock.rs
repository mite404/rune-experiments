//! Lesson 8: time-driven effects. Press the button and a shockwave ripples out through
//! everything drawn under it. `poll` gives the state the clock; the effect reads the press time.

// @snip:l8
// ── 1. IMPORTS ──────────────────────────────────────────────
use std::time::Duration;

use rune::tok::{self, type_};
use rune::{AppSpec, Cx, El, Tx};
use rune_tutorial::gfx::{Gfx, SHOCK};

// ── 2. DATA ─────────────────────────────────────────────────
struct State {
    now: f64,      // seconds on Rune's clock, copied in by `poll`
    fired_at: f64, // when the last wave started; far in the past = no wave
}

impl Default for State {
    fn default() -> Self {
        State {
            now: 0.0,
            fired_at: -10.0,
        }
    }
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

const WIDTH: f32 = 420.0;
const HEIGHT: f32 = 320.0;
const BUTTON: Rect = Rect {
    x: 150.0,
    y: 140.0,
    width: 120.0,
    height: 40.0,
};

// ── 3. CALCULATION ──────────────────────────────────────────
fn button_centre() -> (f32, f32) {
    (
        BUTTON.x + BUTTON.width / 2.0,
        BUTTON.y + BUTTON.height / 2.0,
    ) // → window points
}

fn tiles() -> El {
    let mut tiles = rune::el().absolute(0.0, 0.0).size(WIDTH, HEIGHT);
    for i in 0..30 {
        let (x, y) = ((i % 6) as f32 * 70.0 + 6.0, (i / 6) as f32 * 64.0 + 6.0);
        tiles = tiles.child(
            rune::el()
                .absolute(x, y)
                .size(58.0, 52.0)
                .radius(8.0)
                .fill(rune::color::hex("#1d1a24").expect("valid hex colour")),
        );
    }
    tiles // → El: a grid for the wave to bend
}

fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true);
    let (wave_x, wave_y) = button_centre();
    let button = rune::row()
        .key("boom")
        .absolute(BUTTON.x, BUTTON.y)
        .size(BUTTON.width, BUTTON.height)
        .radius(BUTTON.height / 2.0)
        .center()
        .fill(tok::accent(0, true))
        .press_scale(0.94)
        .child(rune::text("Boom").font(type_::BODY).color(theme.bg))
        .on_press(|state: &mut State| state.fired_at = state.now); // the Action: remember when
    let wave = rune::effect(SHOCK)
        .effect_data([wave_x, wave_y, state.fired_at as f32, 14.0]) // [x, y, start time, strength]
        .effect_backdrop(true)
        .absolute(0.0, 0.0)
        .size(WIDTH, HEIGHT); // → El, last so it bends everything above
    rune::el()
        .w(WIDTH)
        .h(HEIGHT)
        .fill(rune::color::hex("#0b0b0d").expect("valid hex colour"))
        .child(tiles())
        .child(
            rune::col().absolute(0.0, 240.0).w(WIDTH).center().child(
                rune::text("Every tile under the ring bends")
                    .font(type_::LABEL)
                    .color(theme.text_secondary),
            ),
        )
        .child(button)
        .child(wave) // → El
}

// ── 4. ACTIONS ──────────────────────────────────────────────
// Called every frame with Rune's clock. Returns whether anything visible changed.
fn poll(state: &mut State, now: f64, tx: &mut Tx) -> bool {
    state.now = now;
    tx.wake_after(Duration::ZERO); // ask for the next frame, so `now` never goes stale
    false // nothing the view draws changed; the GPU animates the wave on its own
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        poll: Some(poll),
        ..AppSpec::new(
            "Lesson 8 - Shockwave",
            (WIDTH, HEIGHT),
            State::default,
            view,
        )
    };
    rune::run_with(spec, Gfx::default());
}
// @end
