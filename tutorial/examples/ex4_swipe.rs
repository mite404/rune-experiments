//! Exercise 4 stub. Fill every TODO(you). `cargo run --example ex4_swipe`.
//! Exercise 5 adds a `#[cfg(test)] mod tests` to the bottom of your finished copy.

// @snip:ex4
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, Theme, radius, type_};
use rune::{AppSpec, Cx, Drag, El, Phase, Transition, Tx, col, text};

// ── 2. DATA ─────────────────────────────────────────────────
struct Card {
    id: u32,
    title: &'static str,
    body: &'static str,
    dx: f32,
}

struct State {
    cards: Vec<Card>,
}

const THRESHOLD: f32 = 110.0;

const NOTES: [(&str, &str); 4] = [
    ("Build finished", "main · 2m 14s · all checks passed"),
    ("New comment", "\"Ship it, the spring feels right\""),
    ("Calendar", "Design review in 15 minutes"),
    ("Battery low", "Plug in soon - 12% left"),
];

impl Default for State {
    fn default() -> Self {
        let cards = NOTES.iter().zip(0..)
            .map(|(&(title, body), id)| Card { id, title, body, dx: 0.0 })
            .collect();
        State { cards }
    }
}

#[derive(Debug, PartialEq)]
enum Release {
    Archive,
    SnapBack,
}

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - distance travelled in, a decision out. No State, no Drag.
fn release(dx: f32) -> Release { // in: f32 → out: Release
    // TODO(you) 1: archive past THRESHOLD, otherwise snap back. A swipe LEFT has a negative
    //   dx. Does your comparison still archive it? Exactly AT the threshold - which side?
    let _ = dx;
    todo!() // remove when implemented
}

fn card(c: &Card, t: Theme) -> El {
    let id = c.id;
    let away = if c.dx < 0.0 { -420.0 } else { 420.0 };
    col().key(format!("card-{id}")).w_full().h(64.0).px(16.0).gap(4.0).justify_center()
        .radius(radius::LG).fill(t.card)
        .offset(c.dx, 0.0)
        .transition(Transition::asymmetric(Transition::opacity(), Transition::offset(away, 0.0)))
        .child(text(c.title).font(type_::BODY).color(t.text))
        .child(text(c.body).font(type_::LABEL).color(t.text_secondary))
        .on_drag(move |s: &mut State, d: &mut Drag, _tx: &mut Tx| {
            let Some(i) = s.cards.iter().position(|c| c.id == id) else { return };
            match d.phase {
                Phase::Down => {}
                // TODO(you) 2: while the finger moves, the card follows it. Which field of `d`
                //   says how far the pointer travelled since the press?
                Phase::Move => {}
                // TODO(you) 3: on let-go, ASK `release` what happened, then change state for
                //   each answer. Why should the decision live in `release` and not here?
                Phase::Up => { let _ = i; }
            }
        })
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let mut list = col().w_full().gap(8.0);
    for c in &s.cards {
        list = list.child(card(c, t));
    }
    col().w_full().h_full().padding(20.0).pt(40.0).gap(14.0).fill(t.bg)
        .child(text("Swipe to archive").font(type_::TITLE).color(t.text))
        .child(list)
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let _ = release; // remove when `release` is called from the drag handler
    rune::run(AppSpec::new("Exercise 4 - Swipe", (400.0, 380.0), State::default, view));
}
// @end
