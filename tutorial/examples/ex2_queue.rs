//! Exercise 2 stub. Copy this over `src/bin/my_queue.rs` (or edit it here) and fill every TODO(you).
//! `cargo run --example ex2_queue` compiles it now; it panics at the first `todo!()` until you finish.

// @snip:ex2
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, Theme, motion, radius, type_};
use rune::{AppSpec, Cx, El, col, row, text};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Clone, Copy)]
struct Track {
    id: u32,
    title: &'static str,
    artist: &'static str,
    length: &'static str,
}

struct State {
    queue: Vec<Track>,
}

const TRACKS: [(&str, &str, &str); 5] = [
    ("Neon Rain", "Kiko Moss", "3:41"),
    ("Paper Planes", "The Lanterns", "2:58"),
    ("Slow Orbit", "Mira Vale", "4:12"),
    ("Glass Garden", "Odd Weather", "3:05"),
    ("Last Light", "Juno Park", "3:33"),
];

impl Default for State {
    fn default() -> Self {
        let queue = TRACKS.iter().zip(0..)
            .map(|(&(title, artist, length), id)| Track { id, title, artist, length })
            .collect(); // → Vec<Track>
        State { queue }
    }
}

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - a queue and an id in, a NEW queue out. No mutation.
fn play_next(queue: &[Track], id: u32) -> Vec<Track> { // in: &[Track], u32 → out: Vec<Track>
    // TODO(you) 1: put the track with `id` first and keep everyone else in their order.
    //   You can't mutate `queue` (it's borrowed, not owned). Which iterator adapters let you
    //   pick "the one" and "the rest" as two passes over the same slice, then join them?
    //   What does `Track` need to derive so you can turn `&Track` back into `Track`?
    let _ = (queue, id);
    todo!() // remove when implemented
}

// CATEGORY: Calculation - one track in, one row of elements out.
fn track_row(track: &Track, t: Theme) -> El { // in: &Track, Theme → out: El
    let id = track.id;
    // TODO(you) 2: title above artist. Which builder stacks children vertically? It must also
    //   take all the spare width so the length and button get pushed to the right edge -
    //   which flex property does that in CSS, and what is it called here?
    let words: El = todo!(); // remove when implemented
    let button = row().key("next").h(26.0).px(10.0).radius(13.0).center()
        .fill(t.control_off)
        .child(text("Play next").font(type_::LABEL).color(t.text))
        .on_press(move |s: &mut State| s.queue = play_next(&s.queue, id));
    row().h(52.0).px(14.0).gap(12.0).center_y().radius(radius::LG).fill(t.card)
        .child(words)
        .child(text(track.length).font(type_::LABEL).color(t.text_secondary))
        .child(button)
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let mut list = col().gap(6.0);
    for track in &s.queue {
        // TODO(you) 3: give every row an identity that survives reordering. What is stable
        //   about a track that its position in the list is not? Try it with the row's index
        //   first, press "Play next", and watch what moves. Then fix it.
        list = list.child(track_row(track, t));
    }
    col().w_full().h_full().padding(20.0).pt(40.0).gap(14.0).fill(t.bg)
        .motion(motion::SMOOTH)
        .child(text("Up next").font(type_::TITLE).color(t.text))
        .child(list)
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    rune::run(AppSpec::new("Exercise 2 - Up next", (420.0, 400.0), State::default, view));
}
// @end
