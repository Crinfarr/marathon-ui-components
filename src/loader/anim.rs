//prelude for animations
#![allow(unused_imports)]
pub use super::PxMarker::*;
pub use std::sync::LazyLock;

mod prelude {
    pub use super::*;
}

mod flower;
pub use flower::ANIM_FLOWER;
mod arrow_down;
pub use arrow_down::ANIM_ARROW_DOWN;

pub struct TileAnimation<'a> {
    pub frames: Vec<&'a [&'a [super::PxMarker]]>,
}
