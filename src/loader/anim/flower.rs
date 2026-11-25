use super::prelude::*;

pub static ANIM_FLOWER: LazyLock<TileAnimation> = LazyLock::new(|| TileAnimation {
    frames: vec![
        &[&[O, O, O], &[O, O, O], &[O, O, O]],
        &[&[O, O, O], &[O, P, O], &[O, O, O]],
        &[&[O, P, O], &[P, S, P], &[O, P, O]],
        &[&[P, S, P], &[S, T, S], &[P, S, P]],
        &[&[S, T, S], &[T, B, T], &[S, T, S]],
        &[&[T, B, T], &[B, B, B], &[T, B, T]],
        &[&[B, B, B], &[B, B, B], &[B, B, B]],
    ],
});
