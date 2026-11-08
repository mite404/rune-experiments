//! Lesson 4 (and the tests of Lesson 5): drag a notification sideways. Let go past the
//! threshold and it flies off; let go short of it and it springs home.

// @snip:l4
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, Theme, radius, type_};
use rune::{AppSpec, Cx, Drag, El, Phase, Transition, Tx, col, row, text};

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
    NOTES.iter().zip(0..)
        .map(|(&(title, body), id)| Card { id, title, body, dx: 0.0 })
        .collect() // → Vec<Card>
}

impl Default for State {
    fn default() -> Self {
        State { cards: fresh_cards() }
    }
}

#[derive(Debug, PartialEq)]
enum Release {
    Archive,
    SnapBack,
}

// ── 3. CALCULATION ──────────────────────────────────────────
// What a let-go means, from how far the card travelled. Either direction counts.
// @snip:l4-release
fn release(dx: f32) -> Release {
    if dx.abs() >= THRESHOLD { Release::Archive } else { Release::SnapBack } // → Release
}
// @end

// The card fades as it nears the threshold, so the hand can feel the commit point coming.
fn fade(dx: f32) -> f32 {
    1.0 - 0.6 * (dx.abs() / THRESHOLD).min(1.0) // → opacity 0.4..=1.0
}

fn card(c: &Card, t: Theme) -> El {
    let id = c.id;
    let away = if c.dx < 0.0 { -420.0 } else { 420.0 }; // leave the way it was thrown
    col().key(format!("card-{id}")).w_full().h(64.0).px(16.0).gap(4.0).justify_center()
        .radius(radius::LG).fill(t.card)
        .offset(c.dx, 0.0) // drawn shifted; layout (and its neighbours) stay put
        .opacity(fade(c.dx))
        .transition(Transition::asymmetric(Transition::opacity(), Transition::offset(away, 0.0)))
        .child(text(c.title).font(type_::BODY).color(t.text))
        .child(text(c.body).font(type_::LABEL).color(t.text_secondary))
        // @snip:l4-drag
        .on_drag(move |s: &mut State, d: &mut Drag, _tx: &mut Tx| {
            let Some(i) = s.cards.iter().position(|c| c.id == id) else { return }; // → usize
            match d.phase {
                Phase::Down => {}
                Phase::Move => s.cards[i].dx = d.dx, // follows the hand, no spring
                Phase::Up => match release(d.dx) {
                    Release::Archive => { s.cards.remove(i); } // exit transition flies it off
                    Release::SnapBack => s.cards[i].dx = 0.0, // springs home
                },
            }
        }) // → El
        // @end
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let mut list = col().w_full().gap(8.0);
    for c in &s.cards {
        list = list.child(card(c, t));
    }
    if s.cards.is_empty() {
        list = list.child(
            row().key("refill").h(36.0).px(14.0).radius(18.0).center().fill(t.control_off)
                .child(text("All clear. Bring them back").font(type_::LABEL).color(t.text))
                .on_press(|s: &mut State| s.cards = fresh_cards()),
        );
    }
    col().w_full().h_full().padding(20.0).pt(40.0).gap(14.0).fill(t.bg) // pt: clear the titlebar
        .child(text("Swipe to archive").font(type_::TITLE).color(t.text))
        .child(list) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    rune::run(AppSpec::new("Lesson 4 - Swipe", (400.0, 380.0), State::default, view));
}
// @end

// @snip:l5
#[cfg(test)]
mod tests {
    use super::*;
    use rune::{Env, Runtime};

    // ── DATA ────────────────────────────────────────────────
    const SIZE: (f32, f32) = (400.0, 380.0);
    const DT: f64 = 1.0 / 60.0;

    // ── ACTIONS (on a fake window, not a real one) ──────────
    fn app() -> Runtime<State> {
        let env = Env { system_dark: true, scale: 2.0 };
        let mut rt = Runtime::new(State::default(), view, SIZE, env); // → Runtime<State>
        rt.frame(0.0); // build and lay out the first frame
        rt
    }

    // Press the element keyed `key`, move `dx` points over 10 frames, let go, let it settle.
    fn drag(rt: &mut Runtime<State>, key: &str, dx: f32) {
        let (x, y) = rt.find(key).expect("element on screen"); // → its centre, in points
        rt.pointer_down(x, y);
        for step in 1..=10 {
            rt.pointer_move(x + dx * step as f32 / 10.0, y);
            rt.frame(DT);
        }
        rt.pointer_up(x + dx, y);
        for _ in 0..60 {
            rt.frame(DT); // one second of springs
        }
    }

    // ── TESTS ───────────────────────────────────────────────
    #[test]
    fn release_archives_at_the_threshold_in_either_direction() {
        assert_eq!(release(THRESHOLD - 1.0), Release::SnapBack);
        assert_eq!(release(THRESHOLD), Release::Archive);
        assert_eq!(release(-THRESHOLD), Release::Archive);
    }

    #[test]
    fn a_short_drag_springs_home() {
        let mut rt = app();
        let home = rt.presented("card-0").unwrap().0[0]; // → x of the drawn rect
        drag(&mut rt, "card-0", 60.0);
        assert_eq!(rt.state.cards.len(), 4, "nothing archived");
        let now = rt.presented("card-0").unwrap().0[0];
        assert!((now - home).abs() < 0.5, "back home: {home} → {now}");
    }

    #[test]
    fn a_long_drag_archives_the_card() {
        let mut rt = app();
        drag(&mut rt, "card-0", -180.0);
        let ids: Vec<u32> = rt.state.cards.iter().map(|c| c.id).collect(); // → [1, 2, 3]
        assert_eq!(ids, [1, 2, 3]);
        assert!(rt.find("card-0").is_none(), "gone from the screen too");
    }
}
// @end
