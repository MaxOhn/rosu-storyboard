pub(crate) use self::{animation::AnimationInternal, sprite::SpriteInternal};
pub use self::{
    animation::{Animation, AnimationLoopType},
    sample::Sample,
    sprite::Sprite,
    video::Video,
};

mod animation;
mod sample;
mod sprite;
mod video;

/// An element of a [`Storyboard`].
///
/// [`Storyboard`]: crate::Storyboard
#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub path: String,
    pub kind: ElementKind,
}

impl Element {
    pub fn new(path: String, kind: impl Into<ElementKind>) -> Self {
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

/// Additional data for an [`Element`].
#[derive(Clone, Debug, PartialEq)]
pub enum ElementKind {
    Animation(Animation),
    Sample(Sample),
    Sprite(Sprite),
    Video(Video),
}

macro_rules! from_elem_kind {
    ($from:ty, $variant:ident) => {
        impl From<$from> for ElementKind {
            fn from(from: $from) -> Self {
                Self::$variant(from)
            }
        }
    };
}

from_elem_kind!(Animation, Animation);
from_elem_kind!(Sample, Sample);
from_elem_kind!(Sprite, Sprite);
from_elem_kind!(Video, Video);

impl ElementKind {
    pub fn is_drawable(&self) -> bool {
        match self {
            ElementKind::Animation(ref elem) => elem.is_drawable(),
            ElementKind::Sprite(ref elem) => elem.is_drawable(),
            ElementKind::Sample(_) | ElementKind::Video(_) => true,
        }
    }

    pub fn start_time(&self) -> f64 {
        match self {
            ElementKind::Animation(elem) => elem.start_time(),
            ElementKind::Sample(elem) => elem.start_time,
            ElementKind::Sprite(elem) => elem.start_time(),
            ElementKind::Video(elem) => elem.start_time,
        }
    }

    pub fn end_time(&self) -> f64 {
        match self {
            ElementKind::Animation(elem) => elem.end_time(),
            ElementKind::Sample(elem) => elem.start_time,
            ElementKind::Sprite(elem) => elem.end_time(),
            ElementKind::Video(elem) => elem.start_time,
        }
    }
}

pub(crate) struct ElementInternal {
    pub path: String,
    pub kind: ElementKindInternal,
}

impl From<ElementInternal> for Element {
    fn from(elem: ElementInternal) -> Self {
        Self {
            path: elem.path,
            kind: match elem.kind {
                ElementKindInternal::Animation(elem) => ElementKind::Animation(elem.into()),
                ElementKindInternal::Sample(elem) => ElementKind::Sample(elem),
                ElementKindInternal::Sprite(elem) => ElementKind::Sprite(elem.into()),
                ElementKindInternal::Video(elem) => ElementKind::Video(elem),
            },
        }
    }
}

pub(crate) enum ElementKindInternal {
    Animation(AnimationInternal),
    Sample(Sample),
    Sprite(SpriteInternal),
    Video(Video),
}
