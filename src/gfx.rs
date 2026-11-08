//! The compositor, adapted from rune's `examples/showreel/src/gfx.rs`: every `El::effect` slot
//! becomes one rune-fx layer. The slot's mark picks the effect, `effect_data` carries its few live
//! numbers, and its presented rect and radius are the shape. Lenses read a backdrop copy.

use rune::compose::{Compositor, Frame};
use rune::render::{EffectPhase, EffectSite, KIND_EFFECT};
use rune_fx::*;

use crate::State;

/// data: [height pt, intensity, style (0 top, 1 border, 2 heat, 3 embers), smoke]
pub const FIRE: u32 = 1;
/// data: [style (0 soft, 1 neon, 2 aura), colour hex, intensity, radius pt]
pub const GLOW: u32 = 2;
/// data: [palette, speed, seed, drift]
pub const MESH: u32 = 3;
/// data: [border width pt (0 = solid), flow multiplier, 0, 0]
pub const CHROME: u32 = 4;
/// data: [second circle cx, cy, r (0 = none), frost]; the second circle is gooey-unioned.
pub const LENS: u32 = 5;
/// data: [burst start s (< 0 = ambient), count, size, colour hex]
pub const SPARKLES: u32 = 6;
/// Cooled lava crust split by glowing seams. data: unused
pub const LAVA: u32 = 7;
/// Charged electric border. data: [intensity, amplitude, 0, 0]
pub const ELECTRIC: u32 = 8;
/// Shockwave through everything drawn before it. data: [x, y, start s, amount]
pub const SHOCK: u32 = 9;
/// Liquid-glass pill gooey-unioned with a copy of itself at `x`. data: [x, morph, 0, 0]
pub const GOO: u32 = 10;

fn mesh(palette: u32) -> MeshGradientParams {
    let stops: &[u32] = match palette {
        1 => &[0x0d1b4c, 0x3d6bff, 0x9b5cff, 0x22d3ee, 0x101030],
        2 => &[0x2a0a12, 0xff4d4d, 0xffb547, 0x7a1640, 0xff7a1a],
        3 => &[0x062a24, 0x34d399, 0xd9f99d, 0x0f766e],
        4 => &[0x1a1033, 0xff5ca8, 0x7c5cff, 0xffd1e8, 0x2b1b5a],
        _ => return MeshGradientParams::dusk(),
    };
    MeshGradientParams { colors: Stops::hex(stops), ..MeshGradientParams::dusk() }
}

