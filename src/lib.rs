//! Rune experiments: six small stages, each one idea. They share one 960 × 600 stage, one
//! state and one effects compositor (`gfx`). The app opens on a grid of live previews; a tile
//! opens its experiment full size. Headless, `--scene NAME` opens one directly.

mod gfx;

use rune::color::Color;
use rune::{AppSpec, Cx, Drag, El, Key, KeyEvent, Phase, TextStyle, Tx, col, el, row, spacer, text};

use gfx::{CHROME, ELECTRIC, FIRE, GLOW, GOO, LAVA, LENS, MESH, SHOCK, SPARKLES};

const W: f32 = 960.0;
const H: f32 = 600.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exp {
    LavaTodo,
    GradientCrawl,
    MetalDrip,
    WaterDrip,
    HoldConfirm,
    GooTabs,
}

/// Scene name, experiment, title, one-line blurb: the grid's tiles in order.
const EXPERIMENTS: [(&str, Exp, &str, &str); 6] = [
    ("lava-todo", Exp::LavaTodo, "The floor is lava", "Drag a todo into the lava, or tick it"),
    ("gradient-crawl", Exp::GradientCrawl, "Gradient crawl", "A view transition that crawls up the screen"),
    ("metal-drip", Exp::MetalDrip, "Liquid metal drip", "A chrome next step that drips to get noticed"),
    ("water-drip", Exp::WaterDrip, "Water drip", "A glass button beading a drop off its edge"),
    ("hold-confirm", Exp::HoldConfirm, "Hold to delete", "Charges up, then a shockwave"),
    ("goo-tabs", Exp::GooTabs, "Goo tabs", "A glass indicator that stretches like water"),
];

struct Todo {
    /// Index into `TASKS`.
    id: u32,
    /// How far it's being dragged down, pt.
    dy: f32,
    /// Drag speed down, pt/s, sampled at `at`: a flick drops it before it reaches the lava.
    vy: f32,
    at: f64,
}

/// A todo on its way into the lava: its top at `y` when it let go, at time `t0`.
struct Dropped {
    id: u32,
    y: f32,
    t0: f64,
}

pub struct State {
    /// The experiment open full size; `None` is the grid.
    exp: Option<Exp>,
    /// Seconds since the app started (Rune's virtual clock when headless).
    t: f64,
    todos: Vec<Todo>,
    dropped: Option<Dropped>,
    /// Gradient crawl: the step on screen, and the transition in flight (start time, target).
    step: usize,
    crawl: Option<(f64, usize)>,
    swapped_at: f64,
    /// When the open experiment was opened, and when its button was last pressed.
    opened_at: f64,
    pressed_at: f64,
    /// Hold to confirm: when the current hold began, when the last one went off, and when a
    /// release started draining the charge (time, charge at release).
    held_from: Option<f64>,
    fired_at: f64,
    drain: (f64, f32),
    /// Goo tabs: the tab selected, where the pill's leading and trailing edges set off from, and when.
    tab: usize,
    goo_from: [f32; 2],
    tab_at: f64,
}

impl Default for State {
    fn default() -> Self {
        State {
            exp: None,
            t: 0.0,
            todos: fresh_todos(),
            dropped: None,
            step: 0,
            crawl: None,
            swapped_at: -10.0,
            opened_at: -10.0,
            pressed_at: -10.0,
            held_from: None,
            fired_at: -10.0,
            drain: (-10.0, 0.0),
            tab: 0,
            goo_from: [tab_x(0); 2],
            tab_at: -10.0,
        }
    }
}

// ------------------------------------------------------------------ look

fn c(hex: u32) -> Color {
    rune::color::hex(&format!("#{hex:06x}")).unwrap_or(Color::WHITE)
}

fn ca(hex: u32, a: f32) -> Color {
    c(hex).alpha(a.clamp(0.0, 1.0))
}

fn t(s: impl Into<String>, size: f32, weight: f32, color: u32) -> El {
    text(s).font(TextStyle::new(size, weight)).color(c(color))
}

fn fx(kind: u32, data: [f32; 4]) -> El {
    rune::effect(kind).effect_data(data)
}

