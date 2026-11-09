//! The projector: a compositor that turns `rune::effect(MARK)` slots into rune-fx layers.
//! Adapted from rune's `examples/showreel/src/gfx.rs`, cut down to the three effects the
//! tutorial uses. You read this file; you don't write it.

// ── 1. IMPORTS ──────────────────────────────────────────────
use rune::compose::{Compositor, Frame};
use rune::render::{EffectPhase, EffectSite, KIND_EFFECT};
use rune_fx::*;

// ── 2. DATA ─────────────────────────────────────────────────
// @snip:marks
/// Fire rising off the slot's top edge. data: [flame height pt, intensity, 0, 0]
pub const FIRE: u32 = 1;
/// Liquid glass, gooey-unioned with an optional circle. data: [circle x, y, r (0 = none), frost]
pub const LENS: u32 = 2;
/// A ring of distortion through everything drawn before it. data: [x, y, start time s, amount]
pub const SHOCK: u32 = 3;
// @end

// ── 3. CALCULATION ──────────────────────────────────────────
// CATEGORY: Calculation - one slot's mark, numbers and shape in, one rune-fx layer out.
// @snip:layer
fn layer(mark: u32, d: [f32; 4], shape: Shape, t: f32) -> Option<FxLayer> {
    let fx = match mark {
        FIRE => Fx::fire(FireParams {
            height: d[0],
            intensity: d[1],
            embers: 24,
            scale: 24.0,
            ..FireParams::default()
        }),
        LENS => Fx::lens(LensParams {
            second: (d[2] > 0.0).then(|| Shape::circle(d[0], d[1], d[2])), // → Option<Shape>
            frost: d[3],
            bevel: 12.0,
            depth: 18.0,
            dispersion: 0.22,
            magnification: 1.08,
            union_k: 24.0,
            shadow: 0.0, // the shadow follows the goo and drags a dark box behind it
            ..Default::default()
        }),
        SHOCK => Fx::distort(DistortParams::Shockwave {
            at: [d[0], d[1]],
            reach: 700.0,
            width: 70.0,
            amount: d[3],
            chroma: 0.6,
            t0: d[2],
            duration: 1.0,
        }),
        _ => return None,
    };
    Some(fx.on(shape).at(t)) // → FxLayer
}
// @end

// ── 4. ACTIONS ──────────────────────────────────────────────
/// The compositor. Attach it with `rune::run_with(spec, Gfx::default())`.
#[derive(Default)]
pub struct Gfx {
    fx: Option<FxRenderer>,
}

impl<S: 'static> Compositor<S> for Gfx {
    fn animating(&self) -> bool {
        // Fire and shockwaves move on their own, so keep drawing frames.
        // ponytail: always true, so lessons with nothing moving still redraw; track live slots if battery matters.
        true
    }

    fn render(&mut self, f: &mut Frame<'_, S>) {
        let (device, queue, size) = (f.device, f.queue, f.size);
        let t = f.rt.t as f32; // → seconds on Rune's clock
        let marks = &f.rt.marks;
        let mut at: Vec<u32> = Vec::new(); // instance index of each layer
        let mut layers: Vec<FxLayer> = Vec::new();
        let mut slots: Vec<FxSlot> = Vec::new();
        let mut fire = false;
        for (n, i) in f.rt.instances.iter().enumerate() {
            if i.params[1] != KIND_EFFECT {
                continue;
            }
            let mark = marks.get(n).copied().unwrap_or(0) & 0xff;
            let k = i.params[3]; // device pixels per point
            let [x, y, w, h] = i.rect.map(|v| v / k); // → window points
            // A slot under 1 pt sends NaNs into the shaders: skip it.
            if w < 1.0 || h < 1.0 {
                continue;
            }
            let r = i.radii.iter().fold(0.0f32, |a, v| a.max(v.abs())) / k;
            let Some(l) = layer(mark, i.uv, Shape::rounded(x, y, w, h, r), t) else {
                continue;
            };
            fire |= mark == FIRE;
            at.push(n as u32);
            layers.push(l);
            slots.push(FxSlot::default().scale(k).clip(i.clip, i.fade));
        }
        let fx = self.fx.get_or_insert_with(|| {
            let mut fx = FxRenderer::new(device, f.format, FxOptions { dither: true });
            fx.set_fire_sim(device, true);
            fx
        });
        fx.set_srgb_blend(f.srgb_blend());
        fx.add_format(device, wgpu::TextureFormat::Rgba16Float);
        let frame = FxFrame {
            viewport: [size.0, size.1],
            scale: 1.0,
            layers: &layers,
            backdrop: None,
            sources: &[],
        };
        fx.prepare_with_slots(device, queue, &frame, &slots);
        if fire {
            fx.simulate(queue, f.encoder, None);
        }
        let clear = f.rt.clear_color();
        f.ui.prepare(device, queue, &mut f.rt.text.atlas, &f.rt.instances, size);
        f.ui.draw_with_backdrop(
            f.encoder,
            f.target,
            clear,
            &mut |phase, s: &EffectSite<'_>| {
                let Some(k) = at.iter().position(|n| *n == s.index) else {
                    return;
                };
                match phase {
                    EffectPhase::Backdrop(enc) => {
                        if let Some(bd) = s.backdrop {
                            fx.blur_backdrop(device, queue, enc, bd, k);
                        }
                    }
                    EffectPhase::Draw(pass) => fx.draw_layer_in(pass, s.format, s.origin, k),
                }
            },
        );
    }
}
