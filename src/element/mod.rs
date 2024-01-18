pub(crate) use self::{animation::StoryboardAnimationInternal, sprite::StoryboardSpriteInternal};
pub use self::{
    animation::{AnimationLoopType, StoryboardAnimation},
    sample::StoryboardSample,
    sprite::StoryboardSprite,
    video::StoryboardVideo,
};

mod animation;
mod sample;
mod sprite;
mod video;

/// An element of a [`Storyboard`].
///
/// [`Storyboard`]: crate::Storyboard
#[derive(Clone, Debug, PartialEq)]
pub struct StoryboardElement {
    pub path: String,
    pub kind: StoryboardElementKind,
}

impl StoryboardElement {
    pub fn new(path: String, kind: impl Into<StoryboardElementKind>) -> Self {
        Self {
            path,
            kind: kind.into(),
        }
    }

    pub fn is_drawable(&self) -> bool {
        self.kind.is_drawable()
    }

    pub fn start_time(&self) -> f64 {
        self.kind.start_time()
    }

    pub fn end_time(&self) -> f64 {
        self.kind.end_time()
    }
}

/// Additional data for a [`StoryboardElement`].
#[derive(Clone, Debug, PartialEq)]
pub enum StoryboardElementKind {
    Animation(StoryboardAnimation),
    Sample(StoryboardSample),
    Sprite(StoryboardSprite),
    Video(StoryboardVideo),
}

macro_rules! from_elem_kind {
    ($from:ty, $variant:ident) => {
        impl From<$from> for StoryboardElementKind {
            fn from(from: $from) -> Self {
                Self::$variant(from)
            }
        }
    };
}

from_elem_kind!(StoryboardAnimation, Animation);
from_elem_kind!(StoryboardSample, Sample);
from_elem_kind!(StoryboardSprite, Sprite);
from_elem_kind!(StoryboardVideo, Video);

impl StoryboardElementKind {
    pub fn is_drawable(&self) -> bool {
        match self {
            StoryboardElementKind::Animation(ref elem) => elem.is_drawable(),
            StoryboardElementKind::Sprite(ref elem) => elem.is_drawable(),
            StoryboardElementKind::Sample(_) | StoryboardElementKind::Video(_) => true,
        }
    }

    pub fn start_time(&self) -> f64 {
        match self {
            StoryboardElementKind::Animation(elem) => elem.start_time(),
            StoryboardElementKind::Sample(elem) => elem.start_time,
            StoryboardElementKind::Sprite(elem) => elem.start_time(),
            StoryboardElementKind::Video(elem) => elem.start_time,
        }
    }

    pub fn end_time(&self) -> f64 {
        match self {
            StoryboardElementKind::Animation(elem) => elem.end_time(),
            StoryboardElementKind::Sample(elem) => elem.start_time,
            StoryboardElementKind::Sprite(elem) => elem.end_time(),
            StoryboardElementKind::Video(elem) => elem.start_time,
        }
    }
}

pub(crate) struct StoryboardElementInternal {
    pub path: String,
    pub kind: StoryboardElementKindInternal,
}

impl From<StoryboardElementInternal> for StoryboardElement {
    fn from(elem: StoryboardElementInternal) -> Self {
        Self {
            path: elem.path,
            kind: match elem.kind {
                StoryboardElementKindInternal::Animation(elem) => {
                    StoryboardElementKind::Animation(elem.into())
                }
                StoryboardElementKindInternal::Sample(elem) => StoryboardElementKind::Sample(elem),
                StoryboardElementKindInternal::Sprite(elem) => {
                    StoryboardElementKind::Sprite(elem.into())
                }
                StoryboardElementKindInternal::Video(elem) => StoryboardElementKind::Video(elem),
            },
        }
    }
}

pub(crate) enum StoryboardElementKindInternal {
    Animation(StoryboardAnimationInternal),
    Sample(StoryboardSample),
    Sprite(StoryboardSpriteInternal),
    Video(StoryboardVideo),
}