fn layer(kind: u32, d: [f32; 4], shape: Shape, t: f32) -> Option<FxLayer> {
    let fx = match kind {
        FIRE => {
            let style = match d[2] as u32 {
                1 => FireStyle::Border,
                2 => FireStyle::Heat,
                3 => FireStyle::Embers,
                _ => FireStyle::Top,
            };
            Fx::fire(FireParams { style, height: d[0], intensity: d[1], smoke: d[3], embers: 28, scale: 24.0, ..FireParams::default() })
        }
        GLOW => {
            let style = match d[0] as u32 {
                1 => GlowStyle::Neon,
                2 => GlowStyle::Aura,
                _ => GlowStyle::Soft,
            };
            Fx::glow(GlowParams { style, color: rgb(d[1] as u32), intensity: d[2], radius: d[3], aura_speed: 0.5, ..Default::default() })
        }
        MESH => {
            let p = MeshGradientParams { speed: d[1].max(0.01), drift: d[3], ..mesh(d[0] as u32) };
            return Some(Fx::fill(p).on(shape).at(t).seed(d[2] as u32 + 1));
        }
        CHROME => {
            let mut p = LiquidMetalParams { scale: 220.0, ..LiquidMetalParams::chrome() };
            p.flow *= if d[1] > 0.0 { d[1] } else { 1.0 };
            if d[0] > 0.0 { Fx::fill(p.border(d[0])) } else { Fx::fill(p) }
        }
        LENS => Fx::lens(LensParams {
            second: (d[2] > 0.0).then(|| Shape::circle(d[0], d[1], d[2])),
            frost: d[3],
            bevel: 12.0,
            depth: 18.0,
            dispersion: 0.22,
            magnification: 1.08,
            union_k: 18.0,
            // The lens shadow follows the gooey union, so a hanging drop drags a dark column.
            shadow: 0.0,
            ..Default::default()
        }),
        SPARKLES => Fx::sparkles(SparkleParams { burst: (d[0] >= 0.0).then_some(d[0]), count: d[1] as u32, size: d[2], color: rgb(d[3] as u32), margin: 30.0, ..Default::default() }),
        ELECTRIC => Fx::electric(ElectricParams { intensity: d[0], amplitude: d[1], color: rgb(0xff4d6d), cool: rgb(0xff9a5a), ..ElectricParams::charged() }),
        SHOCK => Fx::distort(DistortParams::Shockwave { at: [d[0], d[1]], reach: 1100.0, width: 90.0, amount: d[3], chroma: 0.6, t0: d[2], duration: 1.1 }),
        GOO => Fx::lens(LensParams {
            second: (d[1] > 0.0).then(|| Shape::rounded(d[0], shape.rect.y, shape.rect.w, shape.rect.h, shape.radius)),
            morph: d[1].min(1.0),
            bevel: 10.0,
            depth: 14.0,
            dispersion: 0.3,
            magnification: 1.12,
            union_k: 28.0,
            // The lens shadow follows the gooey union, so a hanging drop drags a dark column.
            shadow: 0.0,
            ..Default::default()
        }),
        LAVA => Fx::fill(VoronoiParams {
            colors: Stops::hex(&[0x120806, 0x1c0d08, 0x26110a, 0x160a07]),
            edge_color: rgb(0xff7a22),
            scale: 40.0,
            speed: 0.1,
            edge: 1.1,
            depth: 0.9,
        }),
        _ => return None,
    };
    Some(fx.on(shape).at(t))
}

#[derive(Default)]
pub struct Gfx {
    fx: Option<FxRenderer>,
}

impl Compositor<State> for Gfx {
    fn animating(&self) -> bool {
        true
    }

    fn render(&mut self, f: &mut Frame<'_, State>) {
        let (device, queue, size) = (f.device, f.queue, f.size);
        let t = f.rt.t as f32;
        let marks = &f.rt.marks;
        let mut at: Vec<u32> = Vec::new();
        let mut layers: Vec<FxLayer> = Vec::new();
        let mut slots: Vec<FxSlot> = Vec::new();
        let mut fire = false;
        for (n, i) in f.rt.instances.iter().enumerate() {
            if i.params[1] != KIND_EFFECT {
                continue;
            }
            let kind = marks.get(n).copied().unwrap_or(0) & 0xff;
            let k = i.params[3];
            let [x, y, w, h] = i.rect.map(|v| v / k);
            // A degenerate shape (a spent drip, zero-sized) sends NaNs into the fill shaders.
            if w < 1.0 || h < 1.0 {
                continue;
            }
            let r = i.radii.iter().fold(0.0f32, |a, v| a.max(v.abs())) / k;
            let Some(l) = layer(kind, i.uv, Shape::rounded(x, y, w, h, r), t) else { continue };
            fire |= kind == FIRE;
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
        fx.prepare_with_slots(device, queue, &FxFrame { viewport: [size.0, size.1], scale: 1.0, layers: &layers, backdrop: None, sources: &[] }, &slots);
        if fire {
            fx.simulate(queue, f.encoder, None);
        }
        let clear = f.rt.clear_color();
        f.ui.prepare(device, queue, &mut f.rt.text.atlas, &f.rt.instances, size);
        f.ui.draw_with_backdrop(f.encoder, f.target, clear, &mut |phase, s: &EffectSite<'_>| {
            let Some(k) = at.iter().position(|n| *n == s.index) else { return };
            match phase {
                EffectPhase::Backdrop(enc) => {
                    if let Some(bd) = s.backdrop {
                        fx.blur_backdrop(device, queue, enc, bd, k);
                    }
                }
                EffectPhase::Draw(pass) => fx.draw_layer_in(pass, s.format, s.origin, k),
            }
        });
    }
}
