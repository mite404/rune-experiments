// @snip:ex5
#[cfg(test)]
mod tests {
    use super::*;
    use rune::{Env, Runtime};

    // ── DATA ────────────────────────────────────────────────
    const SIZE: (f32, f32) = (400.0, 380.0);
    const FRAME_SECONDS: f64 = 1.0 / 60.0;

    // ── ACTIONS (on a fake window, not a real one) ──────────
    fn app() -> Runtime<State> {
        // → Runtime<State>, first frame built
        // TODO(you) 1: build `Runtime::new(state, view, SIZE, Env { .. })` (Env is
        //   imported), then run one frame so there is a layout to query. Why does the first frame use dt = 0?
        todo!() // remove when implemented
    }

    fn drag(runtime: &mut Runtime<State>, key: &str, dx: f32) {
        // in: Runtime, key, distance
        // TODO(you) 2: find the element's centre by key, press there, move in ~10 steps (one
        //   frame each, so Drag sees Move phases), let go, then run about a second of frames
        //   so the springs settle. What breaks if you skip the frames between moves?
        let _ = (runtime, key, dx, FRAME_SECONDS);
        todo!() // remove when implemented
    }

    // ── TESTS ───────────────────────────────────────────────
    #[test]
    fn release_archives_at_the_threshold_in_either_direction() {
        // TODO(you) 3: three asserts: just under, exactly at, and the negative side.
        todo!()
    }

    #[test]
    fn a_short_drag_springs_home() {
        // TODO(you) 4: record the card's drawn x, drag 60 pt, then assert BOTH that no card
        //   was archived (state) and that it is drawn back where it began (screen).
        let _ = (app, drag, SIZE);
        todo!()
    }

    #[test]
    fn a_long_drag_archives_the_card() {
        // TODO(you) 5: drag left past the threshold; assert which ids remain, in order.
        todo!()
    }
}
// @end
