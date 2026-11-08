//! Exercise 8 stub. Fill every TODO(you). `cargo run --example ex8_shock`.

// @snip:ex8
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, El, Tx, el, row, text};
use rune_tutorial::gfx::{Gfx, SHOCK};

// ── 2. DATA ─────────────────────────────────────────────────
struct State {
    now: f64,
    fired_at: f64,
}

impl Default for State {
    fn default() -> Self {
        // TODO(you) 1: what should `fired_at` start as so that NO wave plays at launch?
        //   (The shader plays a wave for about a second after `fired_at`.)
        State { now: 0.0, fired_at: 0.0 }
    }
}

const W: f32 = 420.0;
const H: f32 = 320.0;
const BUTTON: (f32, f32, f32, f32) = (150.0, 140.0, 120.0, 40.0);

// ── 3. CALCULATION ──────────────────────────────────────────
fn tiles() -> El {
    let mut root = el().absolute(0.0, 0.0).size(W, H);
    for i in 0..30 {
        let (x, y) = ((i % 6) as f32 * 70.0 + 6.0, (i / 6) as f32 * 64.0 + 6.0);
        root = root.child(el().absolute(x, y).size(58.0, 52.0).radius(8.0)
            .fill(rune::color::hex("#1d1a24").unwrap()));
    }
    root
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let (x, y, w, h) = BUTTON;
    // TODO(you) 2: on press, remember WHEN. Handlers get `&mut State` only - no clock.
    //   Where does the current time come from, then?
    let button = row().key("boom").absolute(x, y).size(w, h).radius(h / 2.0).center()
        .fill(tok::accent(0, true)).press_scale(0.94)
        .child(text("Boom").font(type_::BODY).color(t.bg));
    // TODO(you) 3: a SHOCK slot over the whole window, centred on the button, reading the
    //   backdrop. Data is [x, y, start time, strength]. Must it be the first child or the last?
    let wave: El = el();
    let _ = (SHOCK, s.fired_at, s.now);
    el().w(W).h(H).fill(rune::color::hex("#0b0b0d").unwrap())
        .child(tiles())
        .child(button)
        .child(wave)
}

// ── 4. ACTIONS ──────────────────────────────────────────────
// CATEGORY: Action - depends on WHEN it runs (it reads the clock).
fn poll(s: &mut State, now: f64, _: &mut Tx) -> bool { // in: &mut State, f64 → out: bool
    // TODO(you) 4: keep the state's clock current. Rune only draws (and polls) while something
    //   asks for frames - which `Tx` call asks for the next one? Then the return value: the
    //   view only needs rebuilding when something it draws changed. Did it?
    let _ = (s, now);
    todo!() // remove when implemented
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        poll: Some(poll),
        ..AppSpec::new("Exercise 8 - Shockwave", (W, H), State::default, view)
    };
    rune::run_with(spec, Gfx::default());
}
// @end
