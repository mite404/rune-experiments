//! Lesson 2: layout with rows and columns, and keys that give each row an identity.
//! Press "Play next" on any track and it moves to the top; keyed rows slide to their new places.

// @snip:l2
// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::tok::{self, Theme, motion, radius, type_};
use rune::{AppSpec, Cx, El, col, row, text};

// ── 2. DATA ─────────────────────────────────────────────────
#[derive(Clone, Copy)]
struct Track {
    id: u32, // never changes, so it can name the row
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
// A new queue with track `id` at the front. Everything else keeps its order.
fn play_next(queue: &[Track], id: u32) -> Vec<Track> {
    let picked = queue.iter().filter(|t| t.id == id); // → the one track
    let rest = queue.iter().filter(|t| t.id != id); // → everyone else, in order
    picked.chain(rest).copied().collect() // → Vec<Track>
}

// One row: title and artist stacked on the left, length and a button on the right.
fn track_row(track: &Track, t: Theme) -> El {
    let id = track.id;
    let words = col().grow(1.0).gap(2.0) // grow: take the free width, push the rest right
        .child(text(track.title).font(type_::BODY).color(t.text))
        .child(text(track.artist).font(type_::LABEL).color(t.text_secondary)); // → El
    let button = row().key("next").h(26.0).px(10.0).radius(13.0).center()
        .fill(t.control_off)
        .child(text("Play next").font(type_::LABEL).color(t.text))
        .on_press(move |s: &mut State| s.queue = play_next(&s.queue, id)); // the Action: replace the queue
    row().h(52.0).px(14.0).gap(12.0).center_y().radius(radius::LG).fill(t.card)
        .child(words)
        .child(text(track.length).font(type_::LABEL).color(t.text_secondary))
        .child(button) // → El
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let mut list = col().gap(6.0); // → El
    for track in &s.queue {
        list = list.child(track_row(track, t).key(format!("track-{}", track.id))); // the identity
    }
    col().w_full().h_full().padding(20.0).pt(40.0).gap(14.0).fill(t.bg) // pt: clear the titlebar
        .motion(motion::SMOOTH)
        .child(text("Up next").font(type_::TITLE).color(t.text))
        .child(list) // → El
}

// ── 5. MAIN ─────────────────────────────────────────────────
fn main() {
    rune::run(AppSpec::new("Lesson 2 - Up next", (420.0, 400.0), State::default, view));
}
// @end