fn smooth(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// Strong ease-out for things entering or leaving: fast start, soft landing.
fn ease_out(x: f32) -> f32 {
    1.0 - (1.0 - x.clamp(0.0, 1.0)).powi(4)
}

fn ease_in_out(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    if x < 0.5 { 4.0 * x * x * x } else { 1.0 - (-2.0 * x + 2.0).powi(3) / 2.0 }
}

// ------------------------------------------------------------------ the floor is lava

const LIST_X: f32 = 220.0;
const LIST_Y: f32 = 104.0;
const LIST_W: f32 = 520.0;
const ROW_H: f32 = 46.0;
const ROW_GAP: f32 = 8.0;
const HORIZON: f32 = 404.0;
const FALL_G: f32 = 2200.0;
/// A release moving down faster than this, pt/s, drops the todo wherever it is.
const FLICK_V: f32 = 700.0;

const TASKS: [(&str, &str, &str, u32); 5] = [
    ("Ship the release notes", "Launch · due today", "Eng", 0x4f8dff),
    ("Review PR #842: layout cache", "Rune · 3 comments", "Eng", 0x4f8dff),
    ("Reply to the legal thread", "Inbox · since Tuesday", "Ops", 0xffb547),
    ("Redesign the empty state", "Design · in progress", "Design", 0xff5ca8),
    ("Book flights for the offsite", "Personal · this week", "Personal", 0x34d399),
];

fn fresh_todos() -> Vec<Todo> {
    (0..TASKS.len() as u32).map(|id| Todo { id, dy: 0.0, vy: 0.0, at: 0.0 }).collect()
}

/// When a todo dropped at height `y` (its top, at rest) at `t0` hits the lava.
fn hit_time(d: &Dropped) -> f64 {
    d.t0 + ((2.0 * (HORIZON - d.y - ROW_H * 0.5).max(0.0) / FALL_G) as f64).sqrt()
}

fn drop_todo(s: &mut State, id: u32) {
    let Some(i) = s.todos.iter().position(|t| t.id == id) else { return };
    let y = LIST_Y + i as f32 * (ROW_H + ROW_GAP) + s.todos[i].dy;
    s.todos.remove(i);
    s.dropped = Some(Dropped { id, y, t0: s.t });
}

fn task_card(id: u32) -> El {
    let (title, meta, tag, tint) = TASKS[id as usize];
    row().w(LIST_W).h(ROW_H).px(14.0).gap(12.0).center_y().radius(10.0)
        .fill(ca(0x1c1a1f, 0.92)).border(1.0, ca(0xffffff, 0.06))
        .child(el().key("check").size(16.0, 16.0).radius(8.0).border(1.5, ca(0xffffff, 0.3))
            .on_press(move |s: &mut State| drop_todo(s, id)))
        .child(col().grow(1.0).gap(2.0)
            .child(t(title, 13.5, 600.0, 0xededef))
            .child(t(meta, 11.5, 450.0, 0x7b7b84)))
        .child(row().h(20.0).px(8.0).radius(6.0).center().fill(ca(tint, 0.16)).child(t(tag, 11.0, 600.0, tint)))
        .child(t("⋮⋮", 11.0, 600.0, 0x4a4a52))
}

fn lava_todo(s: &State, rm: bool) -> El {
    let mut list = col().key("list").absolute(LIST_X, LIST_Y).w(LIST_W).gap(ROW_GAP);
    for todo in &s.todos {
        let id = todo.id;
        let mut card = task_card(id).key(format!("todo-{id}")).offset(0.0, todo.dy).motion(rune::tok::motion::SMOOTH);
        if todo.dy != 0.0 {
            card = card.raise().shadow(12.0, 30.0, ca(0, 0.5));
        }
        list = list.child(card
            .on_drag(move |s: &mut State, d: &mut Drag, tx: &mut Tx| {
                let Some(i) = s.todos.iter().position(|t| t.id == id) else { return };
                match d.phase {
                    Phase::Down => (s.todos[i].vy, s.todos[i].at) = (0.0, s.t),
                    Phase::Move => {
                        let (now, todo) = (s.t, &mut s.todos[i]);
                        let dt = (now - todo.at) as f32;
                        if dt > 0.0 {
                            (todo.vy, todo.at) = ((d.dy - todo.dy) / dt, now);
                        }
                        todo.dy = d.dy;
                        tx.instant();
                    }
                    Phase::Up => {
                        let bottom = LIST_Y + i as f32 * (ROW_H + ROW_GAP) + d.dy + ROW_H;
                        // A pause before letting go isn't a flick.
                        let flick = d.dy > 12.0 && s.todos[i].vy > FLICK_V && s.t - s.todos[i].at < 0.1;
                        if bottom > HORIZON - 8.0 || flick { drop_todo(s, id) } else { s.todos[i].dy = 0.0 }
                    }
                }
            }));
    }
    // Flames flare where a todo went in, then settle back to a low burn.
    let flare = s.dropped.as_ref().map_or(0.0, |d| {
        let k = (s.t - hit_time(d)) as f32;
        if k < 0.0 { 0.0 } else { (k / 0.12).min(1.0) * (-k * 0.9).exp() }
    });
    let n = s.todos.len();
    let mut root = el().w(W).h(H).fill(c(0x0c0b0d))
        .child(row().absolute(LIST_X, 36.0).w(LIST_W).items_end()
            .child(col().gap(2.0)
                .child(t("Today", 24.0, 650.0, 0xededef))
                .child(t(format!("{n} tasks · the floor is lava"), 12.0, 450.0, 0x7b7b84)))
            .child(spacer())
            .child(t("Friday, Nov 6", 11.5, 450.0, 0x5f5f68)))
        .child(list);
    if n == 0 {
        root = root.child(row().key("refill").absolute(LIST_X, LIST_Y).h(36.0).px(14.0).radius(10.0).center()
            .fill(ca(0xffffff, 0.06)).hover(ca(0xffffff, 0.1))
            .child(t("Nothing left to burn. Refill the list", 13.0, 550.0, 0xff9a5a))
            .on_press(|s: &mut State| s.todos = fresh_todos()));
    }
    if let Some(d) = s.dropped.as_ref().filter(|_| !rm) {
        let k = (s.t - d.t0) as f32;
        let y = d.y + 0.5 * FALL_G * k * k;
        if y < HORIZON + 10.0 {
            root = root.child(task_card(d.id).key(format!("drop-{}", d.id)).absolute(LIST_X, y)
                .scale(1.0 - 0.04 * (k * 6.0).min(1.0)).shadow(10.0, 30.0, ca(0xff5a1f, 0.35)));
        }
    }
    // The floor: cooled crust split by glowing seams, a hot horizon line, flames and embers.
    root.child(fx(LAVA, [0.0; 4]).absolute(0.0, HORIZON).size(W, H - HORIZON))
        .child(el().absolute(0.0, HORIZON - 1.0).size(W, 2.0).fill(c(0xffb547)))
        .child(fx(GLOW, [0.0, 0xff5a1f as f32, 0.3 + 0.6 * flare, 40.0 + 40.0 * flare]).absolute(0.0, HORIZON - 1.0).size(W, 2.0).effect_bleed(160.0))
        .child(fx(FIRE, [22.0 + 150.0 * flare, 0.55 + 0.9 * flare, 0.0, 0.25 + 0.6 * flare]).key("flames")
            .absolute(0.0, HORIZON).size(W, 30.0).effect_bleed(240.0))
        .child(fx(FIRE, [HORIZON, 0.8, 3.0, 0.0]).absolute(0.0, HORIZON - 4.0).size(W, 8.0).effect_bleed(420.0))
}

// ------------------------------------------------------------------ gradient crawl

const CRAWL_DUR: f64 = 1.0;
const BAND: f32 = H * 2.0;

const STEPS: [(&str, &str, &str); 3] = [
    ("01 · WELCOME", "Make something\nyou can feel.", "Three quick questions and we'll set up your workspace."),
    ("02 · YOUR VIBE", "Pick a palette\nthat hums.", "Colour is the first thing people notice and the last they forget."),
    ("03 · DONE", "You're in.\nGo build.", "Your workspace is ready. The gradient was the loading screen."),
];

fn crawl_progress(s: &State) -> Option<f32> {
    s.crawl.map(|(t0, _)| ((s.t - t0) / CRAWL_DUR) as f32)
}

fn gradient_crawl(s: &State, rm: bool) -> El {
    let (kicker, title, body) = STEPS[s.step];
    let p = crawl_progress(s);
    // The outgoing screen lifts and fades as the band arrives; the incoming one settles in after.
    let leave = p.map(|p| ease_out(p / 0.45)).unwrap_or(0.0);
    let enter = ease_out(((s.t - s.swapped_at) / 0.3) as f32);
    let (dy, alpha) = if p.is_some_and(|p| p < 0.5) { (-40.0 * leave, 1.0 - leave) } else { (36.0 * (1.0 - enter), enter) };
    // Reduce Motion: the screens crossfade in place, with no band.
    let dy = if rm { 0.0 } else { dy };
    let last = s.step + 1 == STEPS.len();

    let mut dots = row().gap(8.0);
    for i in 0..STEPS.len() {
        dots = dots.child(el().h(6.0).w(if i == s.step { 26.0 } else { 6.0 }).radius(3.0)
            .fill(if i == s.step { c(0xededef) } else { ca(0xffffff, 0.25) }).motion(rune::tok::motion::SNAPPY));
    }
    let next = row().key("next").h(48.0).px(24.0).radius(24.0).center().fill(c(0xededef))
        .child(t(if last { "Start over" } else { "Continue" }, 15.0, 600.0, 0x0b0b0d))
        .on_press(|s: &mut State| {
            if s.crawl.is_none() {
                s.crawl = Some((s.t, (s.step + 1) % STEPS.len()));
            }
        });
    let screen = col().absolute(96.0, 120.0 + dy).w(W - 192.0).gap(18.0).opacity(alpha)
        .child(t(kicker, 12.5, 600.0, 0x9b8cff))
        .child(t(title, 56.0, 650.0, 0xededef))
        .child(t(body, 17.0, 450.0, 0x9a9aa3).w(520.0))
        .child(el().h(18.0))
        .child(row().gap(24.0).center_y().child(next).child(dots));

    let mut root = el().w(W).h(H).fill(c(0x0b0b0d)).clip_rounded(0.0).child(screen);
    if let Some(p) = p.filter(|_| !rm) {
        // Two bands crawl up the screen, the second a beat behind with a different palette.
        for (lag, palette, seed) in [(0.0, 4.0, 1.0), (0.09, 1.0, 2.0)] {
            let q = ease_in_out((p - lag) / (1.0 - lag));
            let y = H - q * (H + BAND);
            root = root.child(fx(MESH, [palette, 1.4, seed, 0.6]).absolute(-40.0, y).size(W + 80.0, BAND).radius(420.0));
        }
    }
    root
}

// ------------------------------------------------------------------ drip buttons

const BTN_W: f32 = 240.0;
const BTN_H: f32 = 60.0;
const BTN_X: f32 = (W - BTN_W) / 2.0;
const BTN_Y: f32 = 250.0;
const GRAVITY: f32 = 1500.0;
/// Drips hint at an untouched button: they start once it has sat a moment, each spot drips
/// three times, then they rest and start over. A press resets the wait.
const HINT_DELAY: f64 = 1.5;
const HINT_CYCLE: f64 = 16.0;
// A polished bead under a studio light: hot highlight, sky, a dark horizon band, a bright rim.
const CHROME_BEAD: [(f32, u32); 5] = [(0.0, 0xffffff), (0.22, 0xe4e9f0), (0.5, 0x7d8592), (0.78, 0x23262c), (1.0, 0xaab3c0)];
const CHROME_TUBE: [(f32, u32); 4] = [(0.0, 0x3a3e46), (0.4, 0xf2f5f9), (0.7, 0x8a919c), (1.0, 0x2a2d33)];

/// One drip's pose at time `t`: a bead that swells under the button's lower edge, then lets go and
/// falls. Returns the bead (centre x, y, radius, vertical stretch) and the neck still holding it.
struct Drip {
    x: f32,
    y: f32,
    r: f32,
    stretch: f32,
    neck: f32,
}

/// Time into the current burst for a drip spot that starts `delay` into it, while it's dripping.
fn burst(s: &State, delay: f64, period: f64) -> Option<f64> {
    let quiet = s.t - s.pressed_at.max(s.opened_at) - HINT_DELAY - delay;
    let u = quiet.rem_euclid(HINT_CYCLE);
    (quiet >= 0.0 && u < 3.0 * period).then_some(u)
}

fn drip(t: f64, x: f32, period: f64, size: f32) -> Option<Drip> {
    let u = ((t / period).fract()) as f32;
    let edge = BTN_Y + BTN_H - 6.0;
    const FORM: f32 = 0.6;
    if u < FORM {
        let k = smooth(u / FORM);
        let r = size * k;
        return Some(Drip { x, y: edge + r * 1.1, r, stretch: 1.0 + 0.35 * k, neck: r * 1.1 });
    }
    let ft = (u - FORM) * period as f32;
    let y = edge + size * 1.1 + 0.5 * GRAVITY * ft * ft;
    (y < H + 40.0).then(|| Drip { x, y, r: size, stretch: (1.35 + ft * 1.6).min(1.9), neck: 0.0 })
}

fn press_cta(s: &mut State) {
    s.pressed_at = s.t;
}

fn cta(label: &str, ink: u32) -> El {
    row().key("cta").absolute(BTN_X, BTN_Y).size(BTN_W, BTN_H).radius(BTN_H / 2.0).center().gap(10.0)
        .press_scale(0.97)
        .child(t(label, 18.0, 700.0, ink))
        .child(t("→", 19.0, 700.0, ink))
        .on_press(press_cta)
}

fn metal_drip(s: &State, rm: bool) -> El {
    let mut root = el().w(W).h(H).fill(c(0x0a0a0c))
        .child(row().absolute(0.0, 150.0).w(W).center().child(t("STEP 2 OF 3", 12.0, 600.0, 0x5f5f68)))
        .child(row().absolute(0.0, 176.0).w(W).center().child(t("Your build is ready", 30.0, 650.0, 0xededef)));
    // Drips first, so a forming bead tucks under the button's edge. They're plain elements with
    // a studio-chrome gradient: rune-fx's liquid-metal fill on several small moving shapes hung
    // the software GPU (lavapipe) this was verified on, and at this size the gradient reads the same.
    for (i, (dx, period, delay, size)) in [(-70.0, 2.9, 0.0, 9.0), (18.0, 3.4, 0.9, 7.0), (82.0, 2.6, 1.7, 5.5)].into_iter().enumerate() {
        let Some(u) = burst(s, delay, period).filter(|_| !rm) else { continue };
        let Some(d) = drip(u, W / 2.0 + dx, period, size) else { continue };
        if d.r < 0.5 {
            continue;
        }
        if d.neck > 0.0 {
            let (w, top) = (d.r * 0.9, BTN_Y + BTN_H - 10.0);
            root = root.child(el().key(format!("neck-{i}")).absolute(d.x - w / 2.0, top).size(w, (d.y - top).max(w)).radius(w / 2.0)
                .linear_gradient(std::f32::consts::FRAC_PI_2, CHROME_TUBE.map(|(k, hex)| (k, c(hex)))));
        }
        let (w, h) = (2.0 * d.r, 2.0 * d.r * d.stretch);
        root = root.child(el().key(format!("drop-{i}")).absolute(d.x - w / 2.0, d.y - h / 2.0).size(w, h).radius(d.r)
            .radial_gradient([0.38, 0.3], [d.r * 1.25, d.r * 1.25 * d.stretch], CHROME_BEAD.map(|(k, hex)| (k, c(hex)))));
    }
    root.child(fx(GLOW, [0.0, 0xcfd8e6 as f32, 0.35, 26.0]).key("glow").absolute(BTN_X, BTN_Y).size(BTN_W, BTN_H).radius(BTN_H / 2.0).effect_bleed(60.0))
        .child(fx(CHROME, [0.0, 0.45, 0.0, 0.0]).key("metal").absolute(BTN_X, BTN_Y).size(BTN_W, BTN_H).radius(BTN_H / 2.0))
        .child(cta("Deploy", 0x0b0c0f))
        .child(fx(SPARKLES, [s.pressed_at as f32, 26.0, 1.0, 0xffffff as f32]).key("sparkles").absolute(BTN_X, BTN_Y).size(BTN_W, BTN_H).radius(BTN_H / 2.0).effect_bleed(80.0))
}

fn water_drip(s: &State, rm: bool) -> El {
    const PERIOD: f64 = 2.6;
    // A new spot along the button's lower edge each cycle.
    let d = burst(s, 0.0, PERIOD).filter(|_| !rm).and_then(|u| {
        let dx = [-52.0, 30.0, -8.0][(u / PERIOD) as usize % 3];
        drip(u, W / 2.0 + dx, PERIOD, 11.0)
    });
    let second = d.map(|d| [d.x, d.y, d.r]).unwrap_or([0.0, 0.0, 0.0]);

    let mut copy = col().absolute(96.0, 70.0).gap(12.0)
        .child(t("STEP 2 OF 3", 12.0, 600.0, 0xffffff).opacity(0.7))
        .child(t("Connect your repo", 44.0, 650.0, 0xffffff));
    for line in ["We'll watch it for pushes and preview every branch.", "Nothing runs until you say so."] {
        copy = copy.child(t(line, 17.0, 450.0, 0xffffff).opacity(0.8));
    }
    // Stripes behind the button and the drop path, so the glass has something to bend.
    let mut stripes = row().absolute(0.0, BTN_Y - 20.0).w(W).h(H - BTN_Y + 20.0).gap(14.0).justify_center();
    for i in 0..40 {
        stripes = stripes.child(el().w(10.0).h_full().fill(ca(0xffffff, if i % 2 == 0 { 0.10 } else { 0.04 })));
    }
    el().w(W).h(H).fill(c(0x062a24))
        .child(fx(MESH, [3.0, 0.5, 4.0, 0.3]).absolute(0.0, 0.0).size(W, H))
        .child(stripes)
        .child(copy)
        // One lens: the button's glass, gooey-unioned with the drop it lets go of. The label sits
        // on top so the glass bends what's behind the button, not its own text.
        .child(fx(LENS, [second[0], second[1], second[2], 0.15]).key("glass").effect_backdrop(true)
            .absolute(BTN_X, BTN_Y).size(BTN_W, BTN_H).radius(BTN_H / 2.0))
        .child(cta("Connect GitHub", 0xffffff))
}

// ------------------------------------------------------------------ hold to confirm

const HOLD: f64 = 1.4;
const HOLD_W: f32 = 260.0;
const HOLD_H: f32 = 52.0;

fn charge(s: &State) -> f32 {
    s.held_from.map_or(0.0, |t0| ((s.t - t0) / HOLD) as f32).min(1.0)
}

/// The charge on screen: it builds linearly while held and drains in 200 ms once let go.
fn shown_charge(s: &State) -> f32 {
    if s.held_from.is_some() { charge(s) } else { s.drain.1 * (1.0 - ease_out(((s.t - s.drain.0) / 0.2) as f32)) }
}

fn hold_confirm(s: &State, rm: bool) -> El {
    let k = shown_charge(s);
    let since = (s.t - s.fired_at) as f32;
    let gone = since < 3.0;
    // The button trembles harder as the charge builds.
    let shake = if k > 0.3 && !rm { (s.t as f32 * 70.0).sin() * 2.2 * (k - 0.3) } else { 0.0 };
    let (bx, by) = ((W - HOLD_W) / 2.0, 340.0);
    let button = el().key("hold").absolute(bx + shake, by).size(HOLD_W, HOLD_H).radius(HOLD_H / 2.0)
        .fill(c(0x2a0f15)).border(1.0, ca(0xff4d6d, 0.35)).clip_rounded(HOLD_H / 2.0)
        .on_drag(|s: &mut State, d: &mut Drag, _: &mut Tx| match d.phase {
            Phase::Down => s.held_from = Some(s.t),
            Phase::Move => {}
            Phase::Up => {
                s.drain = (s.t, charge(s));
                s.held_from = None;
            }
        })
        .child(el().absolute(0.0, 0.0).size(HOLD_W * k, HOLD_H).radius(HOLD_H / 2.0).fill(c(0xff4d6d)))
        .child(row().absolute(0.0, 0.0).size(HOLD_W, HOLD_H).center()
            .child(t(if k > 0.0 { "Keep holding…" } else { "Hold to delete" }, 15.0, 650.0, 0xffe4e9)));
    let card = col().absolute(250.0, 140.0).w(460.0).gap(10.0)
        .opacity(if gone { 1.0 - ease_out(since / 0.3) } else { ease_out((since - 3.0) / 0.4).max(if s.fired_at < 0.0 { 1.0 } else { 0.0 }) })
        .child(t("DANGER ZONE", 12.0, 650.0, 0xff4d6d))
        .child(t("Delete “rune-experiments”?", 28.0, 650.0, 0xededef))
        .child(t("This removes 6 experiments, 4 recordings and every flame we simulated.\nHold the button until it lets go.", 15.0, 450.0, 0x9a9aa3));
    let mut root = el().w(W).h(H).fill(c(0x0b0b0d));
    // A grid of tiles behind everything, so the shockwave has something to bend.
    for i in 0..48 {
        let (x, y) = ((i % 8) as f32 * 120.0 + 8.0, (i / 8) as f32 * 100.0 + 8.0);
        root = root.child(el().absolute(x, y).size(104.0, 84.0).radius(10.0).fill(ca(0xffffff, 0.025)).border(1.0, ca(0xffffff, 0.04)));
    }
    root = root.child(card);
    if gone {
        root = root.child(row().absolute(0.0, 250.0).w(W).center().opacity(ease_out((since - 0.3) / 0.4) * (1.0 - ease_out((since - 2.6) / 0.4)))
            .child(t("Deleted. (Not really.)", 26.0, 650.0, 0xededef)));
    } else {
        root = root
            .child(fx(GLOW, [0.0, 0xff4d6d as f32, 0.2 + 1.2 * k, 18.0 + 30.0 * k]).absolute(bx + shake, by).size(HOLD_W, HOLD_H).radius(HOLD_H / 2.0).effect_bleed(90.0))
            .child(button)
            .child(fx(ELECTRIC, [0.15 + 1.6 * k, 0.8 + 7.0 * k, 0.0, 0.0]).absolute(bx + shake, by).size(HOLD_W, HOLD_H).radius(HOLD_H / 2.0).effect_bleed(60.0));
    }
    root.child(fx(SHOCK, [W / 2.0, by + HOLD_H / 2.0, s.fired_at as f32, 26.0]).key("shock").effect_backdrop(true).absolute(0.0, 0.0).size(W, H))
}

// ------------------------------------------------------------------ goo tabs

const TABS: [&str; 4] = ["Today", "Search", "Saved", "Profile"];
const TAB_W: f32 = 112.0;
const TAB_H: f32 = 44.0;
const BAR_X: f32 = (W - TAB_W * 4.0 - 12.0) / 2.0;
const BAR_Y: f32 = 430.0;
const GOO_DUR: f64 = 0.3;

fn tab_x(i: usize) -> f32 {
    BAR_X + 6.0 + i as f32 * TAB_W
}

/// The pill's leading and trailing edges now. The glass stretches like a liquid: the leading
/// edge runs ahead and the trailing edge catches up, each from where it was when the tab changed.
fn goo_pose(s: &State) -> (f32, f32) {
    let p = ((s.t - s.tab_at) / GOO_DUR) as f32;
    let ([lead, trail], to) = (s.goo_from, tab_x(s.tab));
    (lead + (to - lead) * ease_in_out(p / 0.7), trail + (to - trail) * ease_in_out((p - 0.3) / 0.7))
}

fn goo_tabs(s: &State, rm: bool) -> El {
    let (lead, trail) = if rm { (tab_x(s.tab), tab_x(s.tab)) } else { goo_pose(s) };
    let (gx, gw) = (lead.min(trail), (lead - trail).abs() + TAB_W);
    let mut root = el().w(W).h(H).fill(c(0x0d0a14));
    // Photo-like cards for the glass to bend. Native gradients: animated rune-fx fills here,
    // under a moving lens, hung the software GPU (lavapipe) this was verified on.
    const CARDS: [[u32; 3]; 5] = [[0x3d6bff, 0x9b5cff, 0x22d3ee], [0xff4d4d, 0xffb547, 0x7a1640], [0x34d399, 0xd9f99d, 0x0f766e], [0xff5ca8, 0x7c5cff, 0xffd1e8], [0xffb547, 0xff5ca8, 0x3d6bff]];
    for i in 0..12 {
        let (x, y) = ((i % 4) as f32 * 228.0 + 24.0, (i / 4) as f32 * 160.0 + 56.0);
        let [a, b, d] = CARDS[i % 5];
        root = root.child(el().absolute(x, y).size(216.0, 148.0).radius(16.0)
            .linear_gradient(0.6 + i as f32 * 0.7, [(0.0, c(a)), (0.55, c(b)), (1.0, c(d))]));
    }
    let mut bar = el().absolute(BAR_X, BAR_Y - 6.0).size(TAB_W * 4.0 + 12.0, TAB_H + 12.0).radius((TAB_H + 12.0) / 2.0)
        .fill(ca(0x15121c, 0.35)).border(1.0, ca(0xffffff, 0.18));
    for (i, name) in TABS.iter().enumerate() {
        bar = bar.child(row().key(format!("tab-{i}")).absolute(6.0 + i as f32 * TAB_W, 6.0).size(TAB_W, TAB_H).center()
            .child(t(*name, 14.0, 600.0, if i == s.tab { 0xffffff } else { 0x8d8a96 }))
            .on_press(move |s: &mut State| {
                if s.tab != i {
                    let (lead, trail) = goo_pose(s);
                    s.goo_from = [lead, trail];
                    s.tab = i;
                    s.tab_at = s.t;
                }
            }));
    }
    root.child(bar)
        .child(fx(GOO, [0.0; 4]).key("goo").effect_backdrop(true).absolute(gx, BAR_Y).size(gw, TAB_H).radius(TAB_H / 2.0))
}

// ------------------------------------------------------------------ app

fn stage(exp: Exp, s: &State, rm: bool) -> El {
    match exp {
        Exp::LavaTodo => lava_todo(s, rm),
        Exp::GradientCrawl => gradient_crawl(s, rm),
        Exp::MetalDrip => metal_drip(s, rm),
        Exp::WaterDrip => water_drip(s, rm),
        Exp::HoldConfirm => hold_confirm(s, rm),
        Exp::GooTabs => goo_tabs(s, rm),
    }
}

const TILE_K: f32 = 0.3;
const TILE_W: f32 = W * TILE_K;
const TILE_H: f32 = H * TILE_K;
const TILE_GAP: f32 = 24.0;

fn grid(s: &State, rm: bool) -> El {
    let x0 = (W - 3.0 * TILE_W - 2.0 * TILE_GAP) / 2.0;
    let mut root = el().w(W).h(H).fill(c(0x0b0b0d))
        .child(col().absolute(x0, 26.0).gap(4.0)
            .child(t("Rune experiments", 24.0, 650.0, 0xededef))
            .child(t("Live previews. Click one to open it.", 13.0, 450.0, 0x7b7b84)));
    for (i, (name, exp, title, blurb)) in EXPERIMENTS.into_iter().enumerate() {
        let x = x0 + (i % 3) as f32 * (TILE_W + TILE_GAP);
        let y = 96.0 + (i / 3) as f32 * (TILE_H + 70.0);
        // The experiment at full size, scaled about its centre into the tile.
        let preview = stage(exp, s, rm).absolute((TILE_W - W) / 2.0, (TILE_H - H) / 2.0).scale(TILE_K);
        root = root
            .child(el().key(format!("tile-{name}")).absolute(x, y).size(TILE_W, TILE_H).radius(12.0).clip_rounded(12.0)
                .border(1.0, ca(0xffffff, 0.1)).child(preview))
            // A press layer over the live preview, so a tap opens it instead of reaching inside.
            .child(el().key(name).role("button").label(title).focusable(true).absolute(x, y).size(TILE_W, TILE_H).radius(12.0)
                .fill(ca(0xffffff, 0.0)).hover(ca(0xffffff, 0.06)).cursor(rune::Cursor::Pointer)
                .on_press(move |s: &mut State| (s.exp, s.opened_at) = (Some(exp), s.t)))
            .child(col().absolute(x + 2.0, y + TILE_H + 12.0).gap(3.0)
                .child(t(title, 14.0, 600.0, 0xededef))
                .child(t(blurb, 12.0, 450.0, 0x7b7b84)));
    }
    root
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    // Rune's own springs follow Reduce Motion; the motion timed by hand here checks it itself.
    let rm = cx.system_reduced_motion();
    let Some(exp) = s.exp else { return grid(s, rm) };
    let back = row().key("back").absolute(14.0, 12.0).h(28.0).px(12.0).gap(6.0).radius(14.0).center()
        .fill(ca(0x000000, 0.45)).hover(ca(0x000000, 0.65)).border(1.0, ca(0xffffff, 0.12)).cursor(rune::Cursor::Pointer)
        .child(t("← All experiments", 12.5, 600.0, 0xededef))
        .on_press(|s: &mut State| s.exp = None);
    el().w(W).h(H).child(stage(exp, s, rm)).child(back)
}

fn on_key(s: &mut State, k: &KeyEvent, _: &mut Tx) -> bool {
    let back = k.key == Key::Escape && s.exp.is_some();
    if back {
        s.exp = None;
    }
    back
}

fn poll(s: &mut State, now: f64, tx: &mut rune::Tx) -> bool {
    // Motion timed by hand lands where it's computed; springs a handler started keep running.
    tx.instant();
    // And it needs a frame every frame: nothing else wakes the loop for it, and a handler
    // stamping `s.t` on an idle window would stamp a stale time.
    // ponytail: always 60 fps; wake only while something is in flight if idle CPU matters.
    tx.wake_after(std::time::Duration::from_millis(16));
    s.t = now;
    if s.held_from.is_some() && charge(s) >= 1.0 {
        s.held_from = None;
        s.fired_at = now;
    }
    if let (Some(p), Some((_, target))) = (crawl_progress(s), s.crawl) {
        if p >= 0.5 && s.step != target {
            s.step = target;
            s.swapped_at = now;
        }
        if p >= 1.0 {
            s.crawl = None;
        }
    }
    true
}

/// The app, with the grid and every experiment as headless scenes (`--scene lava-todo`).
pub fn app() -> rune::App<State> {
    rune::App::new(AppSpec {
        scenes: &[
            ("grid", |s: &mut State| s.exp = None),
            ("lava-todo", |s| s.exp = Some(Exp::LavaTodo)),
            ("gradient-crawl", |s| s.exp = Some(Exp::GradientCrawl)),
            ("metal-drip", |s| s.exp = Some(Exp::MetalDrip)),
            ("water-drip", |s| s.exp = Some(Exp::WaterDrip)),
            ("hold-confirm", |s| s.exp = Some(Exp::HoldConfirm)),
            ("goo-tabs", |s| s.exp = Some(Exp::GooTabs)),
        ],
        poll: Some(poll),
        on_key: Some(on_key),
        ..AppSpec::new("Rune experiments", (W, H), State::default, view)
    })
    .compositor(gfx::Gfx::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rune::Env;

    #[test]
    fn a_tab_picked_mid_slide_sets_off_from_where_the_pill_is() {
        let mut rt = app().headless_runtime((W, H), Env { system_dark: true, scale: 1.0 });
        rt.state.exp = Some(Exp::GooTabs);
        rt.frame(0.0);
        let tap = |rt: &mut rune::Runtime<State>, key: &str| {
            let (x, y) = rt.find(key).unwrap();
            rt.pointer_down(x, y);
            rt.pointer_up(x, y);
        };
        tap(&mut rt, "tab-3");
        for _ in 0..9 { rt.frame(1.0 / 60.0); }
        let mid = rt.presented("goo").unwrap().0;
        assert!(mid[2] > TAB_W + 60.0, "the pill is stretched mid-slide: {mid:?}");
        // Handlers run on the next frame; the pill should carry on from where it was drawn.
        tap(&mut rt, "tab-1");
        rt.frame(1.0 / 60.0);
        let next = rt.presented("goo").unwrap().0;
        assert!((next[0] - mid[0]).abs() < 45.0 && (next[2] - mid[2]).abs() < 45.0, "no jump: {mid:?} → {next:?}");
        for _ in 0..40 { rt.frame(1.0 / 60.0); }
        let end = rt.presented("goo").unwrap().0;
        assert!((end[0] - tab_x(1)).abs() < 1.0 && (end[2] - TAB_W).abs() < 1.0, "lands on tab 1: {end:?}");
    }
}
