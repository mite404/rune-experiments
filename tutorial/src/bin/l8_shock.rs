//! Lesson 8: time-driven effects. Press the button and a shockwave ripples out through
//! everything drawn under it. `poll` gives the state the clock; the effect reads the press time.

// @snip:l8
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use std::time::Duration;
use rune::{AppSpec, Cx, El, Tx, col, el, row, text};
use rune_tutorial::gfx::{Gfx, SHOCK};

// ── 2. DATA ─────────────────────────────────────────────────
struct State {
    now: f64,      // seconds on Rune's clock, copied in by `poll`
    fired_at: f64, // when the last wave started; far in the past = no wave
}

impl Default for State {
    fn default() -> Self {
        State { now: 0.0, fired_at: -10.0 }
    }
}

const W: f32 = 420.0;
const H: f32 = 320.0;
const BUTTON: (f32, f32, f32, f32) = (150.0, 140.0, 120.0, 40.0);

// ── 3. CALCULATION ──────────────────────────────────────────
fn button_centre() -> (f32, f32) {
    let (x, y, w, h) = BUTTON;
    (x + w / 2.0, y + h / 2.0) // → window points
}

fn tiles() -> El {
    let mut root = el().absolute(0.0, 0.0).size(W, H);
    for i in 0..30 {
        let (x, y) = ((i % 6) as f32 * 70.0 + 6.0, (i / 6) as f32 * 64.0 + 6.0);
        root = root.child(el().absolute(x, y).size(58.0, 52.0).radius(8.0)
            .fill(rune::color::hex("#1d1a24").unwrap()));
    }
    root // → El: a grid for the wave to bend
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let (x, y, w, h) = BUTTON;
    let (cx, cy) = button_centre();
    let button = row().key("boom").absolute(x, y).size(w, h).radius(h / 2.0).center()
        .fill(tok::accent(0, true)).press_scale(0.94)
        .child(text("Boom").font(type_::BODY).color(t.bg))
        .on_press(|s: &mut State| s.fired_at = s.now); // the Action: remember when
    let wave = rune::effect(SHOCK)
        .effect_data([cx, cy, s.fired_at as f32, 14.0]) // [x, y, start time, strength]
        .effect_backdrop(true)
        .absolute(0.0, 0.0).size(W, H); // → El, last so it bends everything above
    el().w(W).h(H).fill(rune::color::hex("#0b0b0d").unwrap())
        .child(tiles())
        .child(col().absolute(0.0, 240.0).w(W).center()
            .child(text("Every tile under the ring bends").font(type_::LABEL).color(t.text_secondary)))
        .child(button)
        .child(wave) // → El
}

// ── 4. ACTIONS ──────────────────────────────────────────────
// Called every frame with Rune's clock. Returns whether anything visible changed.
fn poll(s: &mut State, now: f64, tx: &mut Tx) -> bool {
    s.now = now;
    tx.wake_after(Duration::ZERO); // ask for the next frame, so `now` never goes stale
    false // nothing the view draws changed; the GPU animates the wave on its own
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        poll: Some(poll),
        ..AppSpec::new("Lesson 8 - Shockwave", (W, H), State::default, view)
    };
    rune::run_with(spec, Gfx::default());
}
// @end
