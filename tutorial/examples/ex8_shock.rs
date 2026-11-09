//! Exercise 8 stub. Fill every TODO(you). `cargo run --example ex8_shock`.

// @snip:ex8
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, type_};
use rune::{AppSpec, Cx, El, Tx};
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
        State {
            now: 0.0,
            fired_at: 0.0,
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
    tiles
}

fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true);
    // TODO(you) 2: on press, remember WHEN. Handlers get `&mut State` only - no clock.
    //   Where does the current time come from, then?
    let button = rune::row()
        .key("boom")
        .absolute(BUTTON.x, BUTTON.y)
        .size(BUTTON.width, BUTTON.height)
        .radius(BUTTON.height / 2.0)
        .center()
        .fill(tok::accent(0, true))
        .press_scale(0.94)
        .child(rune::text("Boom").font(type_::BODY).color(theme.bg));
    // TODO(you) 3: a SHOCK slot over the whole window, centred on the button, reading the
    //   backdrop. Data is [x, y, start time, strength]. Must it be the first child or the last?
    let wave: El = rune::el();
    let _ = (SHOCK, state.fired_at, state.now);
    rune::el()
        .w(WIDTH)
        .h(HEIGHT)
        .fill(rune::color::hex("#0b0b0d").expect("valid hex colour"))
        .child(tiles())
        .child(button)
        .child(wave)
}

// ── 4. ACTIONS ──────────────────────────────────────────────
// CATEGORY: Action - depends on WHEN it runs (it reads the clock).
fn poll(state: &mut State, now: f64, tx: &mut Tx) -> bool {
    // in: &mut State, f64 → out: bool
    // TODO(you) 4: keep the state's clock current. Rune only draws (and polls) while something
    //   asks for frames - which `Tx` call asks for the next one? Then the return value: the
    //   view only needs rebuilding when something it draws changed. Did it?
    let _ = (state, now, tx);
    todo!() // remove when implemented
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let spec = AppSpec::<State> {
        poll: Some(poll),
        ..AppSpec::new(
            "Exercise 8 - Shockwave",
            (WIDTH, HEIGHT),
            State::default,
            view,
        )
    };
    rune::run_with(spec, Gfx::default());
}
// @end
