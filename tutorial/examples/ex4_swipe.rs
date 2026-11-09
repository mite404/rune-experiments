//! Exercise 4 stub. Fill every TODO(you). `cargo run --example ex4_swipe`.
//! Exercise 5 adds a `#[cfg(test)] mod tests` to the bottom of your finished copy.

// @snip:ex4
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, Theme, radius, type_};
use rune::{AppSpec, Cx, Drag, El, Phase, Transition, Tx};

// ── 2. DATA ─────────────────────────────────────────────────
struct Card {
    id: u32,
    title: &'static str,
    body: &'static str,
    dx: f32, // how far it's being dragged sideways, pt; 0 at rest
}

struct State {
    cards: Vec<Card>,
}

const THRESHOLD: f32 = 110.0; // let go past this and the card is archived

const NOTES: [(&str, &str); 4] = [
    ("Build finished", "main · 2m 14s · all checks passed"),
    ("New comment", "\"Ship it, the spring feels right\""),
    ("Calendar", "Design review in 15 minutes"),
    ("Battery low", "Plug in soon - 12% left"),
];

fn fresh_cards() -> Vec<Card> {
    NOTES
        .iter()
        .zip(0..)
        .map(|(&(title, body), id)| Card {
            id,
            title,
            body,
            dx: 0.0,
        })
        .collect() // → Vec<Card>
}

impl Default for State {
    fn default() -> Self {
        State {
            cards: fresh_cards(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Release {
    Archive,
    SnapBack,
}

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - distance travelled in, a decision out. No State, no Drag.
fn release(dx: f32) -> Release {
    // in: f32 → out: Release
    // TODO(you) 1: archive past THRESHOLD, otherwise snap back. A swipe LEFT has a negative
    //   dx. Does your comparison still archive it? Exactly AT the threshold - which side?
    let _ = dx;
    todo!() // remove when implemented
}

// The card fades as it nears the threshold, so the hand can feel the commit point coming.
fn fade(dx: f32) -> f32 {
    1.0 - 0.6 * (dx.abs() / THRESHOLD).min(1.0) // → opacity 0.4..=1.0
}

fn card(note: &Card, theme: Theme) -> El {
    let id = note.id;
    let away = if note.dx < 0.0 { -420.0 } else { 420.0 }; // leave the way it was thrown
    rune::col()
        .key(format!("card-{id}"))
        .w_full()
        .h(64.0)
        .px(16.0)
        .gap(4.0)
        .justify_center()
        .radius(radius::LG)
        .fill(theme.card)
        .offset(note.dx, 0.0) // drawn shifted; layout (and its neighbours) stay put
        .opacity(fade(note.dx))
        .transition(Transition::asymmetric(
            Transition::opacity(),
            Transition::offset(away, 0.0),
        ))
        .child(rune::text(note.title).font(type_::BODY).color(theme.text))
        .child(
            rune::text(note.body)
                .font(type_::LABEL)
                .color(theme.text_secondary),
        )
        .on_drag(move |state: &mut State, drag: &mut Drag, _tx: &mut Tx| {
            let Some(i) = state.cards.iter().position(|card| card.id == id) else {
                return;
            }; // → usize
            match drag.phase {
                Phase::Down => {}
                // TODO(you) 2: while the finger moves, the card follows it. Which field of `drag`
                //   says how far the pointer travelled since the press?
                Phase::Move => {}
                // TODO(you) 3: on let-go, ASK `release` what happened, then change state for
                //   each answer. Why should the decision live in `release` and not here?
                Phase::Up => {
                    let _ = i;
                }
            }
        }) // → El
}

fn view(state: &State, _: &mut Cx<State>) -> El {
    let theme = tok::theme(true);
    let mut list = rune::col().w_full().gap(8.0);
    for note in &state.cards {
        list = list.child(card(note, theme));
    }
    if state.cards.is_empty() {
        list = list.child(
            rune::row()
                .key("refill")
                .h(36.0)
                .px(14.0)
                .radius(18.0)
                .center()
                .fill(theme.control_off)
                .child(
                    rune::text("All clear. Bring them back")
                        .font(type_::LABEL)
                        .color(theme.text),
                )
                .on_press(|state: &mut State| state.cards = fresh_cards()),
        );
    }
    rune::col()
        .w_full()
        .h_full()
        .padding(20.0)
        .pt(40.0)
        .gap(14.0)
        .fill(theme.bg) // pt: clear the titlebar
        .child(
            rune::text("Swipe to archive")
                .font(type_::TITLE)
                .color(theme.text),
        )
        .child(list) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    let _ = release; // remove when `release` is called from the drag handler
    rune::run(AppSpec::new(
        "Exercise 4 - Swipe",
        (400.0, 380.0),
        State::default,
        view,
    ));
}
// @end
