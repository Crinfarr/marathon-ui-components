use super::prelude::*;

pub static ANIM_ARROW_DOWN: LazyLock<TileAnimation> = LazyLock::new(|| TileAnimation {
    frames: vec![
        &[
            &[O, O, O, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
        ],
        &[
            &[O, O, P, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
        ],
        &[
            &[O, P, P, P, O],
            &[O, O, P, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
        ],
        &[
            &[P, P, P, P, P],
            &[O, P, P, P, O],
            &[O, O, P, O, O],
            &[O, O, O, O, O],
            &[O, O, O, O, O],
        ],
        &[
            &[B, P, P, P, B],
            &[P, P, P, P, P],
            &[O, P, P, P, O],
            &[O, O, P, O, O],
            &[O, O, O, O, O],
        ],
        &[
            &[B, P, P, P, B],
            &[B, P, P, P, B],
            &[P, P, P, P, P],
            &[O, P, P, P, O],
            &[O, O, P, O, O],
        ],
        &[
            &[B, P, P, P, B],
            &[B, P, P, P, B],
            &[B, P, P, P, B],
            &[P, P, P, P, P],
            &[O, P, P, P, O],
        ],
        &[
            &[B, P, P, P, B],
            &[B, P, P, P, B],
            &[B, P, P, P, B],
            &[B, P, P, P, B],
            &[P, P, P, P, P],
        ],
    ],
});
