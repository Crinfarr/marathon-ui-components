mod anim;

pub mod animations {
    pub use super::anim::ANIM_ARROW_DOWN;
    pub use super::anim::ANIM_FLOWER;
}

use std::collections::HashMap;

use egui::{Color32, Context, CornerRadius, Rect, Ui, Vec2, util::hash};

#[derive(PartialEq)]
enum MaskState {
    Opaque,
    Covering(f64),
    Revealing(f64),
    Clear,
}
enum TileState {
    Clear,
    Animating(usize),
    Opaque,
}

#[allow(dead_code)]
pub enum AnimDirection {
    T2B,
    B2T,
    L2R,
    R2L,
    TL2BR,
    BL2TR,
    TR2BL,
    BR2TL,
    I2O,
    O2I,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum PxMarker {
    O,
    B,
    P,
    S,
    T,
}
use PxMarker::*;
#[cfg(feature = "log")]
use log::warn;

use crate::COLORS;
pub use anim::TileAnimation;

pub struct TileAnimRenderer<'a> {
    pub tile_size: f32,
    pub anim_duration_secs: f32,
    pub colorset: HashMap<PxMarker, Color32>,
    ///(Covering, Revealing)
    pub direction: (AnimDirection, AnimDirection),
    pub animation: &'a TileAnimation<'static>,
    state: MaskState,
}
impl<'a> Default for TileAnimRenderer<'a> {
    fn default() -> Self {
        Self {
            tile_size: 32.,
            anim_duration_secs: 3.,
            colorset: HashMap::from([
                (O, Color32::TRANSPARENT),
                (B, *COLORS.get("black").unwrap()),
                (P, *COLORS.get("primary").unwrap()),
                (S, *COLORS.get("accent").unwrap()),
                (T, *COLORS.get("warn").unwrap()),
            ]),
            animation: &animations::ANIM_FLOWER,
            direction: (AnimDirection::O2I, AnimDirection::I2O),
            state: MaskState::Clear,
        }
    }
}
impl<'a> TileAnimRenderer<'a> {
    pub fn reveal(&mut self, ctx: &Context) {
        self.state = MaskState::Revealing(ctx.input(|i| i.time));
        ctx.request_repaint();
    }
    pub fn cover(&mut self, ctx: &Context) {
        self.state = MaskState::Covering(ctx.input(|i| i.time));
        ctx.request_repaint();
    }
    pub fn is_opaque(&self) -> bool {
        self.state == MaskState::Opaque
    }
    #[allow(dead_code)]
    pub fn is_clear(&self) -> bool {
        self.state == MaskState::Clear
    }
    pub fn show_rect(&mut self, ui: &Ui, rect: &Rect) {
        let ctx = ui.ctx();
        let now = ctx.input(|i| i.time);

        let pct = match self.state {
            MaskState::Clear => return,
            MaskState::Opaque => 0.,
            MaskState::Covering(time) => {
                let pct = ((now - time) as f32 / self.anim_duration_secs).clamp(0., 1.);
                if pct >= 1. {
                    self.state = MaskState::Opaque;
                    0.
                } else {
                    pct
                }
            }
            MaskState::Revealing(time) => {
                let pct = ((now - time) as f32 / self.anim_duration_secs).clamp(0., 1.);
                if pct >= 1. {
                    self.state = MaskState::Clear;
                    0.
                } else {
                    pct
                }
            }
        };

        ctx.request_repaint();
        let painter_inst = ui.painter_at(*rect);
        let (dx, dy) = (
            (rect.width() / self.tile_size).ceil() as i32,
            (rect.height() / self.tile_size).ceil() as i32,
        );
        for ytile in 0..dy {
            for xtile in 0..dx {
                let weight =
                    (hash((ytile as u64) << 32 | (xtile as u64)) as f64 / u64::MAX as f64) as f32;
                // 80% of window is delay
                // 60% position timing, 40% hash weight
                let delay =
                    (0.6 * self.get_tile_delay(xtile, ytile, dx - 1, dy - 1) + 0.4 * weight) * 0.8;
                // 20% of window is anim
                let anim_pct = ((pct - delay) / 0.2).clamp(0., 1.);
                let frame_num = ((anim_pct * self.animation.frames.len() as f32) as usize)
                    .min(self.animation.frames.len() - 1);
                let tile_state = match self.state {
                    MaskState::Opaque => TileState::Opaque,
                    MaskState::Covering(_) => {
                        if anim_pct >= 1. {
                            TileState::Opaque
                        } else if anim_pct <= 0. {
                            TileState::Clear
                        } else {
                            TileState::Animating(frame_num)
                        }
                    }
                    MaskState::Revealing(_) => {
                        if anim_pct >= 1. {
                            TileState::Clear
                        } else if anim_pct <= 0. {
                            TileState::Opaque
                        } else {
                            TileState::Animating(self.animation.frames.len() - 1 - frame_num)
                        }
                    }
                    MaskState::Clear => TileState::Clear,
                };
                let origin = rect.min
                    + (xtile as f32 * self.tile_size, ytile as f32 * self.tile_size).into();
                let tile = Rect::from_min_size(origin, Vec2::splat(self.tile_size));
                match tile_state {
                    TileState::Clear => { /*render nothing*/ }
                    TileState::Animating(frame_num) => {
                        let frame = self.animation.frames[frame_num];
                        let px_size: Vec2 = (
                            tile.width() / frame[0].len() as f32,
                            tile.height() / frame.len() as f32,
                        )
                            .into();
                        for (y_idx, row) in frame.iter().enumerate() {
                            for (x_idx, px_type) in row.iter().enumerate() {
                                let offset: Vec2 =
                                    (x_idx as f32 * px_size.x, y_idx as f32 * px_size.y).into();
                                painter_inst.rect_filled(
                                    Rect::from_min_size(tile.min + offset, px_size),
                                    CornerRadius::ZERO,
                                    self.colorset
                                        .get(&px_type)
                                        .unwrap_or_else(|| {
                                            #[cfg(feature = "log")]
                                            warn!("No color specified for PxMarker::{px_type:?}");
                                            &Color32::MAGENTA
                                        })
                                        .to_owned(),
                                );
                            }
                        }
                    }
                    TileState::Opaque => {
                        let base_color = COLORS
                            .get("surface")
                            .unwrap()
                            .lerp_to_gamma(*COLORS.get("raised").unwrap(), weight);
                        let blink = if (((now % 1000.) * 0.7) + weight as f64 * 7.).fract() < 0.04 {
                            0.35
                        } else {
                            0.
                        };
                        let color = base_color.lerp_to_gamma(
                            self.colorset
                                .get(&P)
                                .unwrap_or_else(|| {
                                    #[cfg(feature = "log")]
                                    warn!("No color specified for PxMarker::P");
                                    &Color32::MAGENTA
                                })
                                .to_owned(),
                            blink,
                        );
                        painter_inst.rect_filled(tile, CornerRadius::ZERO, color);
                    }
                }
            }
        }
    }
    fn get_tile_delay(&self, x: i32, y: i32, xmax: i32, ymax: i32) -> f32 {
        let anim = match self.state {
            MaskState::Opaque => &self.direction.0,
            MaskState::Covering(_) => &self.direction.0,
            MaskState::Revealing(_) => &self.direction.1,
            MaskState::Clear => &self.direction.1,
        };
        match anim {
            AnimDirection::L2R => x as f32 / xmax as f32,
            AnimDirection::R2L => (xmax - x) as f32 / xmax as f32,
            AnimDirection::T2B => y as f32 / ymax as f32,
            AnimDirection::B2T => (ymax - y) as f32 / ymax as f32,
            AnimDirection::TL2BR => (x + y) as f32 / (xmax + ymax) as f32,
            AnimDirection::BL2TR => (x + (ymax - y)) as f32 / (xmax + ymax) as f32,
            AnimDirection::TR2BL => ((xmax - x) + y) as f32 / (xmax + ymax) as f32,
            AnimDirection::BR2TL => ((xmax - x) + (ymax - y)) as f32 / (xmax + ymax) as f32,
            AnimDirection::I2O => {
                ((x as f32 - (xmax as f32 / 2.)).abs() + (y as f32 - (ymax as f32 / 2.)).abs())
                    / ((xmax as f32 / 2.) + (ymax as f32 / 2.))
            }
            AnimDirection::O2I => {
                1. - ((x as f32 - (xmax as f32 / 2.)).abs() + (y as f32 - (ymax as f32 / 2.)).abs())
                    / ((xmax as f32 / 2.) + (ymax as f32 / 2.))
            }
        }
    }
}
